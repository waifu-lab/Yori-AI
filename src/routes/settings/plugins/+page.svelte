<script lang="ts">
	import { onMount } from 'svelte';
	import { Setting, Button, Badge, Dialog, Textarea, Field, Select } from '@waifu/ui';
	import {
		listPlugins,
		listAvailablePlugins,
		installPlugin,
		enablePlugin,
		disablePlugin,
		uninstallPlugin,
		type PluginItem,
		type AvailablePluginItem
	} from '$lib/backend/http';

	let installed = $state<PluginItem[]>([]);
	let available = $state<AvailablePluginItem[]>([]);
	let loading = $state(false);
	let error = $state<string | null>(null);

	let dialogOpen = $state(false);
	let selectedPlugin = $state<string>('');
	let configText = $state<string>('');
	let formError = $state<string | null>(null);
	let submitting = $state(false);

	const installedNames = $derived(new Set(installed.map((p) => p.name)));
	const installablePlugins = $derived(available.filter((p) => !installedNames.has(p.name)));
	const installOptions = $derived(installablePlugins.map((p) => ({ value: p.name, label: p.name })));

	async function refresh() {
		loading = true;
		error = null;
		try {
			[installed, available] = await Promise.all([listPlugins(), listAvailablePlugins()]);
		} catch (e) {
			error = String(e);
		} finally {
			loading = false;
		}
	}

	function openInstallDialog() {
		formError = null;
		selectedPlugin = installablePlugins[0]?.name ?? '';
		configText = '';
		dialogOpen = true;
	}

	async function onInstall() {
		formError = null;
		if (!selectedPlugin) {
			formError = '請選擇要安裝的外掛';
			return;
		}
		let cfg: Record<string, unknown> | undefined;
		if (configText.trim()) {
			try {
				cfg = JSON.parse(configText);
			} catch {
				formError = 'config 必須是合法的 JSON';
				return;
			}
		}
		submitting = true;
		try {
			await installPlugin(selectedPlugin, cfg);
			dialogOpen = false;
			await refresh();
		} catch (e) {
			formError = String(e);
		} finally {
			submitting = false;
		}
	}

	async function toggle(item: PluginItem) {
		try {
			if (item.enabled) await disablePlugin(item.name);
			else await enablePlugin(item.name);
			await refresh();
		} catch (e) {
			error = String(e);
		}
	}

	async function uninstall(item: PluginItem) {
		if (!confirm(`確認解除安裝外掛 "${item.name}"？`)) return;
		try {
			await uninstallPlugin(item.name);
			await refresh();
		} catch (e) {
			error = String(e);
		}
	}

	onMount(refresh);
</script>

<div class="flex flex-col gap-3">
	<div class="flex items-center justify-between">
		<p class="text-muted-foreground text-xs">管理已安裝的外掛 / 技能</p>
		<div class="flex gap-2">
			<Button variant="ghost" onclick={refresh} disabled={loading}>重新整理</Button>
			<Button variant="primary" onclick={openInstallDialog} disabled={installablePlugins.length === 0}>
				安裝外掛
			</Button>
		</div>
	</div>

	{#if error}
		<p class="text-destructive text-xs">{error}</p>
	{/if}

	{#if installed.length === 0 && !loading}
		<Setting.Card title="尚未安裝任何外掛" icon="icon-[material-symbols--extension-outline-rounded]" />
	{/if}

	{#each installed as item (item.name)}
		<Setting.Card
			title={item.name + (item.version ? ` v${item.version}` : '')}
			description={item.description ?? ''}
			icon="icon-[material-symbols--extension-outline-rounded]"
		>
			{#snippet control()}
				<div class="flex items-center gap-2">
					<Badge variant={item.enabled ? 'primary' : 'muted'}>{item.enabled ? '啟用' : '停用'}</Badge>
					<Button variant="ghost" onclick={() => toggle(item)}>
						{item.enabled ? '停用' : '啟用'}
					</Button>
					<Button variant="danger" onclick={() => uninstall(item)}>解除安裝</Button>
				</div>
			{/snippet}
		</Setting.Card>
	{/each}

	{#if available.length > 0}
		<div class="mt-4">
			<p class="text-muted-foreground mb-2 text-xs font-medium">可用外掛</p>
			{#each available as p (p.name)}
				<Setting.Card
					title={p.name + (p.version ? ` v${p.version}` : '')}
					description={p.description ?? ''}
					icon="icon-[material-symbols--extension-outline-rounded]"
					class="mb-2"
				>
					{#snippet control()}
						{#if installedNames.has(p.name)}
							<Badge variant="muted">已安裝</Badge>
						{:else}
							<Button
								variant="primary"
								onclick={() => {
									selectedPlugin = p.name;
									configText = '';
									formError = null;
									dialogOpen = true;
								}}>安裝</Button
							>
						{/if}
					{/snippet}
				</Setting.Card>
			{/each}
		</div>
	{/if}
</div>

<Dialog bind:open={dialogOpen} title="安裝外掛" size="md">
	<div class="flex flex-col gap-3">
		<Field label="外掛" required>
			{#if installOptions.length > 0}
				<Select options={installOptions} bind:value={selectedPlugin} />
			{:else}
				<p class="text-muted-foreground text-xs">沒有可安裝的外掛</p>
			{/if}
		</Field>

		<Field label="初始設定 (JSON，可選)">
			<Textarea bind:value={configText} placeholder={'{\n  "key": "value"\n}'} rows={5} />
		</Field>

		{#if formError}
			<p class="text-destructive text-xs">{formError}</p>
		{/if}
	</div>

	{#snippet footer()}
		<Button variant="ghost" onclick={() => (dialogOpen = false)} disabled={submitting}>取消</Button>
		<Button variant="primary" onclick={onInstall} disabled={submitting || !selectedPlugin}>
			{submitting ? '安裝中…' : '安裝'}
		</Button>
	{/snippet}
</Dialog>
