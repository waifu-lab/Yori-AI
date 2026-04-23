<script lang="ts">
	import { onMount } from 'svelte';
	import { open } from '@tauri-apps/plugin-dialog';
	import { Setting, Button } from '@waifu/ui';
	import { getModelPath, setModelPath } from '$lib/commands';

	let modelPath = $state<string | null>(null);
	let loading = $state(true);
	let applying = $state(false);
	let error = $state<string | null>(null);

	onMount(async () => {
		modelPath = await getModelPath();
		loading = false;
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
</div>
