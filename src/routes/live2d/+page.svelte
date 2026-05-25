<script lang="ts">
	import { onDestroy, onMount } from 'svelte';
	import { listen, type UnlistenFn } from '@tauri-apps/api/event';
	import { getCurrentWindow, PhysicalPosition } from '@tauri-apps/api/window';
	import { onEvent, sendLive2dSync, sendUserEvent } from '$lib/backend/ws';
	import {
		getCurrentModelUrl,
		getLive2dTransform,
		getModelPath,
		openFloatChatWindow,
		openSettingsWindow,
		setModelPath
	} from '$lib/commands';
	import { vadStore } from '$lib/stores/vad.svelte';
	import {
		attachLipsyncToLast,
		destroyApp,
		getExpressionNames,
		getModelBounds,
		hitTestAt,
		initApp,
		loadModelFromUrl,
		needsBackendLipsync,
		playAudio,
		setExpression,
		setFocus,
		setModelTransform
	} from '$lib/live2d/renderer';

	const PET_WINDOW_MS = 1000;
	const PET_MIN_REVERSALS = 3;
	const PET_COOLDOWN_MS = 3000;
	const PET_MIN_DELTA_PX = 6;

	let petReversals: number[] = [];
	let petLastDir = 0;
	let petLastX = 0;
	let petCooldownUntil = 0;

	function onPetDetected(areas: string[]) {
		sendUserEvent('pet', { areas }).catch(() => {});
	}

	function handlePetMove(e: MouseEvent) {
		const now = performance.now();
		const pixiX = e.clientX;
		const pixiY = e.clientY;

		const bounds = getModelBounds();
		const inBounds =
			!!bounds &&
			pixiX >= bounds.cx - bounds.hw &&
			pixiX <= bounds.cx + bounds.hw &&
			pixiY >= bounds.cy - bounds.hh &&
			pixiY <= bounds.cy + bounds.hh;

		if (!inBounds) return;

		const dx = pixiX - petLastX;
		petLastX = pixiX;
		if (Math.abs(dx) < PET_MIN_DELTA_PX) return;

		const dir = dx > 0 ? 1 : -1;
		if (petLastDir !== 0 && dir !== petLastDir) {
			petReversals.push(now);
		}
		petLastDir = dir;

		petReversals = petReversals.filter((t) => now - t < PET_WINDOW_MS);

		if (petReversals.length >= PET_MIN_REVERSALS && now > petCooldownUntil) {
			petCooldownUntil = now + PET_COOLDOWN_MS;
			petReversals = [];
			petLastDir = 0;
			onPetDetected(hitTestAt(pixiX, pixiY));
		}
	}

	let container: HTMLDivElement;
	let unlisten: (() => void) | null = null;
	let unlistenCursor: UnlistenFn | null = null;
	let unlistenEvent: UnlistenFn | null = null;
	let unlistenIdle: UnlistenFn | null = null;
	let unlistenTransform: UnlistenFn | null = null;

	function openSettings() {
		openSettingsWindow().catch(console.error);
	}

	function openChat() {
		openFloatChatWindow().catch(console.error);
	}

	function hideWindow() {
		getCurrentWindow().hide();
	}

	async function onDragMouseDown(e: MouseEvent) {
		if (e.button !== 0) return;
		const win = getCurrentWindow();
		const pos = await win.outerPosition();
		const startWinX = pos.x;
		const startWinY = pos.y;
		const dpr = window.devicePixelRatio || 1;
		const startMouseX = e.screenX * dpr;
		const startMouseY = e.screenY * dpr;

		let rafId: number | null = null;
		let pendingX = startWinX;
		let pendingY = startWinY;

		function onMouseMove(ev: MouseEvent) {
			pendingX = Math.round(startWinX + ev.screenX * dpr - startMouseX);
			pendingY = Math.round(startWinY + ev.screenY * dpr - startMouseY);
			if (rafId !== null) return;
			rafId = requestAnimationFrame(() => {
				rafId = null;
				win.setPosition(new PhysicalPosition(pendingX, pendingY));
			});
		}

		function onMouseUp() {
			if (rafId !== null) {
				cancelAnimationFrame(rafId);
				rafId = null;
			}
			window.removeEventListener('mousemove', onMouseMove);
			window.removeEventListener('mouseup', onMouseUp);
		}

		window.addEventListener('mousemove', onMouseMove);
		window.addEventListener('mouseup', onMouseUp);
	}

	onMount(async () => {
		await vadStore.init();

		function loadScript(src: string): Promise<void> {
			return new Promise((resolve, reject) => {
				const script = document.createElement('script');
				script.src = src;
				script.onload = () => resolve();
				script.onerror = () => reject(new Error(`Failed to load ${src}`));
				document.head.appendChild(script);
			});
		}

		await Promise.all([loadScript('/live2d.min.js'), loadScript('/live2dcubismcore.min.js')]);

		await initApp(container);

		const savedTransform = await getLive2dTransform();
		setModelTransform(savedTransform.scale, savedTransform.offset_x, savedTransform.offset_y);

		unlistenTransform = await listen<{ scale: number; offset_x: number; offset_y: number }>(
			'set-live2d-transform',
			(event) => {
				setModelTransform(event.payload.scale, event.payload.offset_x, event.payload.offset_y);
			}
		);

		let savedUrl = await getCurrentModelUrl();
		if (!savedUrl) {
			const savedPath = await getModelPath();
			if (savedPath) {
				await setModelPath(savedPath);
				savedUrl = await getCurrentModelUrl();
			}
		}
		if (savedUrl) {
			await loadModelFromUrl(savedUrl);
			sendLive2dSync(getExpressionNames()).catch(console.warn);
		}

		unlisten = await listen<string>('load-model', async (event) => {
			await loadModelFromUrl(event.payload);
			sendLive2dSync(getExpressionNames()).catch(console.warn);
		});

		unlistenCursor = await listen<{ x: number; y: number }>('cursor-pos', (event) => {
			setFocus(event.payload.x, event.payload.y);
		});

		unlistenEvent = await onEvent((msg) => {
			if (msg.event === 'emotion') {
				setExpression(msg.data.emotion);
			} else if (msg.event === 'audio') {
				playAudio(msg.data.audio, msg.data.format, msg.data.duration_ms);
			} else if (msg.event === 'lipsync' && needsBackendLipsync()) {
				attachLipsyncToLast(msg.data.visemes);
			}
		});

		const unlistenIdleEvt = await listen('user-idle', () => {
			import('@tauri-apps/api/event').then(({ emit }) => emit('user-idle-forward', {})).catch(() => {});
		});
		const unlistenActiveEvt = await listen('user-active', () => {
			import('@tauri-apps/api/event').then(({ emit }) => emit('user-active-forward', {})).catch(() => {});
		});
		unlistenIdle = () => {
			unlistenIdleEvt();
			unlistenActiveEvt();
		};

		// Pet detection: track horizontal oscillation over the model.
		window.addEventListener('mousemove', handlePetMove);
	});

	onDestroy(() => {
		unlisten?.();
		unlistenCursor?.();
		unlistenEvent?.();
		unlistenIdle?.();
		unlistenTransform?.();
		window.removeEventListener('mousemove', handlePetMove);
		vadStore.stop();
		destroyApp();
	});
