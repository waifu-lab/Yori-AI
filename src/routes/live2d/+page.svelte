<script lang="ts">
	import { onMount, onDestroy } from 'svelte';
	import { listen } from '@tauri-apps/api/event';
	import { getCurrentModelUrl } from '$lib/commands';
	import { initApp, loadModelFromUrl, destroyApp, setExpression, playAudio, scheduleLipsync, needsBackendLipsync } from '$lib/live2d/renderer';
	import { onEvent } from '$lib/backend/ws';
	import type { UnlistenFn } from '@tauri-apps/api/event';

	let container: HTMLDivElement;
	let unlisten: (() => void) | null = null;
	let unlistenEvent: UnlistenFn | null = null;

	onMount(async () => {
		// Load both Cubism 2 and Cubism 4 runtimes before importing pixi-live2d-display
		function loadScript(src: string): Promise<void> {
			return new Promise((resolve, reject) => {
				const script = document.createElement('script');
				script.src = src;
				script.onload = () => resolve();
				script.onerror = () => reject(new Error(`Failed to load ${src}`));
				document.head.appendChild(script);
			});
		}

		await Promise.all([
			loadScript('/live2d.min.js'),
			loadScript('/live2dcubismcore.min.js'),
		]);

		await initApp(container);

		// Restore previously loaded model (survives page reload)
		const savedUrl = await getCurrentModelUrl();
		if (savedUrl) {
			await loadModelFromUrl(savedUrl);
		}

		// Listen for model-load events from the main window (payload is model URL)
		unlisten = await listen<string>('load-model', async (event) => {
			await loadModelFromUrl(event.payload);
		});

		// Handle Live2D events pushed from the backend (emotion / audio / lipsync)
		unlistenEvent = await onEvent((msg) => {
			if (msg.event === 'emotion') {
				setExpression(msg.data.emotion);
			} else if (msg.event === 'audio') {
				playAudio(msg.data.audio, msg.data.format);
			} else if (msg.event === 'lipsync' && needsBackendLipsync()) {
				// Only drive the mouth manually when the model has no native LipSync group.
				// If the model supports speak(), the engine handles lip sync internally.
				scheduleLipsync(msg.data.visemes);
			}
		});
	});

	onDestroy(() => {
		unlisten?.();
		unlistenEvent?.();
		destroyApp();
	});
</script>

<div bind:this={container}></div>

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
		width: 100vw;
		height: 100vh;
	}

	div :global(canvas) {
		width: 100% !important;
		height: 100% !important;
	}
</style>
