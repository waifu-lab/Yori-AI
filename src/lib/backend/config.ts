import type {
	ContextConfig,
	GatewayConfig,
	ProviderConfig,
	STTConfig,
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
	system_prompt_template: 'You are a helpful assistant.',
	max_history_messages: 20,
	history_strategy: 'sliding',
	soul_content: null,
	context_token_budget: null,
	compact_threshold: 0.85
}

export const defaultProviderConfig: ProviderConfig = {
	provider: '',
	api_key: '',
	base_url: null,
	model: '',
	temperature: 0.7,
	max_tokens: null,
	stream: true
}

export const defaultSTTConfig: STTConfig = {
	api_key: '',
	base_url: 'https://api.openai.com/v1/audio/transcriptions',
	model: 'whisper-1',
	language: null,
	temperature: 0.0
}

export const defaultTTSConfig: TTSConfig = {
	api_key: '',
	base_url: 'https://api.openai.com/v1/audio/speech',
	model: 'tts-1',
	voice: 'nova'
}

export const defaultVoiceConfig: VoiceConfig = {
	stt: null,
	tts: null,
	lipsync_fps: 20
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

export function configToGateway(config: Partial<WaifuConfig>): GatewayConfig {
	return withDefaults(defaultGatewayConfig, {
		host: config['gateway.host'],
		port: config['gateway.port'],
		session_ttl: config['gateway.session_ttl'],
		lane_max_queue: config['gateway.lane_max_queue']
	})
}

export function gatewayToConfig(gateway: GatewayConfig): Partial<WaifuConfig> {
	return {
		'gateway.host': gateway.host,
		'gateway.port': gateway.port,
		'gateway.session_ttl': gateway.session_ttl,
		'gateway.lane_max_queue': gateway.lane_max_queue
	}
}

export function configToContext(config: Partial<WaifuConfig>): ContextConfig {
	return withDefaults(defaultContextConfig, {
		system_prompt_template: config['context.system_prompt_template'],
		max_history_messages: config['context.max_history_messages'],
		history_strategy: config['context.history_strategy'],
		soul_content: config['context.soul_content'],
		context_token_budget: config['context.context_token_budget'],
		compact_threshold: config['context.compact_threshold']
	})
}

export function contextToConfig(context: ContextConfig): Partial<WaifuConfig> {
	return {
		'context.system_prompt_template': context.system_prompt_template,
		'context.max_history_messages': context.max_history_messages,
		'context.history_strategy': context.history_strategy,
		'context.soul_content': context.soul_content,
		'context.context_token_budget': context.context_token_budget,
		'context.compact_threshold': context.compact_threshold
	}
}

export function configToProvider(
	config: Partial<WaifuConfig>,
	prefix: 'provider' | 'utility_provider'
): ProviderConfig {
	return withDefaults(defaultProviderConfig, {
		provider: config[`${prefix}.provider`] ?? undefined,
		api_key: config[`${prefix}.api_key`] ?? undefined,
		base_url: config[`${prefix}.base_url`],
		model: config[`${prefix}.model`] ?? undefined,
		temperature: config[`${prefix}.temperature`] ?? undefined,
		max_tokens: config[`${prefix}.max_tokens`],
		stream: prefix === 'provider' ? (config['provider.stream'] ?? undefined) : true
	})
}

export function providerToConfig(
	provider: ProviderConfig,
	prefix: 'provider' | 'utility_provider'
): Partial<WaifuConfig> {
	return {
		[`${prefix}.provider`]: provider.provider,
		[`${prefix}.api_key`]: provider.api_key,
		[`${prefix}.base_url`]: provider.base_url,
		[`${prefix}.model`]: provider.model,
		[`${prefix}.temperature`]: provider.temperature,
		[`${prefix}.max_tokens`]: provider.max_tokens,
		...(prefix === 'provider' ? { 'provider.stream': provider.stream } : {})
	} as Partial<WaifuConfig>
}

export function nullProviderConfig(
	prefix: 'utility_provider'
): Partial<WaifuConfig> {
	return {
		[`${prefix}.provider`]: null,
		[`${prefix}.api_key`]: null,
		[`${prefix}.base_url`]: null,
		[`${prefix}.model`]: null,
		[`${prefix}.temperature`]: null,
		[`${prefix}.max_tokens`]: null
	} as Partial<WaifuConfig>
}

export function configToVoice(config: Partial<WaifuConfig>): VoiceConfig {
	const hasStt =
		config['voice.stt.api_key'] != null ||
		config['voice.stt.base_url'] != null ||
		config['voice.stt.model'] != null
	const hasTts =
		config['voice.tts.api_key'] != null ||
		config['voice.tts.base_url'] != null ||
		config['voice.tts.model'] != null

	return {
		stt: hasStt
			? withDefaults(defaultSTTConfig, {
					api_key: config['voice.stt.api_key'] ?? undefined,
					base_url: config['voice.stt.base_url'] ?? undefined,
					model: config['voice.stt.model'] ?? undefined,
					language: config['voice.stt.language'],
					temperature: config['voice.stt.temperature'] ?? undefined
				})
			: null,
		tts: hasTts
			? withDefaults(defaultTTSConfig, {
					api_key: config['voice.tts.api_key'] ?? undefined,
					base_url: config['voice.tts.base_url'] ?? undefined,
					model: config['voice.tts.model'] ?? undefined,
					voice: config['voice.tts.voice'] ?? undefined
				})
			: null,
		lipsync_fps: config['voice.lipsync_fps'] ?? defaultVoiceConfig.lipsync_fps
	}
}

export function voiceToConfig(voice: VoiceConfig): Partial<WaifuConfig> {
	return {
		'voice.stt.api_key': voice.stt?.api_key ?? null,
		'voice.stt.base_url': voice.stt?.base_url ?? null,
		'voice.stt.model': voice.stt?.model ?? null,
		'voice.stt.language': voice.stt?.language ?? null,
		'voice.stt.temperature': voice.stt?.temperature ?? null,
		'voice.tts.api_key': voice.tts?.api_key ?? null,
		'voice.tts.base_url': voice.tts?.base_url ?? null,
		'voice.tts.model': voice.tts?.model ?? null,
		'voice.tts.voice': voice.tts?.voice ?? null,
		'voice.lipsync_fps': voice.lipsync_fps
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

export function nodeToConfig(node: WaifuNodeConfig): Partial<WaifuConfig> {
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
