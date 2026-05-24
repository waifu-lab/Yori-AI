<script lang="ts">
	import { onMount } from 'svelte';
	import { Setting, Button, Dialog, Input, Checkbox, Field } from '@waifu/ui';
	import {
		listSkills,
		installSkill,
		uninstallSkill,
		reloadSkills,
		type SkillItem
	} from '$lib/backend/http';

	let items = $state<SkillItem[]>([]);
	let loading = $state(false);
	let error = $state<string | null>(null);

	let dialogOpen = $state(false);
	let source = $state('');
	let overwrite = $state(false);
	let formError = $state<string | null>(null);
	let submitting = $state(false);
	let reloading = $state(false);

	async function refresh() {
		loading = true;
		error = null;
		try {
			items = await listSkills();
		} catch (e) {
			error = String(e);
		} finally {
			loading = false;
		}
	}

	function openDialog() {
		source = '';
		overwrite = false;
		formError = null;
		dialogOpen = true;
	}

	async function onInstall() {
		formError = null;
		if (!source.trim()) {
			formError = '請輸入來源（路徑或 URL）';
			return;
		}
		submitting = true;
		try {
			await installSkill(source.trim(), overwrite);
			dialogOpen = false;
			await refresh();
		} catch (e) {
			formError = String(e);
		} finally {
			submitting = false;
		}
	}

	async function uninstall(item: SkillItem) {
		if (!confirm(`確認解除安裝技能 "${item.name}"？`)) return;
		try {
			await uninstallSkill(item.name);
			await refresh();
		} catch (e) {
			error = String(e);
		}
	}

	async function onReload() {
		reloading = true;
		error = null;
		try {
			const r = await reloadSkills();
			items = await listSkills();
			error = `已重新載入 ${r.count} 個技能`;
		} catch (e) {
			error = String(e);
		} finally {
			reloading = false;
		}
	}

	onMount(refresh);
</script>

<div class="flex flex-col gap-3">
	<div class="flex items-center justify-between">
		<p class="text-muted-foreground text-xs">管理已安裝的技能（skills）</p>
		<div class="flex gap-2">
			<Button variant="ghost" onclick={refresh} disabled={loading}>重新整理</Button>
			<Button variant="ghost" onclick={onReload} disabled={reloading}>
				{reloading ? '載入中…' : '重新載入'}
			</Button>
			<Button variant="primary" onclick={openDialog}>安裝技能</Button>
		</div>
	</div>

	{#if error}
		<p class="text-destructive text-xs">{error}</p>
	{/if}

	{#if items.length === 0 && !loading}
		<Setting.Card title="尚未安裝任何技能" icon="icon-[material-symbols--auto-awesome-outline-rounded]" />
	{/if}

	{#each items as item (item.name)}
		<Setting.Card
			title={item.name + (item.version ? ` v${item.version}` : '')}
			description={item.description ?? ''}
			icon="icon-[material-symbols--auto-awesome-outline-rounded]"
		>
			{#snippet control()}
				<Button variant="danger" onclick={() => uninstall(item)}>解除安裝</Button>
			{/snippet}
		</Setting.Card>
	{/each}
</div>

<Dialog bind:open={dialogOpen} title="安裝技能" size="md">
	<div class="flex flex-col gap-3">
		<Field label="來源" hint="本地路徑、Git URL 或封裝檔位置" required>
			<Input bind:value={source} placeholder="/path/to/skill 或 https://..." />
		</Field>

		<label class="flex items-center gap-2 text-sm">
			<Checkbox bind:checked={overwrite} />
			若已存在則覆蓋
		</label>

		{#if formError}
			<p class="text-destructive text-xs">{formError}</p>
		{/if}
	</div>

	{#snippet footer()}
		<Button variant="ghost" onclick={() => (dialogOpen = false)} disabled={submitting}>取消</Button>
		<Button variant="primary" onclick={onInstall} disabled={submitting}>
			{submitting ? '安裝中…' : '安裝'}
		</Button>
	{/snippet}
</Dialog>
