import type { UnlistenFn } from '@tauri-apps/api/event';
import { onChunk, onDone, onError } from '$lib/backend/ws';
import type { ChatEntry } from '$lib/backend/types';

export type { ChatEntry };

class ChatStore {
	messages = $state<ChatEntry[]>([]);

	private unlisteners: UnlistenFn[] = [];

	async init() {
		const [unChunk, unDone, unErr] = await Promise.all([
			onChunk((msg) => {
				const entry = this.messages.find((m) => m.id === msg.request_id);
				if (entry) {
					entry.content += msg.content;
				} else {
					this.messages.push({
						id: msg.request_id,
						role: 'assistant',
						content: msg.content,
						pending: true
					});
				}
			}),

			onDone((msg) => {
				const entry = this.messages.find((m) => m.id === msg.request_id);
				if (entry) {
					entry.content = msg.content;
					entry.pending = false;
				}
			}),

			onError((msg) => {
				console.error('[backend]', msg.code, msg.message);
			})
		]);

		this.unlisteners = [unChunk, unDone, unErr];
	}

	destroy() {
		for (const fn of this.unlisteners) fn();
		this.unlisteners = [];
	}

	addUserMessage(requestId: string, content: string) {
		this.messages.push({ id: requestId, role: 'user', content, pending: false });
	}
}

export const chatStore = new ChatStore();
