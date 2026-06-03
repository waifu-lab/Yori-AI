<script lang="ts">
import { Badge, Button, Field, Input, Setting } from '@waifu/ui'
import { onMount } from 'svelte'
import {
	configToGateway,
	defaultGatewayConfig,
	gatewayToConfig,
	numberValue,
	requiredString
} from '$lib/backend/config'
import { batchUpdateConfig, getAllConfig, getStatus } from '$lib/backend/http'
import type { GatewayConfig } from '$lib/backend/types'

type StatusData = Record<string, unknown>

let status = $state<StatusData | null>(null)
let statusError = $state<string | null>(null)
let statusLoading = $state(true)

let gateway = $state<GatewayConfig>({ ...defaultGatewayConfig })
let gatewayLoading = $state(true)
let gatewaySaving = $state(false)
let gatewayError = $state<string | null>(null)
let gatewaySaved = $state(false)

async function loadStatus() {
	statusLoading = true
	statusError = null
	try {
		status = await getStatus()
	} catch (e) {
		statusError = String(e)
	} finally {
		statusLoading = false
	}
}

async function loadGateway() {
	gatewayLoading = true
	gatewayError = null
	try {
		const config = await getAllConfig()
		gateway = configToGateway(config)
	} catch (e) {
		gatewayError = String(e)
	} finally {
		gatewayLoading = false
	}
}

function cleanGateway(): GatewayConfig | null {
	const host = requiredString(gateway.host)
	const port = numberValue(gateway.port)
	const sessionTtl = numberValue(gateway.session_ttl)
	const laneMaxQueue = numberValue(gateway.lane_max_queue)

	if (!host) {
		gatewayError = '請輸入 Gateway host'
		return null
	}
	if (port <= 0 || port > 65535) {
		gatewayError = 'port 必須介於 1 到 65535'
		return null
	}
	if (sessionTtl <= 0) {
		gatewayError = 'session TTL 必須大於 0'
		return null
	}
	if (laneMaxQueue <= 0) {
		gatewayError = 'lane max queue 必須大於 0'
		return null
	}

	return {
		host,
		port,
		session_ttl: sessionTtl,
		lane_max_queue: laneMaxQueue
	}
}

async function saveGateway() {
	gatewayError = null
	gatewaySaved = false
	const cleaned = cleanGateway()
	if (!cleaned) return

	gatewaySaving = true
	try {
		const config = await batchUpdateConfig(gatewayToConfig(cleaned))
		gateway = configToGateway(config)
		gatewaySaved = true
	} catch (e) {
		gatewayError = String(e)
	} finally {
		gatewaySaving = false
	}
}

onMount(() => {
	void loadStatus()
	void loadGateway()
})
</script>

<div class="flex flex-col gap-3">
	<Setting.Card
		title="系統狀態"
		icon="icon-[material-symbols--monitor-heart-outline-rounded]"
		description="後端服務的即時運行狀態"
	>
		{#snippet control()}
			<Button variant="ghost" onclick={loadStatus} disabled={statusLoading}>重新整理</Button>
		{/snippet}

		{#if statusLoading}
			<p class="text-muted-foreground text-xs">載入中…</p>
		{:else if statusError}
			<p class="text-danger text-xs">無法取得狀態：{statusError}</p>
		{:else if status}
			<div class="flex flex-col gap-1.5">
				{#each Object.entries(status) as [key, value]}
					<div class="flex items-center justify-between gap-2">
						<span class="text-muted-foreground text-xs">{key}</span>
						<Badge
							variant={value === true || value === 'ok' || value === 'running'
								? 'primary'
								: value === false || value === 'error'
									? 'danger'
									: 'muted'}
						>
							{String(value)}
						</Badge>
					</div>
				{/each}
			</div>
		{/if}
	</Setting.Card>

	<Setting.Card
		title="Gateway"
		icon="icon-[material-symbols--settings-input-component-outline-rounded]"
		description="後端 Gateway 連線與佇列設定；host/port 變更可能需要重啟後端或重新連線才會生效"
	>
		<div class="flex flex-col gap-3">
			{#if gatewayLoading}
				<p class="text-muted-foreground text-xs">載入中…</p>
			{:else}
				<div class="grid gap-3 md:grid-cols-2">
					<Field label="Host" required>
						<Input bind:value={gateway.host} placeholder="localhost" />
					</Field>
					<Field label="Port" required>
						<Input type="number" min="1" max="65535" bind:value={gateway.port} />
					</Field>
					<Field label="Session TTL (秒)" required>
						<Input type="number" min="1" bind:value={gateway.session_ttl} />
					</Field>
					<Field label="Lane Max Queue" required>
						<Input type="number" min="1" bind:value={gateway.lane_max_queue} />
					</Field>
				</div>

				<div class="flex items-center gap-2">
					<Button variant="primary" onclick={saveGateway} disabled={gatewaySaving}>
						{gatewaySaving ? '儲存中…' : '儲存設定'}
					</Button>
					{#if gatewaySaved}
						<span class="text-primary text-xs">已儲存</span>
					{/if}
				</div>
			{/if}

			{#if gatewayError}
				<p class="text-danger text-xs">{gatewayError}</p>
			{/if}
		</div>
	</Setting.Card>
</div>
