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

export interface EventMessage {
	type: 'event';
	[key: string]: unknown;
}

export interface ErrorMessage {
	type: 'error';
	request_id?: string;
	code: string;
	message: string;
}

export type ServerMessage = ChunkMessage | DoneMessage | EventMessage | ErrorMessage;

export interface ChatEntry {
	id: string;
	role: 'user' | 'assistant';
	content: string;
	pending: boolean;
}
