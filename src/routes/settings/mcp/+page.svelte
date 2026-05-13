<script lang="ts">
	import { onMount } from 'svelte';
	import { Setting, Button, Badge, Dialog, Input, Textarea, Field, Select } from '@waifu/ui';
	import {
		listMCP,
		addMCP,
		enableMCP,
		disableMCP,
		removeMCP,
		type MCPServerItem,
		type AddMCPPayload
	} from '$lib/backend/http';

	let items = $state<MCPServerItem[]>([]);
	let loading = $state(false);
	let error = $state<string | null>(null);

	let dialogOpen = $state(false);
	let form = $state({
		name: '',
		type: 'stdio' as 'stdio' | 'sse',
		command: '',
		url: '',
		config: ''
	});
	let formError = $state<string | null>(null);
	let submitting = $state(false);

	const typeOptions = [
		{ value: 'stdio', label: 'stdio' },
		{ value: 'sse', label: 'sse' }
	];

	async function refresh() {
		loading = true;
		error = null;
		try {
			items = await listMCP();
		} catch (e) {
			error = String(e);
		} finally {
			loading = false;
		}
	}

	function resetForm() {
		form = { name: '', type: 'stdio', command: '', url: '', config: '' };
		formError = null;
	}

	async function onAdd() {
		formError = null;
		if (!form.name.trim()) {
			formError = '請輸入名稱';
			return;
		}
		const payload: AddMCPPayload = { name: form.name.trim(), type: form.type };
		if (form.type === 'stdio') {
			const cmd = form.command
				.split(/\s+/)
				.map((s) => s.trim())
				.filter(Boolean);
			if (cmd.length === 0) {
				formError = '請輸入啟動指令';
				return;
			}
			payload.command = cmd;
		} else {
			if (!form.url.trim()) {
				formError = '請輸入 URL';
				return;
			}
			payload.url = form.url.trim();
		}
		if (form.config.trim()) {
			try {
				payload.config = JSON.parse(form.config);
			} catch {
				formError = 'config 必須是合法的 JSON';
				return;
			}
		}
		submitting = true;
		try {
			await addMCP(payload);
			dialogOpen = false;
			resetForm();
			await refresh();
		} catch (e) {
			formError = String(e);
		} finally {
			submitting = false;
		}
	}

	async function toggle(item: MCPServerItem) {
		try {
			if (item.enabled) await disableMCP(item.name);
			else await enableMCP(item.name);
			await refresh();
		} catch (e) {
			error = String(e);
		}
	}

	async function remove(item: MCPServerItem) {
		if (!confirm(`確認移除 MCP 伺服器 "${item.name}"？`)) return;
		try {
			await removeMCP(item.name);
			await refresh();
		} catch (e) {
			error = String(e);
		}
	}

	onMount(refresh);
</script>

<div class="flex flex-col gap-3">
	<div class="flex items-center justify-between">
		<p class="text-muted-foreground text-xs">管理已註冊的 MCP 伺服器</p>
		<div class="flex gap-2">
			<Button variant="ghost" onclick={refresh} disabled={loading}>重新整理</Button>
			<Button
				variant="primary"
				onclick={() => {
					resetForm();
					dialogOpen = true;
				}}>新增</Button
			>
		</div>
	</div>

	{#if error}
		<p class="text-destructive text-xs">{error}</p>
	{/if}

	{#if items.length === 0 && !loading}
		<Setting.Card title="尚未註冊任何 MCP 伺服器" icon="icon-[material-symbols--hub-outline-rounded]" />
	{/if}

	{#each items as item (item.name)}
		<Setting.Card
			title={item.name}
			description={`類型：${item.type}`}
			icon="icon-[material-symbols--hub-outline-rounded]"
		>
			{#snippet control()}
				<div class="flex items-center gap-2">
					<Badge variant={item.enabled ? 'primary' : 'muted'}>{item.enabled ? '啟用' : '停用'}</Badge>
					<Button variant="ghost" onclick={() => toggle(item)}>
						{item.enabled ? '停用' : '啟用'}
					</Button>
					<Button variant="danger" onclick={() => remove(item)}>移除</Button>
				</div>
			{/snippet}
		</Setting.Card>
	{/each}
</div>

<Dialog bind:open={dialogOpen} title="新增 MCP 伺服器" size="md">
	<div class="flex flex-col gap-3">
		<Field label="名稱" required>
			<Input bind:value={form.name} placeholder="my-mcp-server" />
		</Field>

		<Field label="類型" required>
			<Select options={typeOptions} bind:value={form.type} />
		</Field>

		{#if form.type === 'stdio'}
			<Field label="啟動指令" hint="以空白分隔，例如：node ./server.js" required>
				<Input bind:value={form.command} placeholder="node ./server.js" />
			</Field>
		{:else}
			<Field label="URL" required>
				<Input bind:value={form.url} placeholder="https://example.com/sse" />
			</Field>
		{/if}

		<Field label="額外設定 (JSON，可選)">
			<Textarea bind:value={form.config} placeholder={'{\n  "env": { "KEY": "value" }\n}'} rows={4} />
		</Field>

		{#if formError}
			<p class="text-destructive text-xs">{formError}</p>
		{/if}
	</div>

	{#snippet footer()}
		<Button variant="ghost" onclick={() => (dialogOpen = false)} disabled={submitting}>取消</Button>
		<Button variant="primary" onclick={onAdd} disabled={submitting}>
			{submitting ? '新增中…' : '新增'}
		</Button>
	{/snippet}
</Dialog>
