import type { Application } from 'pixi.js';
import type { Live2DModel as Live2DModelType } from 'untitled-pixi-live2d-engine';
import type { Viseme } from '$lib/backend/types';
import { setLipsyncMode } from '$lib/commands';

let Application_: typeof import('pixi.js').Application;
let Live2DModel_: typeof Live2DModelType;

let app: Application | null = null;
let currentModel: Live2DModelType | null = null;
let modelNaturalWidth = 0;
let modelNaturalHeight = 0;
let initialized = false;
let dprCleanup: (() => void) | null = null;

/** Whether the loaded model has a formal LipSync parameter group. */
let hasNativeLipSync = false;

/** Active audio element for TTS playback. */
let activeAudio: HTMLAudioElement | null = null;

/** Scheduled lipsync animation frame ID. */
let lipsyncRafId: number | null = null;

async function ensureLoaded() {
	if (initialized) return;
	const pixi = await import('pixi.js');
	Application_ = pixi.Application;

	// WebView2 (Chromium in Tauri) fails with createImageBitmap for cross-origin textures.
	// Force the legacy <img> loader instead.
	pixi.loadTextures.config.preferCreateImageBitmap = false;
	pixi.loadTextures.config.preferWorkers = false;

	const engine = await import('untitled-pixi-live2d-engine');
	Live2DModel_ = engine.Live2DModel;
	initialized = true;
}

export async function initApp(container: HTMLElement): Promise<Application> {
	await ensureLoaded();
	if (app) return app;

	app = new Application_();
	const { w: initW, h: initH } = logicalSize();
	await app.init({
		width: initW,
		height: initH,
		preference: 'webgl',
		backgroundAlpha: 0,
		autoDensity: true,
		resolution: window.devicePixelRatio || 1
	});

	window.addEventListener('resize', handleResize);

	watchDevicePixelRatio();

	container.appendChild(app.canvas);
	return app;
}

function handleResize() {
	if (!app) return;
	const { w, h } = logicalSize();
	app.renderer.resize(w, h);
	repositionModel();
}

function watchDevicePixelRatio() {
	const dpr = window.devicePixelRatio;
	const mql = window.matchMedia(`(resolution: ${dpr}dppx)`);

	function onChange() {
		const newDpr = window.devicePixelRatio || 1;

		if (app) {
			// eslint-disable-next-line @typescript-eslint/no-explicit-any
			(app.renderer as unknown as { resolution: number }).resolution = newDpr;
			const { w, h } = logicalSize();
			app.renderer.resize(w, h);
		}
		repositionModel();
		dprCleanup?.();
		dprCleanup = null;
		watchDevicePixelRatio();
	}
	mql.addEventListener('change', onChange);
	dprCleanup = () => mql.removeEventListener('change', onChange);
}

function logicalSize() {
	const dpr = window.devicePixelRatio || 1;
	return { w: window.innerWidth / dpr, h: window.innerHeight / dpr };
}

function repositionModel() {
	if (!app || !currentModel || modelNaturalWidth === 0 || modelNaturalHeight === 0) return;
	const { w, h } = logicalSize();
	const scale = Math.min((w / modelNaturalWidth) * 0.9, (h / modelNaturalHeight) * 0.9);
	currentModel.scale.set(scale);
	currentModel.position.set(w / 2, h / 2);
}

/**
 * Load a Live2D model from a URL served by the local file server.
 * Sub-resources (moc3, textures, etc.) are resolved relatively by the engine.
 */
