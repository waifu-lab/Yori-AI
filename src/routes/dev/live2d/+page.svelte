<script lang="ts">
	import { open } from '@tauri-apps/plugin-dialog';
	import { openLive2dWindow, closeLive2dWindow, setModelPath } from '$lib/commands';

	let modelPath = $state('');
	let status = $state<'idle' | 'running' | 'error'>('idle');
	let errorMsg = $state('');

	async function pickModelDir() {
		const selected = await open({ directory: true, multiple: false });
		if (typeof selected === 'string') {
			modelPath = selected;
		}
	}

	async function startPreview() {
		if (!modelPath) return;
		try {
			status = 'running';
			errorMsg = '';
			await setModelPath(modelPath);
			await openLive2dWindow();
		} catch (e) {
			status = 'error';
			errorMsg = String(e);
		}
	}

	async function stopPreview() {
		try {
			await closeLive2dWindow();
			status = 'idle';
		} catch (e) {
			errorMsg = String(e);
		}
	}
</script>

<div class="page">
	<h1>Live2D 預覽</h1>

	<div class="field">
		<label for="model-path">模型目錄</label>
		<div class="input-row">
			<input id="model-path" type="text" placeholder="選擇或輸入 model3.json 所在目錄" bind:value={modelPath} />
			<button onclick={pickModelDir}>瀏覽...</button>
		</div>
	</div>

	<div class="actions">
		<button class="primary" onclick={startPreview} disabled={!modelPath || status === 'running'}> 開啟預覽 </button>
		<button onclick={stopPreview} disabled={status !== 'running'}> 關閉預覽 </button>
	</div>

	{#if status === 'running'}
		<p class="info">Live2D 渲染中，請查看預覽視窗。</p>
	{/if}

	{#if errorMsg}
		<p class="error">{errorMsg}</p>
	{/if}
</div>

<style>
	.page {
		padding: 2rem;
		max-width: 600px;
		font-family: sans-serif;
    background: transparent;
	}

	h1 {
		margin-bottom: 1.5rem;
	}

	.field {
		margin-bottom: 1rem;
	}

	label {
		display: block;
		margin-bottom: 0.25rem;
		font-size: 0.875rem;
		font-weight: 600;
	}

	.input-row {
		display: flex;
		gap: 0.5rem;
	}

	input {
		flex: 1;
		padding: 0.4rem 0.6rem;
		border: 1px solid #555;
		border-radius: 4px;
		background: #1e1e2e;
		color: inherit;
		font-size: 0.875rem;
	}

	.actions {
		display: flex;
		gap: 0.75rem;
		margin-top: 1.25rem;
	}

	button {
		padding: 0.4rem 1rem;
		border: 1px solid #555;
		border-radius: 4px;
		background: #2a2a3e;
		color: inherit;
		cursor: pointer;
		font-size: 0.875rem;
	}

	button:disabled {
		opacity: 0.4;
		cursor: default;
	}

	button.primary {
		background: #5865f2;
		border-color: #5865f2;
		color: #fff;
	}

	.info {
		margin-top: 1rem;
		color: #7ec8a0;
		font-size: 0.875rem;
	}

	.error {
		margin-top: 1rem;
		color: #f28b82;
		font-size: 0.875rem;
	}
</style>
