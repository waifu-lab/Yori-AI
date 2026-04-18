import type { Application } from 'pixi.js';
import type { Live2DModel as Live2DModelType } from 'untitled-pixi-live2d-engine';

let Application_: typeof import('pixi.js').Application;
let Live2DModel_: typeof Live2DModelType;

let app: Application | null = null;
let currentModel: Live2DModelType | null = null;
let initialized = false;

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
	await app.init({
		resizeTo: window,
		preference: 'webgl',
		backgroundAlpha: 0,
		autoDensity: true,
		resolution: window.devicePixelRatio || 1
	});

	container.appendChild(app.canvas);
	return app;
}

/**
 * Load a Live2D model from a URL served by the local file server.
 * Sub-resources (moc3, textures, etc.) are resolved relatively by the engine.
 */
export async function loadModelFromUrl(url: string): Promise<void> {
	if (!app) throw new Error('PixiJS app not initialised');

	if (currentModel) {
		app.stage.removeChild(currentModel as any);
		currentModel.destroy();
		currentModel = null;
	}

	const model = await Live2DModel_.from(url);

	model.anchor.set(0.5);
	const scale = Math.min((app.screen.width / model.width) * 0.8, (app.screen.height / model.height) * 0.8);
	model.scale.set(scale);
	model.position.set(app.screen.width / 2, app.screen.height / 2);

	app.stage.addChild(model as any);
	currentModel = model;
}

export function setExpression(name: string): void {
	if (!currentModel) return;
	currentModel.expression(name);
}

export function destroyApp(): void {
	if (currentModel) {
		currentModel.destroy();
		currentModel = null;
	}
	if (app) {
		app.destroy(true, { children: true });
		app = null;
	}
}
