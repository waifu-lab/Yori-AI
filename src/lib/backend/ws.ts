import { invoke } from '@tauri-apps/api/core';
import { listen, type UnlistenFn } from '@tauri-apps/api/event';

import type { Attachment, ChunkMessage, ConnectedMessage, DoneMessage, ErrorMessage, EventMessage, Live2dSyncAckMessage, TranscriptMessage } from './types';

export function sendChat(content: string, live2d?: boolean, attachments?: Attachment[]): Promise<string> {
	return invoke<string>('backend_send_chat', { content, live2d, attachments });
}

export function sendVoice(audioB64: string): Promise<string> {
	return invoke<string>('backend_send_voice', { audioB64 });
}

export function onTranscript(cb: (msg: TranscriptMessage) => void): Promise<UnlistenFn> {
	return listen<TranscriptMessage>('chat-transcript', (e) => cb(e.payload));
}

export function onChunk(cb: (msg: ChunkMessage) => void): Promise<UnlistenFn> {
	return listen<ChunkMessage>('chat-chunk', (e) => cb(e.payload));
}

export function onDone(cb: (msg: DoneMessage) => void): Promise<UnlistenFn> {
	return listen<DoneMessage>('chat-done', (e) => cb(e.payload));
}

export function onEvent(cb: (msg: EventMessage) => void): Promise<UnlistenFn> {
	return listen<EventMessage>('backend-event', (e) => cb(e.payload));
}

export function onError(cb: (msg: ErrorMessage) => void): Promise<UnlistenFn> {
	return listen<ErrorMessage>('backend-error', (e) => cb(e.payload));
}

export function onConnected(cb: (msg: ConnectedMessage) => void): Promise<UnlistenFn> {
	return listen<ConnectedMessage>('backend-connected', (e) => cb(e.payload));
}

export function onDisconnected(cb: () => void): Promise<UnlistenFn> {
	return listen('backend-disconnected', () => cb());
}

export function sendLive2dSync(expressions: string[]): Promise<void> {
	return invoke('backend_send_live2d_sync', { expressions });
}

export function sendUserEvent(name: string, data?: Record<string, unknown>): Promise<void> {
	return invoke('backend_send_user_event', { name, data: data ?? null });
}

export function onLive2dSyncAck(cb: (msg: Live2dSyncAckMessage) => void): Promise<UnlistenFn> {
	return listen<Live2dSyncAckMessage>('backend-live2dsync-ack', (e) => cb(e.payload));
}