</script>

<div bind:this={container}></div>

<aside class="sidebar">
	<button class="icon-btn drag-btn" title="拖動視窗" onmousedown={onDragMouseDown} style="margin-top: auto;">
		<span class="icon-[material-symbols--drag-pan-rounded]"></span>
	</button>
	<button class="icon-btn" title="開啟聊天" onclick={openChat}>
		<span class="icon-[material-symbols--chat-outline-rounded]"></span>
	</button>
	<button
		class="icon-btn"
		class:vad-active={vadStore.active}
		title={vadStore.active ? '停止語音偵測' : '開始語音偵測'}
		onclick={() => vadStore.toggle()}
		disabled={vadStore.busy}
	>
		<span class={vadStore.active ? 'icon-[material-symbols--mic]' : 'icon-[material-symbols--mic-off-outline-rounded]'}
		></span>
	</button>
	<button class="icon-btn" title="設定" onclick={openSettings}>
		<span class="icon-[material-symbols--settings-outline-rounded]"></span>
	</button>
	<button class="icon-btn close-btn" title="關閉" onclick={hideWindow}>
		<span class="icon-[material-symbols--close-rounded]"></span>
	</button>
</aside>

<style>
	:global(html, body) {
		background: transparent !important;
		margin: 0;
		padding: 0;
		overflow: hidden;
	}

	div {
		position: fixed;
		inset: 0;
		z-index: 0;
	}

	div :global(canvas) {
		width: 100% !important;
		height: 100% !important;
	}

	.sidebar {
		position: fixed;
		right: 0;
		top: 0;
		height: 100%;
		width: 48px;
		z-index: 10;
		display: flex;
		flex-direction: column;
		align-items: center;
		padding: 8px 0;
		gap: 6px;
	}

	.icon-btn {
		width: 34px;
		height: 34px;
		border-radius: 8px;
		border: none;
		background: rgba(255, 255, 255, 0.05);
		color: rgba(255, 255, 255, 0.7);
		cursor: pointer;
		display: flex;
		align-items: center;
		justify-content: center;
		font-size: 18px;
		transition:
			background 0.15s,
			color 0.15s;
	}

	.icon-btn:hover {
		background: rgba(255, 255, 255, 0.12);
		color: #fff;
	}

	.icon-btn.vad-active {
		background: rgba(239, 68, 68, 0.25);
		color: rgb(239, 68, 68);
	}

	.icon-btn.vad-active:hover {
		background: rgba(239, 68, 68, 0.4);
	}

	.close-btn:hover {
		background: rgba(220, 60, 60, 0.4);
		color: #fff;
	}

	.drag-btn {
		cursor: grab;
	}

	.drag-btn:active {
		cursor: grabbing;
	}
</style>
