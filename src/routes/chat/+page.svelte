<script lang="ts">
	import { onMount } from 'svelte';
	import { AppShell, Chat, ConversationList, ToolPanel } from '@waifu/ui';
	import { chatHttpStore } from '$lib/stores/chatHttp.svelte';

	let inputValue = $state('');
	let toolPanelOpen = $state(false);

	onMount(() => {
		chatHttpStore.loadConversations();
	});

	async function handleSend(value: string) {
		if (!value.trim() || chatHttpStore.sending) return;
		await chatHttpStore.send(value.trim());
	}

	function handleNewConversation() {
		chatHttpStore.newConversation();
	}

	const conversations = $derived(
		chatHttpStore.conversations.map((c) => ({
			id: c.id,
			title: (c.title as string) || '對話',
			preview: '',
			timestamp: c.updated_at ? new Date(c.updated_at as string).toLocaleDateString('zh-TW') : ''
		}))
	);
</script>

<AppShell.Root>
	<AppShell.Sidebar class="w-[240px]">
		<ConversationList
			{conversations}
			activeId={chatHttpStore.activeConversationId ?? ''}
			onSelect={(id) => chatHttpStore.selectConversation(id)}
			onNew={handleNewConversation}
		/>
	</AppShell.Sidebar>

	<AppShell.Content>
		<Chat.Layout hasMessages={chatHttpStore.messages.length > 0}>
			{#snippet header()}
				<div class="flex w-full items-center justify-end">
					<button
						type="button"
						onclick={() => (toolPanelOpen = true)}
						class="text-muted-foreground hover:bg-elevated hover:text-foreground flex h-8 items-center
							 rounded-md text-xs font-medium transition-colors"
						title="工具記錄"
					>
						<svg class="h-4 w-4" viewBox="0 0 16 16" fill="none">
							<rect x="2" y="3" width="12" height="2" rx="1" fill="currentColor" />
							<rect x="2" y="7" width="8" height="2" rx="1" fill="currentColor" />
							<rect x="2" y="11" width="10" height="2" rx="1" fill="currentColor" />
						</svg>
					</button>
				</div>
			{/snippet}

			{#snippet empty()}
				<div class="text-faint flex h-full items-center justify-center text-sm">開始一段對話吧</div>
			{/snippet}

			{#snippet messages()}
				{#each chatHttpStore.messages as msg (msg.id)}
					{#if msg.role === 'user'}
						<div class="flex justify-end">
							<Chat.UserBubble content={msg.content} />
						</div>
					{:else}
						<Chat.AiResult content={msg.content} isStreaming={msg.pending} />
					{/if}
				{/each}
			{/snippet}

			{#snippet input()}
				<Chat.SmartInput
					bind:value={inputValue}
					disabled={chatHttpStore.sending}
					placeholder="輸入訊息..."
					models={[
						{ value: 'gpt-4o', label: 'GPT-4o' },
						{ value: 'claude-3-5', label: 'Claude 3.5 Sonnet' },
						{ value: 'gemini-1-5', label: 'Gemini 1.5 Pro' }
					]}
					selectedModel="gpt-4o"
					onSend={handleSend}
				/>
			{/snippet}
		</Chat.Layout>
	</AppShell.Content>
</AppShell.Root>

<ToolPanel open={toolPanelOpen} title="工具記錄" onClose={() => (toolPanelOpen = false)}>
	<div class="space-y-3">
		<p class="text-muted-foreground text-xs">此次對話使用的工具與產生的檔案會顯示在這裡。</p>
	</div>
</ToolPanel>
