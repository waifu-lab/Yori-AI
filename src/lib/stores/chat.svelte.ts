import type { UnlistenFn } from '@tauri-apps/api/event';
import { onChunk, onDone, onError, onEvent, onTranscript } from '$lib/backend/ws';
import type { ChatEntry, MessageEntry, ToolEntry } from '$lib/backend/types';

export type { ChatEntry, MessageEntry, ToolEntry };

class ChatStore {
	messages = $state<MessageEntry[]>([]);

	private unlisteners: UnlistenFn[] = [];

	async init() {
		const [unTranscript, unChunk, unDone, unErr, unEvt] = await Promise.all([
			onTranscript((msg) => {
				this.addUserMessage(msg.request_id, msg.content);
			}),

			onChunk((msg) => {
				const aid = this.currentAsstId(msg.request_id);
				const idx = this.messages.findIndex((m) => m.id === aid);
				if (idx !== -1) {
					const entry = this.messages[idx] as ChatEntry;
					this.messages[idx] = { ...entry, content: entry.content + msg.content };
				} else {
					this.messages.push({ id: aid, role: 'assistant', content: msg.content, pending: true });
				}
			}),

			onDone((msg) => {
				const aid = this.currentAsstId(msg.request_id);
				const idx = this.messages.findIndex((m) => m.id === aid);
				if (idx !== -1) {
					const entry = this.messages[idx] as ChatEntry;
					this.messages[idx] = { ...entry, content: msg.content, pending: false };
				} else {
					this.messages.push({ id: aid, role: 'assistant', content: msg.content, pending: false });
				}
				this.asstSegment.delete(msg.request_id);
			}),

			onError((msg) => {
				console.error('[backend]', msg.code, msg.message);
			}),

			onEvent((msg) => {
				if (msg.event === 'tool_call') {
					const data = msg.data as { name: string; arguments: string; id?: string };
					this.upsertToolCall(data);
				} else if (msg.event === 'tool_result') {
					const data = msg.data as { name?: string; id?: string; result?: unknown; error?: string };
					this.resolveToolResult(data);
				}
			})
		]);

		this.unlisteners = [unTranscript, unChunk, unDone, unErr, unEvt];
	}

	destroy() {
		for (const fn of this.unlisteners) fn();
		this.unlisteners = [];
		this.asstSegment.clear();
	}

	addUserMessage(requestId: string, content: string, attachments?: ChatEntry['attachments']) {
		this.messages.push({ id: requestId, role: 'user', content, pending: false, attachments });
	}

	private asstSegment = new Map<string, number>();

	private currentAsstId(requestId: string) {
		const seg = this.asstSegment.get(requestId) ?? 0;
		return `${requestId}-a${seg}`;
	}

	private bumpAsstSegment(requestId: string) {
		const seg = (this.asstSegment.get(requestId) ?? 0) + 1;
		this.asstSegment.set(requestId, seg);
	}

	private upsertToolCall(data: { name: string; arguments: string; id?: string }) {
		// Mark any pending assistant segment as complete (so next chunks open a new one).
		for (const [reqId, seg] of this.asstSegment) {
			const aid = `${reqId}-a${seg}`;
			const idx = this.messages.findIndex((m) => m.id === aid);
			if (idx !== -1) {
				const entry = this.messages[idx] as ChatEntry;
				if (entry.content.length === 0) {
					this.messages.splice(idx, 1);
				} else {
					this.messages[idx] = { ...entry, pending: false };
				}
			}
			this.bumpAsstSegment(reqId);
		}
		const id = data.id ?? `tool-${crypto.randomUUID()}`;
		const entry: ToolEntry = {
			id,
			role: 'tool',
			name: data.name,
			status: 'running',
			args: data.arguments,
			pending: true
		};
		this.messages.push(entry);
	}

	private resolveToolResult(data: { name?: string; id?: string; result?: unknown; error?: string }) {
		let idx = -1;
		if (data.id) {
			idx = this.messages.findIndex((m) => m.role === 'tool' && m.id === data.id);
		}
		if (idx === -1) {
			for (let i = this.messages.length - 1; i >= 0; i--) {
				const m = this.messages[i];
				if (m.role === 'tool' && m.pending && (!data.name || m.name === data.name)) {
					idx = i;
					break;
				}
			}
		}
		if (idx === -1) {
			this.messages.push({
				id: data.id ?? `tool-${crypto.randomUUID()}`,
				role: 'tool',
				name: data.name ?? 'tool',
				status: data.error ? 'error' : 'success',
				result: data.result,
				error: data.error,
				pending: false
			});
			return;
		}
		const prev = this.messages[idx] as ToolEntry;
		this.messages[idx] = {
			...prev,
			status: data.error ? 'error' : 'success',
			result: data.result,
			error: data.error,
			pending: false
		};
	}
}

export const chatStore = new ChatStore();