export async function loadModelFromUrl(url: string): Promise<void> {
	if (!app) throw new Error('PixiJS app not initialised');

	if (currentModel) {
		app.stage.removeChild(currentModel as Parameters<typeof app.stage.removeChild>[0]);
		currentModel = null;
	}

	const model = await Live2DModel_.from(url, {
		autoHitTest: false,
		autoFocus: false
	} as Parameters<typeof Live2DModel_.from>[1]);

	model.anchor.set(0.5);
	modelNaturalWidth = model.width;
	modelNaturalHeight = model.height;
	const scale = Math.min((app.screen.width / modelNaturalWidth) * 0.9, (app.screen.height / modelNaturalHeight) * 0.9);
	model.scale.set(scale);
	model.position.set(app.screen.width / 2, app.screen.height / 2);

	app.stage.addChild(model as any);
	currentModel = model;

	// Detect whether the model has formal LipSync parameter ids.
	// CubismMotionManager.lipSyncIds is populated from the model3.json Groups[].Ids
	// when the group Name is 'LipSync'.
	const lipSyncIds: string[] =
		(model as Live2DModelType & { internalModel: { motionManager: { lipSyncIds: string[] } } }).internalModel
			?.motionManager?.lipSyncIds ?? [];
	hasNativeLipSync = lipSyncIds.length > 0;

	// Notify Rust backend: if no native LipSync, backend should compute viseme data.
	setLipsyncMode(!hasNativeLipSync).catch((e) => console.warn('[live2d] setLipsyncMode failed:', e));
}

/** Returns true when the loaded model has no native LipSync and requires backend visemes. */
export function needsBackendLipsync(): boolean {
	return !hasNativeLipSync;
}

export function setExpression(name: string): void {
	if (!currentModel) return;
	currentModel.expression(name);
}

/** Feed an external cursor position (normalised -1..1) to the model's eye-tracking. */
export function setFocus(x: number, y: number): void {
	if (!currentModel) return;
	(currentModel as Live2DModelType & { focus(x: number, y: number): void }).focus(x, y);
}

/**
 * Play a base64-encoded audio clip. If the model has native LipSync support the
 * engine's `speak()` method is used, which drives the mouth automatically.
 * Otherwise we rely on `scheduleLipsync()` called separately with viseme data.
 */
export function playAudio(audioB64: string, format: string): void {
	stopAudio();
	const dataUrl = `data:audio/${format};base64,${audioB64}`;
	if (hasNativeLipSync && currentModel) {
		(currentModel as any).speak(dataUrl);
	} else {
		activeAudio = new Audio(dataUrl);
		activeAudio.play().catch((e) => console.warn('[live2d] audio play failed:', e));
	}
}

function stopAudio(): void {
	if (activeAudio) {
		activeAudio.pause();
		activeAudio = null;
	}
}

/**
 * Schedule manual lipsync viseme animation.
 * `visemes` is an array of `{ time_ms, value }` where `value` is 0–1 mouth open.
 * Requires the model to expose a "ParamMouthOpenY" (or similar) core parameter.
 */
export function scheduleLipsync(visemes: Viseme[]): void {
	if (!currentModel || visemes.length === 0) return;
	cancelLipsync();

	const coreModel = (currentModel as any).internalModel?.coreModel;
	if (!coreModel) return;

	// Use the LipSync parameter id from motionManager if available,
	// otherwise fall back to the standard Cubism 4 / Cubism 2 name.
	const lipSyncIds: string[] = (currentModel as any).internalModel?.motionManager?.lipSyncIds ?? [];
	const paramId: string = lipSyncIds[0] ?? 'ParamMouthOpenY';

	const startTime = performance.now();
	let index = 0;

	function frame() {
		if (!currentModel) return;
		const elapsed = performance.now() - startTime;

		// Advance index to the current viseme.
		while (index + 1 < visemes.length && visemes[index + 1].time_ms <= elapsed) {
			index++;
		}

		const value = visemes[index].value;
		try {
			coreModel.setParameterValueById(paramId, value);
		} catch {
			// Parameter not found — silently skip.
		}

		if (index < visemes.length - 1) {
			lipsyncRafId = requestAnimationFrame(frame);
		} else {
			lipsyncRafId = null;
		}
	}

	lipsyncRafId = requestAnimationFrame(frame);
}

function cancelLipsync(): void {
	if (lipsyncRafId !== null) {
		cancelAnimationFrame(lipsyncRafId);
		lipsyncRafId = null;
	}
}

export function destroyApp(): void {
	window.removeEventListener('resize', handleResize);
	dprCleanup?.();
	dprCleanup = null;
	cancelLipsync();
	stopAudio();
	currentModel = null;
	if (app) {
		app.destroy(true, { children: true });
		app = null;
	}
}
