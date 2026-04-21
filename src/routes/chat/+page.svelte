<!--TODO: 需要重寫-->
<script lang="ts">
	import { onMount, onDestroy } from 'svelte';
	import { chatStore } from '$lib/stores/chat.svelte';
	import { sendChat } from '$lib/backend/ws';

	let input = $state('');
	let sending = $state(false);
	let listEl: HTMLElement;

	onMount(() => chatStore.init());
	onDestroy(() => chatStore.destroy());

	$effect(() => {
		// 有新訊息時捲到底部
		chatStore.messages;
		listEl?.scrollTo({ top: listEl.scrollHeight, behavior: 'smooth' });
	});

	async function handleSend() {
		const content = input.trim();
		if (!content || sending) return;

		input = '';
		sending = true;
		try {
			const requestId = await sendChat(content);
			chatStore.addUserMessage(requestId, content);
		} finally {
			sending = false;
		}
	}

	function handleKeydown(e: KeyboardEvent) {
		if (e.key === 'Enter' && !e.shiftKey) {
			e.preventDefault();
			handleSend();
		}
	}
</script>

<div class="flex h-full flex-col">
	<!-- message list -->
	<div bind:this={listEl} class="flex-1 space-y-3 overflow-y-auto p-4">
		{#each chatStore.messages as msg (msg.id)}
			<div class="flex {msg.role === 'user' ? 'justify-end' : 'justify-start'}">
				<div
					class="max-w-[70%] rounded-2xl px-4 py-2 text-sm
						{msg.role === 'user' ? 'bg-primary text-primary-foreground' : 'bg-muted text-foreground'}
						{msg.pending ? 'opacity-70' : ''}"
				>
					{msg.content}{#if msg.pending}<span class="animate-pulse">▍</span>{/if}
				</div>
			</div>
		{/each}
	</div>

	<!-- input bar -->
	<div class="flex gap-2 border-t p-4">
		<textarea
			bind:value={input}
			onkeydown={handleKeydown}
			placeholder="輸入訊息..."
			rows={1}
			class="bg-background focus:ring-primary flex-1 resize-none rounded-xl border px-3 py-2 text-sm focus:ring-2 focus:outline-none"
		></textarea>
		<button
			onclick={handleSend}
			disabled={sending || !input.trim()}
			class="bg-primary text-primary-foreground rounded-xl px-4 py-2 text-sm disabled:opacity-40"
		>
			送出
		</button>
	</div>
</div>
