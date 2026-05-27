import { getConversationMessages, listConversations, resolveAttachmentUrl, streamChat, type ConversationItem } from '$lib/backend/http';
import type { Attachment, ChatEntry, MessageEntry, ToolEntry } from '$lib/backend/types';

function normalizeAttachments(raw: unknown): Attachment[] | undefined {
	if (!Array.isArray(raw) || raw.length === 0) return undefined;
	return raw.map((a) => {
		const att = a as Attachment;
		return { ...att, url: resolveAttachmentUrl(att.url) };
	});
}

export type { ChatEntry, ConversationItem, MessageEntry, ToolEntry };

class ChatHttpStore {
	messages = $state<MessageEntry[]>([]);
	sending = $state(false);

	private upsertToolCall(data: { name: string; arguments: string; id?: string }) {
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
			const entry: ToolEntry = {
				id: data.id ?? `tool-${crypto.randomUUID()}`,
				role: 'tool',
				name: data.name ?? 'tool',
				status: data.error ? 'error' : 'success',
				result: data.result,
				error: data.error,
				pending: false
			};
			this.messages.push(entry);
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

	conversations = $state<ConversationItem[]>([]);
	conversationsLoading = $state(false);
	activeConversationId = $state<string | null>(null);

	async loadConversations(q?: string) {
		this.conversationsLoading = true;
		try {
			this.conversations = await listConversations(50, q);
		} catch (e) {
			console.error('[http] loadConversations:', e);
		} finally {
			this.conversationsLoading = false;
		}
	}

	async selectConversation(id: string) {
		if (this.activeConversationId === id) return;
		this.activeConversationId = id;
		this.messages = [];
		try {
			const history = await getConversationMessages(id);
			this.messages = history.map<MessageEntry>((m) => {
				const parts = m.parts as Array<{ type: string; text?: string }> | undefined;
				const content =
					parts
						?.filter((p) => p.type === 'text')
						.map((p) => p.text ?? '')
						.join('') ??
					(m.content as string | undefined) ??
					'';
				return {
					id: m.id,
					role: m.role,
					content,
					pending: false,
					attachments: normalizeAttachments(m.attachments)
				};
			});
		} catch (e) {
			console.error('[http] loadMessages:', e);
		}
	}

	newConversation() {
		this.activeConversationId = crypto.randomUUID();
		this.messages = [];
	}

	updateConversationTitle(conversationId: string, title: string) {
		const idx = this.conversations.findIndex((c) => c.id === conversationId);
		if (idx !== -1) {
			this.conversations[idx] = { ...this.conversations[idx], title };
		}
	}

	async send(content: string, attachments?: Attachment[]) {
		if (this.sending) return;

		if (!this.activeConversationId) {
			this.activeConversationId = crypto.randomUUID();
		}
		const convId = this.activeConversationId;

		this.sending = true;

		const userMsgId = crypto.randomUUID();
		this.messages.push({
			id: userMsgId,
			role: 'user',
			content,
			pending: false,
			attachments: attachments?.length
				? attachments.map((a) => ({
						...a,
						data: undefined,
						url: a.url ? resolveAttachmentUrl(a.url) : a.data ? `data:${a.content_type};base64,${a.data}` : undefined
					}))
				: undefined
		});

		let currentAsstId: string | null = crypto.randomUUID();
		this.messages.push({ id: currentAsstId, role: 'assistant', content: '', pending: true });

		const finalizeCurrentAsst = () => {
			if (!currentAsstId) return;
			const idx = this.messages.findIndex((m) => m.id === currentAsstId);
			if (idx !== -1) {
				const entry = this.messages[idx] as ChatEntry;
				if (entry.content.length === 0) {
					this.messages.splice(idx, 1);
				} else {
					this.messages[idx] = { ...entry, pending: false };
				}
			}
			currentAsstId = null;
		};

		const ensureCurrentAsst = () => {
			if (currentAsstId) return currentAsstId;
			currentAsstId = crypto.randomUUID();
			this.messages.push({ id: currentAsstId, role: 'assistant', content: '', pending: true });
			return currentAsstId;
		};

		await streamChat(content, convId, {
			onChunk: (chunk) => {
				const id = ensureCurrentAsst();
				const idx = this.messages.findIndex((m) => m.id === id);
				if (idx !== -1) {
					const entry = this.messages[idx] as ChatEntry;
					this.messages[idx] = { ...entry, content: entry.content + chunk };
				}
			},
			onDone: (full) => {
				if (currentAsstId) {
					const idx = this.messages.findIndex((m) => m.id === currentAsstId);
					if (idx !== -1) {
						const entry = this.messages[idx] as ChatEntry;
						this.messages[idx] = { ...entry, content: full || entry.content, pending: false };
					}
				} else if (full) {
					this.messages.push({ id: crypto.randomUUID(), role: 'assistant', content: full, pending: false });
				}
				currentAsstId = null;
				this.sending = false;
				this.loadConversations();
			},
			onError: (code, message) => {
				console.error('[http-chat]', code, message);
				if (currentAsstId) {
					const idx = this.messages.findIndex((m) => m.id === currentAsstId);
					if (idx !== -1) {
						this.messages[idx] = { ...this.messages[idx], pending: false } as ChatEntry;
					}
				}
				currentAsstId = null;
				this.sending = false;
			},
			onEvent: (evt) => {
				if (evt.event === 'tool_call') {
					finalizeCurrentAsst();
					this.upsertToolCall(evt.data);
				} else if (evt.event === 'tool_result') {
					this.resolveToolResult(evt.data);
				} else if (evt.event === 'title_update') {
					this.updateConversationTitle(evt.data.conversation_id, evt.data.title);
				}
			}
		}, attachments);
	}
}

export const chatHttpStore = new ChatHttpStore();

