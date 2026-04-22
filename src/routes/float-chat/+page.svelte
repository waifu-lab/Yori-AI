<script lang="ts">
	import { onMount, onDestroy } from 'svelte';
	import { FloatingChatLayout, Chat } from '@waifu/ui';
	import { chatStore } from '$lib/stores/chat.svelte';
	import { sendChat } from '$lib/backend/ws';

	let inputValue = $state('');
	let sending = $state(false);

	onMount(() => chatStore.init());
	onDestroy(() => chatStore.destroy());

	async function handleSend(value: string) {
		if (!value.trim() || sending) return;
		sending = true;
		try {
			const requestId = await sendChat(value.trim(), true);
			chatStore.addUserMessage(requestId, value.trim());
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
						<Chat.UserBubble content={msg.content} />
					</div>
				{:else}
					<Chat.AiResult content={msg.content} isStreaming={msg.pending} />
				{/if}
			{/each}
		</div>
	{/snippet}

	{#snippet input()}
		<Chat.SmartInput bind:value={inputValue} disabled={sending} placeholder="輸入訊息..." onSend={handleSend} />
	{/snippet}
</FloatingChatLayout.Root>
