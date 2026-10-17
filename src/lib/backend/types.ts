export type HistoryStrategy = 'sliding' | 'summary' | 'full';

export interface GatewayConfig {
	host: string;
	port: number;
	session_ttl: number;
	lane_max_queue: number;
}

export interface ContextConfig {
	builtin_prompt: boolean;
	system_prompt_template: string;
	max_history_messages: number;
	history_strategy: HistoryStrategy;
	context_token_budget?: number | null;
	compact_threshold: number;
}

export interface HooksConfig {
	emotion: boolean;
	tts: boolean;
	logging: boolean;
	rate_limit_rpm?: number | null;
}

export interface ProviderConfig {
	provider: string;
	api_key: string;
	base_url?: string | null;
	model: string;
	temperature: number;
	max_tokens?: number | null;
}

export interface STTConfig {
	api_key: string;
	base_url: string;
	model: string;
	language?: string | null;
	temperature: number;
}

export type TTSBackend = 'openai' | 'gptsovits';

interface TTSBackendConfig {
	timeout_retries: number;
	retry_backoff_s: number;
}

export interface OpenAITTSConfig extends TTSBackendConfig {
	backend: 'openai';
	api_key: string;
	base_url: string;
	model: string;
	voice: string;
	response_format: string;
	speed: number;
}

export interface GPTSoVITSTTSConfig extends TTSBackendConfig {
	backend: 'gptsovits';
	base_url: string;
	ref_audio_path: string;
	prompt_text: string;
	prompt_lang: string;
	text_lang: string;
	text_split_method: string;
	batch_size: number;
	speed_factor: number;
	streaming_mode: number; // 0=停用; 1=最佳品質; 2=中; 3=最快
	top_k: number;
	top_p: number;
	temperature: number;
	timeout: number;
}

export type TTSConfig = OpenAITTSConfig | GPTSoVITSTTSConfig;

export interface DeepLConfig {
	api_key: string;
	target_lang: string;
}

export interface VoiceConfig {
	stt?: STTConfig | null;
	tts?: TTSConfig | null;
	lipsync_fps: number;
	translation?: DeepLConfig | null;
}

export interface WaifuNodeConfig {
	enabled: boolean;
	path: string;
	token: string;
	invoke_timeout_ms: number;
	ping_interval_s: number;
}

export interface WaifuConfig {
	'gateway.host': string;
	'gateway.port': number;
	'gateway.session_ttl': number;
	'gateway.lane_max_queue': number;
	'context.builtin_prompt'?: boolean;
	'context.system_prompt_template': string;
	'context.max_history_messages': number;
	'context.history_strategy': HistoryStrategy;
	'context.context_token_budget'?: number | null;
	'context.compact_threshold': number;
	'provider.provider': string;
	'provider.api_key': string;
	'provider.base_url'?: string | null;
	'provider.model': string;
	'provider.temperature': number;
	'provider.max_tokens'?: number | null;
	'utility_provider.provider'?: string | null;
	'utility_provider.api_key'?: string | null;
	'utility_provider.base_url'?: string | null;
	'utility_provider.model'?: string | null;
	'utility_provider.temperature'?: number | null;
	'utility_provider.max_tokens'?: number | null;
	'voice.lipsync_fps'?: number | null;
	'hooks.emotion'?: boolean;
	'hooks.tts'?: boolean;
	'hooks.logging'?: boolean;
	'hooks.rate_limit_rpm'?: number | null;
	'node.enabled'?: boolean | null;
	'node.path'?: string | null;
	'node.token'?: string | null;
	'node.invoke_timeout_ms'?: number | null;
	'node.ping_interval_s'?: number | null;
	// 巢狀設定（voice.stt.* 等）以 flatten 後的 key 回傳
	[key: string]: unknown;
}

/**
 * PUT /config 的寫入內容：key 可為葉節點或子樹（如 'voice.tts'），
 * 子樹以物件整批取代、以 null 移除；後端每個 key 都會驗證整份設定
 */
export type ConfigUpdate = Record<string, unknown>;

