import { invoke } from '@tauri-apps/api/core';
import axios from 'axios';

import type { Attachment } from './types';
export type { Attachment };

function toHttpBase(wsUrl: string): string {
	const trimmed = wsUrl.replace(/\/ws\/?$/, '').replace(/\/$/, '');
	return trimmed.startsWith('wss://') ? trimmed.replace('wss://', 'https://') : trimmed.replace('ws://', 'http://');
}

let _httpBase: string | null = null;
async function getHttpBase(): Promise<string> {
	if (!_httpBase) {
		const wsUrl = await invoke<string>('backend_get_url');
		_httpBase = toHttpBase(wsUrl);
	}
	return _httpBase;
}

export function resolveAttachmentUrl(url: string | undefined): string {
	if (!url) return '';
	if (/^(?:https?:|data:|blob:)/i.test(url)) return url;
	if (!_httpBase) return url;
	return url.startsWith('/') ? `${_httpBase}${url}` : `${_httpBase}/${url}`;
}

export interface ConversationItem {
	id: string;
	title?: string;
	created_at?: string;
	updated_at?: string;
	message_count?: number;
	[key: string]: unknown;
}

export interface HistoryMessage {
	id: string;
	role: 'user' | 'assistant';
	content: string;
	created_at?: string;
	[key: string]: unknown;
}

export interface StreamChatCallbacks {
	onChunk: (content: string) => void;
	onDone: (content: string, conversationId?: string) => void;
	onError: (code: string, message: string) => void;
}

export async function listConversations(limit = 20, q?: string): Promise<ConversationItem[]> {
	const base = await getHttpBase();
	const params = new URLSearchParams({ limit: String(limit) });
	if (q) params.set('q', q);
	const res = await fetch(`${base}/conversations/?${params}`);
	if (!res.ok) throw new Error(`HTTP ${res.status}`);
	return res.json();
}

export async function getConversationMessages(id: string, limit = 100): Promise<HistoryMessage[]> {
	const base = await getHttpBase();
	const res = await fetch(`${base}/conversations/${id}/messages?limit=${limit}`);
	if (!res.ok) throw new Error(`HTTP ${res.status}`);
	return res.json();
}

function parseSseLines(
	text: string,
	buf: { value: string },
	accumulatedContent: { value: string },
	doneSignalled: { value: boolean },
	callbacks: StreamChatCallbacks
) {
	buf.value += text;

	const lines = buf.value.split('\n');
	buf.value = lines.pop() ?? '';

	for (const raw of lines) {
		const line = raw.replace(/\r$/, '');
		if (!line.startsWith('data: ')) continue;
		const data = line.slice(6).trim();
		if (!data) continue;

		if (data === '[DONE]') {
			if (!doneSignalled.value) {
				doneSignalled.value = true;
				callbacks.onDone(accumulatedContent.value);
			}
			continue;
		}

		try {
			const evt = JSON.parse(data) as Record<string, unknown>;
			const type = evt.type as string | undefined;
			const evtContent = (evt.content as string) ?? '';
			const convId = evt.conversation_id as string | undefined;

			if (type === 'chunk' || (!type && evtContent)) {
				accumulatedContent.value += evtContent;
				callbacks.onChunk(evtContent);
			} else if (type === 'done') {
				doneSignalled.value = true;
				callbacks.onDone(evtContent || accumulatedContent.value, convId);
			} else if (type === 'error') {
				callbacks.onError((evt.code as string) ?? 'error', (evt.message as string) ?? '');
			}
		} catch {
			// ignore malformed SSE lines
		}
	}
}

export async function streamChat(
	content: string,
	conversationId: string,
	callbacks: StreamChatCallbacks,
	attachments?: Attachment[]
): Promise<void> {
	const base = await getHttpBase();
	const buf = { value: '' };
	const accumulatedContent = { value: '' };
	const doneSignalled = { value: false };
	let processedLength = 0;

	try {
		await axios.post(
			`${base}/chat/`,
			{ conversation_id: conversationId, content, ...(attachments?.length ? { attachments } : {}) },
			{
				responseType: 'text',
				headers: { 'Content-Type': 'application/json', Accept: 'text/event-stream' },
				onDownloadProgress: (progressEvent) => {
					const responseText = (progressEvent.event?.target as XMLHttpRequest | undefined)?.responseText;
					if (typeof responseText !== 'string') return;

					const newText = responseText.slice(processedLength);
					processedLength = responseText.length;
					parseSseLines(newText, buf, accumulatedContent, doneSignalled, callbacks);
				}
			}
		);
	} catch (e: unknown) {
		if (!doneSignalled.value) {
			callbacks.onError('request_error', String(e));
			return;
		}
	}

	if (!doneSignalled.value) {
		callbacks.onDone(accumulatedContent.value);
	}
}

export async function getStatus(): Promise<Record<string, unknown>> {
	const base = await getHttpBase();
	const res = await fetch(`${base}/status`);
	if (!res.ok) throw new Error(`HTTP ${res.status}`);
	return res.json();
}

export async function getAllConfig(): Promise<Record<string, unknown>> {
	const base = await getHttpBase();
	const res = await fetch(`${base}/config/`);
	if (!res.ok) throw new Error(`HTTP ${res.status}`);
	return res.json();
}

export async function batchUpdateConfig(updates: Record<string, unknown>): Promise<Record<string, unknown>> {
	const base = await getHttpBase();
	const res = await fetch(`${base}/config/`, {
		method: 'PUT',
		headers: { 'Content-Type': 'application/json' },
		body: JSON.stringify(updates)
	});
	if (!res.ok) throw new Error(`HTTP ${res.status}`);
	return res.json();
}

export async function getConfig(key: string): Promise<Record<string, unknown>> {
	const base = await getHttpBase();
	const res = await fetch(`${base}/config/${encodeURIComponent(key)}`);
	if (!res.ok) throw new Error(`HTTP ${res.status}`);
	return res.json();
}

export async function setConfig(key: string, value: Record<string, unknown>): Promise<Record<string, unknown>> {
	const base = await getHttpBase();
	const res = await fetch(`${base}/config/${encodeURIComponent(key)}`, {
		method: 'PUT',
		headers: { 'Content-Type': 'application/json' },
		body: JSON.stringify(value)
	});
	if (!res.ok) throw new Error(`HTTP ${res.status}`);
	return res.json();
}
