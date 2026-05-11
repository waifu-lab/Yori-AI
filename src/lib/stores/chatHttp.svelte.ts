import { getConversationMessages, listConversations, resolveAttachmentUrl, streamChat, type ConversationItem } from '$lib/backend/http';
import type { Attachment, ChatEntry } from '$lib/backend/types';

function normalizeAttachments(raw: unknown): Attachment[] | undefined {
	if (!Array.isArray(raw) || raw.length === 0) return undefined;
	return raw.map((a) => {
		const att = a as Attachment;
		return { ...att, url: resolveAttachmentUrl(att.url) };
	});
}

export type { ChatEntry, ConversationItem };

class ChatHttpStore {
	messages = $state<ChatEntry[]>([]);
	sending = $state(false);

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
			this.messages = history.map((m) => {
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

		const asstMsgId = crypto.randomUUID();
		this.messages.push({ id: asstMsgId, role: 'assistant', content: '', pending: true });

		await streamChat(content, convId, {
			onChunk: (chunk) => {
				const idx = this.messages.findIndex((m) => m.id === asstMsgId);
				if (idx !== -1) {
					const entry = this.messages[idx];
					this.messages[idx] = { ...entry, content: entry.content + chunk };
				}
			},
			onDone: (full) => {
				const idx = this.messages.findIndex((m) => m.id === asstMsgId);
				if (idx !== -1) {
					const entry = this.messages[idx];
					this.messages[idx] = { ...entry, content: full || entry.content, pending: false };
				}
				this.sending = false;
				this.loadConversations();
			},
			onError: (code, message) => {
				console.error('[http-chat]', code, message);
				const idx = this.messages.findIndex((m) => m.id === asstMsgId);
				if (idx !== -1) {
					this.messages[idx] = { ...this.messages[idx], pending: false };
				}
				this.sending = false;
			}
		}, attachments);
	}
}

export const chatHttpStore = new ChatHttpStore();

