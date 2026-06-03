<script lang="ts">
import { Button, Field, Input, Select, Setting } from '@waifu/ui'
import { onMount } from 'svelte'
import {
	configToContext,
	contextToConfig,
	defaultContextConfig,
	nullableNumber,
	numberValue,
	withDefaults
} from '$lib/backend/config'
import { batchUpdateConfig, getAllConfig } from '$lib/backend/http'
import type { ContextConfig, HistoryStrategy } from '$lib/backend/types'

type ContextForm = Omit<ContextConfig, 'context_token_budget'> & {
	context_token_budget: string | number
}

const historyStrategyOptions = [
	{ value: 'sliding', label: 'Sliding' },
	{ value: 'summary', label: 'Summary' },
	{ value: 'full', label: 'Full' }
]

let context = $state<ContextForm>(contextToForm(defaultContextConfig))
let loading = $state(true)
let saving = $state(false)
let error = $state<string | null>(null)
let saved = $state(false)

function contextToForm(
	value: Partial<ContextConfig> | null | undefined
): ContextForm {
	const merged = withDefaults(defaultContextConfig, value)
	return {
		...merged,
		context_token_budget: merged.context_token_budget ?? ''
	}
}

function cleanContext(): ContextConfig | null {
	const maxHistoryMessages = numberValue(
		context.max_history_messages,
		Number.NaN
	)
	const contextTokenBudget = nullableNumber(context.context_token_budget)
	const compactThreshold = numberValue(context.compact_threshold, Number.NaN)

	if (!Number.isFinite(maxHistoryMessages) || maxHistoryMessages < 0) {
		error = 'max history messages 必須是 0 以上的數字'
		return null
	}
	if (contextTokenBudget !== null && contextTokenBudget <= 0) {
		error = 'context token budget 必須大於 0，或留空表示不限制'
		return null
	}
	if (
		!Number.isFinite(compactThreshold) ||
		compactThreshold <= 0 ||
		compactThreshold > 1
	) {
		error = 'compact threshold 必須介於 0 到 1'
		return null
	}

	return {
		...context,
		system_prompt_template: context.system_prompt_template,
		soul_content: context.soul_content ?? null,
		max_history_messages: maxHistoryMessages,
		history_strategy: context.history_strategy,
		context_token_budget: contextTokenBudget,
		compact_threshold: compactThreshold
	}
}

async function loadConfig() {
	loading = true
	error = null
	try {
		const config = await getAllConfig()
		context = contextToForm(configToContext(config))
	} catch (e) {
		error = String(e)
	} finally {
		loading = false
	}
}

async function saveConfig() {
	error = null
	saved = false
	const cleanedContext = cleanContext()
	if (!cleanedContext) return

	saving = true
	try {
		const config = await batchUpdateConfig(contextToConfig(cleanedContext))
		context = contextToForm(configToContext(config))
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
		title="對話歷史策略"
		icon="icon-[material-symbols--history-outline-rounded]"
		description="控制後端送入模型的歷史訊息數量與保留策略"
	>
		<div class="flex flex-col gap-3">
			{#if loading}
				<p class="text-muted-foreground text-xs">載入中…</p>
			{:else}
				<div class="grid gap-3 md:grid-cols-2">
					<Field label="History Strategy" required>
						<Select
							options={historyStrategyOptions}
							bind:value={context.history_strategy}
							onValueChange={(value) => (context.history_strategy = value as HistoryStrategy)}
						/>
					</Field>
					<Field label="Max History Messages" required>
						<Input type="number" min="0" bind:value={context.max_history_messages} />
					</Field>
				</div>
			{/if}
		</div>
	</Setting.Card>

	<Setting.Card
		title="Context Budget"
		icon="icon-[material-symbols--data-usage-outline-rounded]"
		description="設定 context token 預算與觸發額外截斷的門檻；prompt 內容不在此頁提供編輯"
	>
		<div class="flex flex-col gap-3">
			{#if !loading}
				<div class="grid gap-3 md:grid-cols-2">
					<Field label="Context Token Budget" hint="空白代表不限制">
						<Input type="number" min="1" bind:value={context.context_token_budget} />
					</Field>
					<Field label="Compact Threshold" hint="0 到 1，例如 0.85" required>
						<Input type="number" min="0" max="1" step="0.01" bind:value={context.compact_threshold} />
					</Field>
				</div>
			{/if}
		</div>
	</Setting.Card>

	<div class="flex items-center gap-2">
		<Button variant="primary" onclick={saveConfig} disabled={loading || saving}>
			{saving ? '儲存中…' : '儲存設定'}
		</Button>
		{#if saved}
			<span class="text-primary text-xs">已儲存</span>
		{/if}
	</div>
</div>
