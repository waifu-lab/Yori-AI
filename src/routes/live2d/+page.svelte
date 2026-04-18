<script lang="ts">
	import { onMount, onDestroy } from 'svelte';
	import { listen } from '@tauri-apps/api/event';
	import { getCurrentModelUrl } from '$lib/commands';
	import { initApp, loadModelFromUrl, destroyApp } from '$lib/live2d/renderer';

	let container: HTMLDivElement;
	let unlisten: (() => void) | null = null;

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
	});

	onDestroy(() => {
		unlisten?.();
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
