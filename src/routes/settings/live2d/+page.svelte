<script lang="ts">
	import { onMount } from 'svelte';
	import { open } from '@tauri-apps/plugin-dialog';
	import { getModelPath, setModelPath } from '$lib/commands';
	import { live2dStore } from '$lib/stores/live2d.svelte';
	import { Button, Setting, Range } from '@waifu/ui';

	let modelPath = $state<string | null>(null);
	let loading = $state(true);
	let applying = $state(false);
	let saving = $state(false);
	let error = $state<string | null>(null);

	onMount(async () => {
		const [path] = await Promise.all([getModelPath(), live2dStore.load()]);
		modelPath = path;
		loading = false;
	});

	$effect(() => {
		live2dStore.scale;
		live2dStore.offsetX;
		live2dStore.offsetY;
		live2dStore.preview();
	});

	async function pickModel() {
		const selected = await open({
			title: '選擇 Live2D 模型',
			filters: [{ name: 'Live2D Model', extensions: ['json'] }],
			directory: false,
			multiple: false
		});
		if (!selected) return;
		await applyPath(selected as string);
	}

	async function pickDirectory() {
		const selected = await open({
			title: '選擇 Live2D 模型資料夾',
			directory: true,
			multiple: false
		});
		if (!selected) return;
		await applyPath(selected as string);
	}

	async function applyPath(path: string) {
		applying = true;
		error = null;
		try {
			await setModelPath(path);
			modelPath = path;
		} catch (e) {
			error = String(e);
		} finally {
			applying = false;
		}
	}

	async function saveTransform() {
		saving = true;
		try {
			await live2dStore.save();
		} catch (e) {
			error = String(e);
		} finally {
			saving = false;
		}
	}
</script>

<div class="flex flex-col gap-3">
	<Setting.Card
		title="Live2D 模型"
		icon="icon-[material-symbols--person-outline-rounded]"
		description="選擇要顯示的 Live2D 模型檔案或資料夾"
	>
		<div class="flex flex-col gap-2">
			{#if loading}
				<p class="text-muted-foreground text-xs">載入中…</p>
			{:else}
				<p class="text-muted-foreground truncate text-xs">
					{modelPath ?? '尚未選擇模型'}
				</p>
			{/if}

			<div class="flex gap-2">
				<Button variant="primary" onclick={pickModel} disabled={applying}>選擇 .json 檔案</Button>
				<Button variant="secondary" onclick={pickDirectory} disabled={applying}>選擇資料夾</Button>
			</div>

			{#if error}
				<p class="text-destructive text-xs">{error}</p>
			{/if}
		</div>
	</Setting.Card>

	<Setting.Card
		title="模型顯示調整"
		icon="icon-[material-symbols--open-with-rounded]"
		description="拖曳滑桿即時預覽"
	>
		<div class="flex flex-col gap-4">
			<div class="flex flex-col gap-1.5">
				<div class="flex items-center justify-between">
					<label class="text-sm font-medium">縮放大小</label>
					<span class="text-muted-foreground w-12 text-right text-xs tabular-nums">{live2dStore.scale.toFixed(2)}x</span
					>
				</div>
				<Range min={0.1} max={3} step={0.05} bind:value={live2dStore.scale} />
				<div class="text-muted-foreground flex justify-between text-xs">
					<span>0.1x</span>
					<span>3x</span>
				</div>
			</div>

			<div class="flex flex-col gap-1.5">
				<div class="flex items-center justify-between">
					<label class="text-sm font-medium">水平位移</label>
					<span class="text-muted-foreground w-16 text-right text-xs tabular-nums">{live2dStore.offsetX}px</span>
				</div>
				<Range min={-500} max={500} step={1} bind:value={live2dStore.offsetX} />
				<div class="text-muted-foreground flex justify-between text-xs">
					<span>-500px</span>
					<span>+500px</span>
				</div>
			</div>

			<div class="flex flex-col gap-1.5">
				<div class="flex items-center justify-between">
					<label class="text-sm font-medium">垂直位移</label>
					<span class="text-muted-foreground w-16 text-right text-xs tabular-nums">{live2dStore.offsetY}px</span>
				</div>
				<Range min={-500} max={500} step={1} bind:value={live2dStore.offsetY} />
				<div class="text-muted-foreground flex justify-between text-xs">
					<span>-500px</span>
					<span>+500px</span>
				</div>
			</div>

			<div class="flex gap-2">
				<Button variant="primary" onclick={saveTransform} disabled={saving}>確認</Button>
				<Button variant="secondary" onclick={() => live2dStore.reset()}>重設</Button>
			</div>
		</div>
	</Setting.Card>
</div>