export interface ConfigSetResult {
	key: string;
	status: string;
	path?: string;
	note?: string;
}

export interface ConnectedMessage {
	type: 'connected';
	session_token: string;
}

export interface Attachment {
	id?: string;
	filename: string;
	content_type: string;
	data?: string; // base64 (outgoing only)
	url?: string;
	size_bytes?: number;
}

/** IncomingRequest.features 的已知旗標 */
export type Feature = 'live2d' | 'lipsync';

export interface ChatMessage {
	type: 'chat';
	conversation_id?: string;
	request_id?: string;
	content: string;
	attachments?: Attachment[];
	features?: Feature[];
}

export interface VoiceMessage {
	type: 'voice';
	conversation_id?: string;
	request_id?: string;
	audio: string; // base64
	format: 'webm';
}

export type ClientMessage = ChatMessage | VoiceMessage | Live2dSyncMessage | StopMessage;

export interface TranscriptMessage {
	type: 'transcript';
	request_id: string;
	content: string;
}

export interface ChunkMessage {
	type: 'chunk';
	request_id: string;
	content: string;
}

export interface DoneMessage {
	type: 'done';
	request_id: string;
	content: string;
}

export interface Viseme {
	time_ms: number;
	value: number; // 0–1 mouth open amount
}

export interface EmotionEventMessage {
	type: 'event';
	event: 'emotion';
	data: { emotion: string };
}

export interface AudioEventMessage {
	type: 'event';
	event: 'audio';
	data: {
		audio: string;
		format: string;
		duration_ms?: number;
		emotion?: string;
	};
}

export interface Live2dSyncMessage {
	type: 'live2dsync';
	expressions: string[];
}

export interface Live2dSyncAckMessage {
	type: 'live2dsync_ack';
	count: number;
}

export interface LipsyncEventMessage {
	type: 'event';
	event: 'lipsync';
	data: { visemes: Viseme[] };
}

export interface ToolCallEventMessage {
	type: 'event';
	event: 'tool_call';
	data: { name: string; arguments: string; id?: string };
}

export interface ToolResultEventMessage {
	type: 'event';
	event: 'tool_result';
	data: { name?: string; id?: string; result?: unknown; error?: string };
}

export interface ToolErrorEventMessage {
	type: 'event';
	event: 'tool_error';
	data: { name: string; error: string };
}

/** 串流中斷後重跑該輪：從目前輸出尾端丟棄 discard 個字元 */
export interface StreamResetEventMessage {
	type: 'event';
	event: 'stream_reset';
	data: { discard: number };
}

export interface DialogueEventMessage {
	type: 'event';
	event: 'dialogue';
	data: { name?: string; text: string };
}

export interface TitleUpdateEventMessage {
	type: 'event';
	event: 'title_update';
	data: { conversation_id: string; title: string };
}

export type EventMessage =
	| EmotionEventMessage
	| AudioEventMessage
	| LipsyncEventMessage
	| ToolCallEventMessage
	| ToolResultEventMessage
	| ToolErrorEventMessage
	| StreamResetEventMessage
	| DialogueEventMessage
	| TitleUpdateEventMessage;

export interface ErrorMessage {
	type: 'error';
	request_id?: string;
	code: string;
	message: string;
}

export interface StopMessage {
	type: 'stop';
	conversation_id?: string;
}

export interface StoppedMessage {
	type: 'stopped';
	request_id: string | null;
}

export type ServerMessage =
	| TranscriptMessage
	| ChunkMessage
	| DoneMessage
	| EventMessage
	| ErrorMessage
	| StoppedMessage;

export interface ChatEntry {
	id: string;
	role: 'user' | 'assistant';
	content: string;
	pending: boolean;
	attachments?: Attachment[];
}

export interface ToolEntry {
	id: string;
	role: 'tool';
	name: string;
	status: 'running' | 'success' | 'error';
	args?: string;
	result?: unknown;
	error?: string;
	pending: boolean;
}

export type MessageEntry = ChatEntry | ToolEntry;
