import type { UnlistenFn } from '@tauri-apps/api/event';
import { dropTail } from '$lib/backend/http';
import { onChunk, onDone, onError, onEvent, onStopped, onTranscript } from '$lib/backend/ws';
import type { ChatEntry, MessageEntry, ToolEntry } from '$lib/backend/types';

export type { ChatEntry, MessageEntry, ToolEntry };

class ChatStore {
	messages = $state<MessageEntry[]>([]);

	private inflight = $state<string[]>([]);

	private unlisteners: UnlistenFn[] = [];

	get busy() {
		return this.inflight.length > 0;
	}

	async init() {
		const [unTranscript, unChunk, unDone, unErr, unEvt, unStopped] = await Promise.all([
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
				this.inflight = this.inflight.filter((id) => id !== msg.request_id);
			}),

			onStopped((msg) => {
				this.finishInterrupted(msg.request_id !== null);
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
				} else if (msg.event === 'tool_error') {
					this.markToolError(msg.data);
				} else if (msg.event === 'stream_reset') {
					this.dropStreamTail(msg.data.discard);
				}
			})
		]);

		this.unlisteners = [unTranscript, unChunk, unDone, unErr, unEvt, unStopped];
	}

	destroy() {
		for (const fn of this.unlisteners) fn();
		this.unlisteners = [];
		this.asstSegment.clear();
		this.inflight = [];
	}

	addUserMessage(requestId: string, content: string, attachments?: ChatEntry['attachments']) {
		this.messages.push({ id: requestId, role: 'user', content, pending: false, attachments });
		if (!this.inflight.includes(requestId)) this.inflight = [...this.inflight, requestId];
	}

	private finishInterrupted(interrupted: boolean) {
		this.messages = this.messages.map((m) => {
			if (!m.pending) return m;
			if (m.role === 'assistant' && interrupted) {
				return { ...m, content: m.content ? `${m.content}\n\n〔已中斷〕` : '〔已中斷〕', pending: false };
			}
			return { ...m, pending: false };
		});
		this.asstSegment.clear();
		this.inflight = [];
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

	// stream_reset 不帶 request_id；lane 串行處理，故套用到最後一個仍在串流的回覆
	private dropStreamTail(discard: number) {
		for (let i = this.messages.length - 1; i >= 0; i--) {
			const m = this.messages[i];
			if (m.role === 'assistant' && m.pending) {
				this.messages[i] = { ...m, content: dropTail(m.content, discard) };
				return;
			}
		}
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
		const error = data.error ?? prev.error;
		this.messages[idx] = {
			...prev,
			status: error ? 'error' : 'success',
			result: data.result,
			error,
			pending: false
		};
	}

	// tool_error 先於 tool_result 送達；先記下錯誤，由 tool_result 結束該工具
	private markToolError(data: { name: string; error: string }) {
		for (let i = this.messages.length - 1; i >= 0; i--) {
			const m = this.messages[i];
			if (m.role === 'tool' && m.pending && m.name === data.name) {
				this.messages[i] = { ...m, error: data.error };
				return;
			}
		}
	}
}

export const chatStore = new ChatStore();
