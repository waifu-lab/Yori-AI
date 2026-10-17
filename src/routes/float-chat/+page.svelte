<script lang="ts">
	import { onMount, onDestroy } from 'svelte';
	import { emit, listen, type UnlistenFn } from '@tauri-apps/api/event';
	import { FloatingChatLayout, Chat } from '@waifu/ui';
	import { chatStore } from '$lib/stores/chat.svelte';
	import { sendChat, sendStop } from '$lib/backend/ws';
	import { filesToAttachments } from '$lib/backend/attachments';

	let inputValue = $state('');
	let sending = $state(false);

	let speaking = $state(false);
	let unlistenSpeaking: UnlistenFn | null = null;


	function reportVisible(visible: boolean) {
		emit('float-chat-visible', visible).catch(() => {});
	}

	function onVisibilityChange() {
		reportVisible(document.visibilityState === 'visible');
	}

	onMount(async () => {
		await chatStore.init();
		unlistenSpeaking = await listen<boolean>('live2d-speaking', (e) => {
			speaking = e.payload;
		});
		document.addEventListener('visibilitychange', onVisibilityChange);
		onVisibilityChange();
	});
	onDestroy(() => {
		chatStore.destroy();
		unlistenSpeaking?.();
		document.removeEventListener('visibilitychange', onVisibilityChange);
		reportVisible(false);
	});

	function stopReply() {
		sendStop().catch(console.warn);
	}

	async function handleSend(value: string, files?: File[]) {
		if ((!value.trim() && !files?.length) || sending) return;
		sending = true;
		try {
			const attachments = files?.length ? await filesToAttachments(files) : undefined;
			const requestId = await sendChat(value.trim(), true, attachments);
			const localAtts = attachments?.map((a) => ({
				filename: a.filename,
				content_type: a.content_type,
				url: a.data ? `data:${a.content_type};base64,${a.data}` : a.url
			}));
			chatStore.addUserMessage(requestId, value.trim(), localAtts);
		} finally {
			sending = false;
		}
	}
</script>

<FloatingChatLayout.Root>
	{#snippet header()}
		<div class="flex items-center gap-2 px-4 py-3">
			<div class="bg-primary-bloom flex h-6 w-6 items-center justify-center rounded-md">
				<svg class="text-primary h-3.5 w-3.5" viewBox="0 0 16 16" fill="currentColor">
					<circle cx="8" cy="8" r="5" />
				</svg>
			</div>
			<span class="text-foreground text-sm font-semibold">Yori AI</span>
		</div>
	{/snippet}

	{#snippet messages()}
		<div class="space-y-4 px-4 py-4">
			{#if chatStore.messages.length === 0}
				<div class="text-faint flex h-32 items-center justify-center text-sm">開始對話吧</div>
			{/if}
			{#each chatStore.messages as msg (msg.id)}
				{#if msg.role === 'user'}
					<div class="flex justify-end">
						<Chat.UserBubble content={msg.content} attachments={msg.attachments} />
					</div>
				{:else if msg.role === 'tool'}
					<Chat.ToolCall
						name={msg.name}
						status={msg.status}
						args={msg.args}
						result={msg.result}
						error={msg.error}
					/>
				{:else}
					<Chat.AiResult content={msg.content} isStreaming={msg.pending} />
				{/if}
			{/each}
		</div>
	{/snippet}

	{#snippet input()}
		<Chat.SmartInput
			bind:value={inputValue}
			disabled={sending}
			placeholder="輸入訊息..."
			onSend={handleSend}
			busy={chatStore.busy || speaking}
			onStop={stopReply}
		/>
	{/snippet}
</FloatingChatLayout.Root>
