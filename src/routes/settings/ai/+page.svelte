<script lang="ts">
import { Button, Field, Input, Setting, Switch } from '@waifu/ui'
import { onMount } from 'svelte'
import {
	configToProvider,
	defaultProviderConfig,
	nullableNumber,
	nullableString,
	numberValue,
	providerToConfig,
	requiredString,
	utilityProviderToConfig,
	withDefaults
} from '$lib/backend/config'
import { batchUpdateConfig, getAllConfig } from '$lib/backend/http'
import type { ProviderConfig } from '$lib/backend/types'

type ProviderForm = Omit<ProviderConfig, 'base_url' | 'max_tokens'> & {
	base_url: string
	max_tokens: string | number
}

let provider = $state<ProviderForm>(providerToForm(defaultProviderConfig))
let utilityProvider = $state<ProviderForm>(
	providerToForm(defaultProviderConfig)
)
let utilityEnabled = $state(false)
let loading = $state(true)
let saving = $state(false)
let error = $state<string | null>(null)
let saved = $state(false)

function providerToForm(
	value: Partial<ProviderConfig> | null | undefined
): ProviderForm {
	const merged = withDefaults(defaultProviderConfig, value)
	return {
		provider: merged.provider,
		api_key: merged.api_key,
		base_url: merged.base_url ?? '',
		model: merged.model,
		temperature: merged.temperature,
		max_tokens: merged.max_tokens ?? ''
	}
}

function cleanProvider(
	form: ProviderForm,
	label: string
): ProviderConfig | null {
	const providerName = requiredString(form.provider)
	const apiKey = requiredString(form.api_key)
	const model = requiredString(form.model)
	const temperature = numberValue(form.temperature, Number.NaN)
	const maxTokens = nullableNumber(form.max_tokens)

	if (!providerName) {
		error = `請輸入${label} provider`
		return null
	}
	if (!apiKey) {
		error = `請輸入${label} API key`
		return null
	}
	if (!model) {
		error = `請輸入${label} model`
		return null
	}
	if (!Number.isFinite(temperature)) {
		error = `${label} temperature 必須是數字`
		return null
	}
	if (maxTokens !== null && maxTokens <= 0) {
		error = `${label} max tokens 必須大於 0`
		return null
	}

	return {
		provider: providerName,
		api_key: apiKey,
		base_url: nullableString(form.base_url),
		model,
		temperature,
		max_tokens: maxTokens
	}
}

async function loadConfig() {
	loading = true
	error = null
	try {
		const config = await getAllConfig()
		provider = providerToForm(configToProvider(config, 'provider'))
		utilityEnabled = !!config['utility_provider.provider']
		utilityProvider = providerToForm(
			utilityEnabled
				? configToProvider(config, 'utility_provider')
				: defaultProviderConfig
		)
	} catch (e) {
		error = String(e)
	} finally {
		loading = false
	}
}

async function saveConfig() {
	error = null
	saved = false
	const cleanedProvider = cleanProvider(provider, '主要')
	if (!cleanedProvider) return
	const cleanedUtilityProvider = utilityEnabled
		? cleanProvider(utilityProvider, '輔助')
		: null
	if (utilityEnabled && !cleanedUtilityProvider) return

	saving = true
	try {
		const config = await batchUpdateConfig({
			...providerToConfig(cleanedProvider),
			...utilityProviderToConfig(cleanedUtilityProvider)
		})
		provider = providerToForm(configToProvider(config, 'provider'))
		utilityEnabled = !!config['utility_provider.provider']
		utilityProvider = providerToForm(
			utilityEnabled
				? configToProvider(config, 'utility_provider')
				: defaultProviderConfig
		)
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
		title="主要模型供應商"
		icon="icon-[material-symbols--smart-toy-outline-rounded]"
		description="設定主要對話模型的 provider、模型名稱與 API 連線資訊"
	>
		<div class="flex flex-col gap-3">
			{#if loading}
				<p class="text-muted-foreground text-xs">載入中…</p>
			{:else}
				<div class="grid gap-3 md:grid-cols-2">
					<Field label="Provider" required>
						<Input bind:value={provider.provider} placeholder="openai" />
					</Field>
					<Field label="Model" required>
						<Input bind:value={provider.model} placeholder="gpt-4o" />
					</Field>
					<Field label="API Key" required>
						<Input type="password" bind:value={provider.api_key} placeholder="sk-..." />
					</Field>
					<Field label="Base URL" hint="自訂 provider 可填；空白會保存為 null">
						<Input bind:value={provider.base_url} placeholder="https://api.openai.com/v1/chat/completions" />
					</Field>
					<Field label="Temperature" required>
						<Input type="number" step="0.1" bind:value={provider.temperature} />
					</Field>
					<Field label="Max Tokens" hint="空白代表不限制">
						<Input type="number" min="1" bind:value={provider.max_tokens} />
					</Field>
				</div>
			{/if}
		</div>
	</Setting.Card>

	<Setting.Card
		title="輔助模型供應商"
		icon="icon-[material-symbols--bolt-outline-rounded]"
		description="可選的 utility provider，用於較輕量或工具型模型呼叫"
	>
		<div class="flex flex-col gap-3">
			<Switch id="utility-provider-toggle" label="啟用輔助模型供應商" bind:checked={utilityEnabled} disabled={loading} />

			{#if utilityEnabled && !loading}
				<div class="grid gap-3 md:grid-cols-2">
					<Field label="Provider" required>
						<Input bind:value={utilityProvider.provider} placeholder="openai" />
					</Field>
					<Field label="Model" required>
						<Input bind:value={utilityProvider.model} placeholder="gpt-4o" />
					</Field>
					<Field label="API Key" required>
						<Input type="password" bind:value={utilityProvider.api_key} placeholder="sk-..." />
					</Field>
					<Field label="Base URL" hint="自訂 provider 可填；空白會保存為 null">
						<Input bind:value={utilityProvider.base_url} placeholder="https://api.openai.com/v1/chat/completions" />
					</Field>
					<Field label="Temperature" required>
						<Input type="number" step="0.1" bind:value={utilityProvider.temperature} />
					</Field>
					<Field label="Max Tokens" hint="空白代表不限制">
						<Input type="number" min="1" bind:value={utilityProvider.max_tokens} />
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
