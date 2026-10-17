import type {
	ConfigUpdate,
	ContextConfig,
	DeepLConfig,
	GatewayConfig,
	GPTSoVITSTTSConfig,
	HooksConfig,
	OpenAITTSConfig,
	ProviderConfig,
	STTConfig,
	TTSBackend,
	TTSConfig,
	VoiceConfig,
	WaifuConfig,
	WaifuNodeConfig
} from './types'

export const defaultGatewayConfig: GatewayConfig = {
	host: 'localhost',
	port: 8765,
	session_ttl: 3600,
	lane_max_queue: 10
}

export const defaultContextConfig: ContextConfig = {
	builtin_prompt: true,
	system_prompt_template: '',
	max_history_messages: 20,
	history_strategy: 'sliding',
	context_token_budget: null,
	compact_threshold: 0.85
}

export const defaultHooksConfig: HooksConfig = {
	emotion: true,
	tts: true,
	logging: true,
	rate_limit_rpm: null
}

export const defaultProviderConfig: ProviderConfig = {
	provider: '',
	api_key: '',
	base_url: null,
	model: '',
	temperature: 0.7,
	max_tokens: null
}

export const defaultSTTConfig: STTConfig = {
	api_key: '',
	base_url: 'https://api.openai.com/v1/audio/transcriptions',
	model: 'whisper-1',
	language: null,
	temperature: 0.0
}

export const defaultOpenAITTSConfig: OpenAITTSConfig = {
	backend: 'openai',
	api_key: '',
	base_url: 'https://api.openai.com/v1/audio/speech',
	model: 'tts-1',
	voice: 'nova',
	response_format: 'mp3',
	speed: 1.0,
	timeout_retries: 1,
	retry_backoff_s: 0.1
}

export const defaultGPTSoVITSTTSConfig: GPTSoVITSTTSConfig = {
	backend: 'gptsovits',
	base_url: 'http://127.0.0.1:9880',
	ref_audio_path: '',
	prompt_text: '',
	prompt_lang: 'zh',
	text_lang: 'zh',
	text_split_method: 'cut5',
	batch_size: 1,
	speed_factor: 1.0,
	streaming_mode: 0,
	top_k: 15,
	top_p: 1.0,
	temperature: 1.0,
	timeout: 60.0,
	timeout_retries: 1,
	retry_backoff_s: 0.1
}

export const defaultTTSConfigs: Record<TTSBackend, TTSConfig> = {
	openai: defaultOpenAITTSConfig,
	gptsovits: defaultGPTSoVITSTTSConfig
}

export const defaultDeepLConfig: DeepLConfig = {
	api_key: '',
	target_lang: 'JA'
}

export const defaultVoiceConfig: VoiceConfig = {
	stt: null,
	tts: null,
	lipsync_fps: 20,
	translation: null
}

export const defaultNodeConfig: WaifuNodeConfig = {
	enabled: true,
	path: '/node',
	token: '',
	invoke_timeout_ms: 10000,
	ping_interval_s: 30
}

export function withDefaults<T extends object>(
	defaults: T,
	value: Partial<T> | null | undefined
): T {
	return { ...defaults, ...(value ?? {}) }
}

/** 取出 flatten 後 `prefix.*` 的直接子欄位；沒有任何欄位時回傳 null */
export function pickSection(
	config: Partial<WaifuConfig>,
	prefix: string
): Record<string, unknown> | null {
	const section: Record<string, unknown> = {}
	const head = `${prefix}.`
	for (const [key, value] of Object.entries(config)) {
		if (!key.startsWith(head)) continue
		const leaf = key.slice(head.length)
		if (!leaf.includes('.')) section[leaf] = value
	}
	return Object.keys(section).length ? section : null
}

export function configToGateway(config: Partial<WaifuConfig>): GatewayConfig {
	return withDefaults(defaultGatewayConfig, {
		host: config['gateway.host'],
		port: config['gateway.port'],
		session_ttl: config['gateway.session_ttl'],
		lane_max_queue: config['gateway.lane_max_queue']
	})
}

export function gatewayToConfig(gateway: GatewayConfig): ConfigUpdate {
	return {
		'gateway.host': gateway.host,
		'gateway.port': gateway.port,
		'gateway.session_ttl': gateway.session_ttl,
		'gateway.lane_max_queue': gateway.lane_max_queue
	}
}

export function configToContext(config: Partial<WaifuConfig>): ContextConfig {
	return withDefaults(
		defaultContextConfig,
		pickSection(config, 'context') as Partial<ContextConfig> | null
	)
}

