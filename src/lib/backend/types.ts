export type HistoryStrategy = 'sliding' | 'summary' | 'full'

export interface GatewayConfig {
	host: string
	port: number
	session_ttl: number
	lane_max_queue: number
}

export interface ContextConfig {
	system_prompt_template: string
	max_history_messages: number
	history_strategy: HistoryStrategy
	soul_content?: string | null
	context_token_budget?: number | null
	compact_threshold: number
}

export interface ProviderConfig {
	provider: string
	api_key: string
	base_url?: string | null
	model: string
	temperature: number
	max_tokens?: number | null
	stream?: boolean
}

export interface STTConfig {
	api_key: string
	base_url: string
	model: string
	language?: string | null
	temperature: number
}

export interface TTSConfig {
	api_key: string
	base_url: string
	model: string
	voice: string
}

export interface VoiceConfig {
	stt?: STTConfig | null
	tts?: TTSConfig | null
	lipsync_fps: number
}

export interface WaifuNodeConfig {
	enabled: boolean
	path: string
	token: string
	invoke_timeout_ms: number
	ping_interval_s: number
}

export interface WaifuConfig {
	'gateway.host': string
	'gateway.port': number
	'gateway.session_ttl': number
	'gateway.lane_max_queue': number
	'context.system_prompt_template': string
	'context.max_history_messages': number
	'context.history_strategy': HistoryStrategy
	'context.soul_content'?: string | null
	'context.context_token_budget'?: number | null
	'context.compact_threshold': number
	'provider.provider': string
	'provider.api_key': string
	'provider.base_url'?: string | null
	'provider.model': string
	'provider.temperature': number
	'provider.max_tokens'?: number | null
	'provider.stream'?: boolean
	'utility_provider.provider'?: string | null
	'utility_provider.api_key'?: string | null
	'utility_provider.base_url'?: string | null
	'utility_provider.model'?: string | null
	'utility_provider.temperature'?: number | null
	'utility_provider.max_tokens'?: number | null
	'voice.stt.api_key'?: string | null
	'voice.stt.base_url'?: string | null
	'voice.stt.model'?: string | null
	'voice.stt.language'?: string | null
	'voice.stt.temperature'?: number | null
	'voice.tts.api_key'?: string | null
	'voice.tts.base_url'?: string | null
	'voice.tts.model'?: string | null
	'voice.tts.voice'?: string | null
	'voice.lipsync_fps'?: number | null
	'node.enabled'?: boolean | null
	'node.path'?: string | null
	'node.token'?: string | null
	'node.invoke_timeout_ms'?: number | null
	'node.ping_interval_s'?: number | null
}

export interface ConnectedMessage {
	type: 'connected'
	session_token: string
}

export interface Attachment {
	id?: string
	filename: string
	content_type: string
	data?: string // base64 (outgoing only)
	url?: string
	size_bytes?: number
}

export interface ChatMessage {
	type: 'chat'
	conversation_id?: string
	request_id?: string
	content: string
	attachments?: Attachment[]
	live2d?: boolean
	need_lipsync?: boolean
}

export interface VoiceMessage {
	type: 'voice'
	conversation_id?: string
	request_id?: string
	audio: string // base64
	format: 'webm'
}

export type ClientMessage = ChatMessage | VoiceMessage | Live2dSyncMessage

export interface TranscriptMessage {
	type: 'transcript'
	request_id: string
	content: string
}

export interface ChunkMessage {
	type: 'chunk'
	request_id: string
	content: string
}

export interface DoneMessage {
	type: 'done'
	request_id: string
	content: string
}

export interface Viseme {
	time_ms: number
	value: number // 0–1 mouth open amount
}

export interface EmotionEventMessage {
	type: 'event'
	event: 'emotion'
	data: { emotion: string }
}

export interface AudioEventMessage {
	type: 'event'
	event: 'audio'
	data: {
		audio: string
		format: string
		duration_ms?: number
		emotion?: string
	}
}

export interface Live2dSyncMessage {
	type: 'live2dsync'
	expressions: string[]
}

export interface Live2dSyncAckMessage {
	type: 'live2dsync_ack'
	count: number
}

export interface LipsyncEventMessage {
	type: 'event'
	event: 'lipsync'
	data: { visemes: Viseme[] }
}

export interface ToolCallEventMessage {
	type: 'event'
	event: 'tool_call'
	data: { name: string; arguments: string; id?: string }
}

export interface ToolResultEventMessage {
	type: 'event'
	event: 'tool_result'
	data: { name?: string; id?: string; result?: unknown; error?: string }
}

export interface DialogueEventMessage {
	type: 'event'
	event: 'dialogue'
	data: { name?: string; text: string }
}

export interface TitleUpdateEventMessage {
	type: 'event'
	event: 'title_update'
	data: { conversation_id: string; title: string }
}

export type EventMessage =
	| EmotionEventMessage
	| AudioEventMessage
	| LipsyncEventMessage
	| ToolCallEventMessage
	| ToolResultEventMessage
	| DialogueEventMessage
	| TitleUpdateEventMessage

export interface ErrorMessage {
	type: 'error'
	request_id?: string
	code: string
	message: string
}

export type ServerMessage =
	| TranscriptMessage
	| ChunkMessage
	| DoneMessage
	| EventMessage
	| ErrorMessage

export interface ChatEntry {
	id: string
	role: 'user' | 'assistant'
	content: string
	pending: boolean
	attachments?: Attachment[]
}

export interface ToolEntry {
	id: string
	role: 'tool'
	name: string
	status: 'running' | 'success' | 'error'
	args?: string
	result?: unknown
	error?: string
	pending: boolean
}

export type MessageEntry = ChatEntry | ToolEntry
