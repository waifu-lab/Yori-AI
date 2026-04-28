export interface ConnectedMessage {
	type: 'connected';
	session_token: string;
	emotions?: string[];
}

export interface ChatMessage {
	type: 'chat';
	conversation_id?: string;
	request_id?: string;
	content: string;
	live2d?: boolean;
}

export interface VoiceMessage {
	type: 'voice';
	conversation_id?: string;
	request_id?: string;
	audio: string; // base64
	format: 'webm';
}

export type ClientMessage = ChatMessage | VoiceMessage;

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
	data: { audio: string; format: string }; // audio is base64
}

export interface LipsyncEventMessage {
	type: 'event';
	event: 'lipsync';
	data: { visemes: Viseme[] };
}

export type EventMessage = EmotionEventMessage | AudioEventMessage | LipsyncEventMessage;

export interface ErrorMessage {
	type: 'error';
	request_id?: string;
	code: string;
	message: string;
}

export type ServerMessage = TranscriptMessage | ChunkMessage | DoneMessage | EventMessage | ErrorMessage;

export interface ChatEntry {
	id: string;
	role: 'user' | 'assistant';
	content: string;
	pending: boolean;
}