export function contextToConfig(context: ContextConfig): ConfigUpdate {
	return {
		'context.builtin_prompt': context.builtin_prompt,
		'context.system_prompt_template': context.system_prompt_template,
		'context.max_history_messages': context.max_history_messages,
		'context.history_strategy': context.history_strategy,
		'context.context_token_budget': context.context_token_budget,
		'context.compact_threshold': context.compact_threshold
	}
}

export function configToHooks(config: Partial<WaifuConfig>): HooksConfig {
	return withDefaults(
		defaultHooksConfig,
		pickSection(config, 'hooks') as Partial<HooksConfig> | null
	)
}

export function hooksToConfig(hooks: HooksConfig): ConfigUpdate {
	return {
		'hooks.emotion': hooks.emotion,
		'hooks.tts': hooks.tts,
		'hooks.logging': hooks.logging,
		'hooks.rate_limit_rpm': hooks.rate_limit_rpm
	}
}

export function configToProvider(
	config: Partial<WaifuConfig>,
	prefix: 'provider' | 'utility_provider'
): ProviderConfig {
	return withDefaults(
		defaultProviderConfig,
		pickSection(config, prefix) as Partial<ProviderConfig> | null
	)
}

/** 主要 provider 逐欄寫入，model / temperature / max_tokens 可即時生效 */
export function providerToConfig(provider: ProviderConfig): ConfigUpdate {
	return {
		'provider.provider': provider.provider,
		'provider.api_key': provider.api_key,
		'provider.base_url': provider.base_url,
		'provider.model': provider.model,
		'provider.temperature': provider.temperature,
		'provider.max_tokens': provider.max_tokens
	}
}

/** 輔助 provider 以子樹整批寫入（逐欄寫入時中途狀態會驗證失敗），null 代表停用 */
export function utilityProviderToConfig(
	provider: ProviderConfig | null
): ConfigUpdate {
	return { utility_provider: provider }
}

export function configToVoice(config: Partial<WaifuConfig>): VoiceConfig {
	const stt = pickSection(config, 'voice.stt')
	const tts = pickSection(config, 'voice.tts')
	const translation = pickSection(config, 'voice.translation')
	const backend: TTSBackend = tts?.backend === 'gptsovits' ? 'gptsovits' : 'openai'

	return {
		stt: stt ? withDefaults(defaultSTTConfig, stt as Partial<STTConfig>) : null,
		tts: tts
			? ({ ...defaultTTSConfigs[backend], ...tts, backend } as TTSConfig)
			: null,
		lipsync_fps:
			(config['voice.lipsync_fps'] as number | null | undefined) ??
			defaultVoiceConfig.lipsync_fps,
		translation: translation
			? withDefaults(defaultDeepLConfig, translation as Partial<DeepLConfig>)
			: null
	}
}

/** stt / tts / translation 以子樹整批寫入，null 代表停用 */
export function voiceToConfig(voice: VoiceConfig): ConfigUpdate {
	return {
		'voice.lipsync_fps': voice.lipsync_fps,
		'voice.stt': voice.stt ?? null,
		'voice.tts': voice.tts ?? null,
		'voice.translation': voice.translation ?? null
	}
}

export function configToNode(config: Partial<WaifuConfig>): WaifuNodeConfig {
	return withDefaults(defaultNodeConfig, {
		enabled: config['node.enabled'] ?? undefined,
		path: config['node.path'] ?? undefined,
		token: config['node.token'] ?? undefined,
		invoke_timeout_ms: config['node.invoke_timeout_ms'] ?? undefined,
		ping_interval_s: config['node.ping_interval_s'] ?? undefined
	})
}

export function nodeToConfig(node: WaifuNodeConfig): ConfigUpdate {
	return {
		'node.enabled': node.enabled,
		'node.path': node.path,
		'node.token': node.token,
		'node.invoke_timeout_ms': node.invoke_timeout_ms,
		'node.ping_interval_s': node.ping_interval_s
	}
}

export function nullableString(
	value: string | null | undefined
): string | null {
	const trimmed = value?.trim() ?? ''
	return trimmed ? trimmed : null
}

export function requiredString(value: string | null | undefined): string {
	return value?.trim() ?? ''
}

export function nullableNumber(
	value: string | number | null | undefined
): number | null {
	if (value === null || value === undefined || value === '') return null
	const numberValue = Number(value)
	return Number.isFinite(numberValue) ? numberValue : null
}

export function numberValue(
	value: string | number | null | undefined,
	fallback = 0
): number {
	if (value === null || value === undefined || value === '') return fallback
	const parsed = Number(value)
	return Number.isFinite(parsed) ? parsed : fallback
}
