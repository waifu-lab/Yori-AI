<script lang="ts">
import { Button, Field, Input, Setting, Switch } from '@waifu/ui'
import { onMount } from 'svelte'
import {
	configToNode,
	defaultNodeConfig,
	nodeToConfig,
	numberValue,
	requiredString
} from '$lib/backend/config'
import { batchUpdateConfig, getAllConfig } from '$lib/backend/http'
import type { WaifuNodeConfig } from '$lib/backend/types'

let node = $state<WaifuNodeConfig>({ ...defaultNodeConfig })
let loading = $state(true)
let saving = $state(false)
let error = $state<string | null>(null)
let saved = $state(false)

async function loadConfig() {
	loading = true
	error = null
	try {
		const config = await getAllConfig()
		node = configToNode(config)
	} catch (e) {
		error = String(e)
	} finally {
		loading = false
	}
}

function cleanNode(): WaifuNodeConfig | null {
	const path = requiredString(node.path)
	const invokeTimeoutMs = numberValue(node.invoke_timeout_ms, Number.NaN)
	const pingIntervalS = numberValue(node.ping_interval_s, Number.NaN)

	if (!path) {
		error = '請輸入 Node path'
		return null
	}
	if (!Number.isFinite(invokeTimeoutMs) || invokeTimeoutMs <= 0) {
		error = 'invoke timeout 必須大於 0'
		return null
	}
	if (!Number.isFinite(pingIntervalS) || pingIntervalS <= 0) {
		error = 'ping interval 必須大於 0'
		return null
	}

	return {
		enabled: node.enabled,
		path,
		token: node.token,
		invoke_timeout_ms: invokeTimeoutMs,
		ping_interval_s: pingIntervalS
	}
}

async function saveConfig() {
	error = null
	saved = false
	const cleaned = cleanNode()
	if (!cleaned) return

	saving = true
	try {
		const config = await batchUpdateConfig(nodeToConfig(cleaned))
		node = configToNode(config)
		saved = true
	} catch (e) {
		error = String(e)
	} finally {
		saving = false
	}
}

onMount(loadConfig)
</script>

<div class="flex flex-col gap-3">
	{#if error}
		<p class="text-danger text-xs">{error}</p>
	{/if}

	<Setting.Card
		title="Waifu-Node"
		icon="icon-[material-symbols--account-tree-outline-rounded]"
		description="設定本機電腦控制節點的後端連線路徑、驗證 token 與呼叫逾時"
	>
		<div class="flex flex-col gap-3">
			{#if loading}
				<p class="text-muted-foreground text-xs">載入中…</p>
			{:else}
				<Switch id="node-enabled" label="啟用 Waifu-Node" bind:checked={node.enabled} />

				<div class="grid gap-3 md:grid-cols-2">
					<Field label="Path" required>
						<Input bind:value={node.path} placeholder="/node" />
					</Field>
					<Field label="Token" hint="空字串代表不驗證">
						<Input type="password" bind:value={node.token} />
					</Field>
					<Field label="Invoke Timeout (ms)" required>
						<Input type="number" min="1" bind:value={node.invoke_timeout_ms} />
					</Field>
					<Field label="Ping Interval (s)" required>
						<Input type="number" min="1" bind:value={node.ping_interval_s} />
					</Field>
				</div>

				<div class="flex items-center gap-2">
					<Button variant="primary" onclick={saveConfig} disabled={saving}>
						{saving ? '儲存中…' : '儲存設定'}
					</Button>
					{#if saved}
						<span class="text-primary text-xs">已儲存</span>
					{/if}
				</div>
			{/if}
		</div>
	</Setting.Card>
</div>
