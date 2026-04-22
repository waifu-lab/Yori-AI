<script module>
	import { defineMeta } from '@storybook/addon-svelte-csf';
	import * as Chat from '$lib/molecules/chat/index.js';
	import MarkdownRenderer from '$lib/molecules/MarkdownRenderer.svelte';

	const { Story } = defineMeta({
		title: 'Molecules/Chat',
		component: Chat.Root,
		tags: ['autodocs']
	});
</script>

<script lang="ts">
	const models = [
		{ value: 'claude-sonnet', label: 'Claude Sonnet 4.6' },
		{ value: 'claude-haiku', label: 'Claude Haiku 4.5' },
		{ value: 'gpt-4o', label: 'GPT-4o' }
	];
</script>

<Story name="AI Bubble — Default" asChild>
	<div class="max-w-xl p-8">
		<Chat.AiResult content="Hello! I'm Yori, your AI companion. How can I help you today?" />
	</div>
</Story>

<Story name="AI Bubble — Streaming" asChild>
	<div class="max-w-xl p-8">
		<Chat.AiResult content="Generating a response for you..." isStreaming />
	</div>
</Story>

<Story name="AI Bubble — With Markdown" asChild>
	<div class="max-w-xl p-8">
		<Chat.AiResult>
			{#snippet children()}
				<MarkdownRenderer
					source={`Sure! Here's a quick example:

\`\`\`typescript
const greeting = (name: string) => \`Hello, \${name}!\`;
\`\`\`

Let me know if you need more details.`}
				/>
			{/snippet}
		</Chat.AiResult>
	</div>
</Story>

<Story name="User Bubble" asChild>
	<div class="flex max-w-xl flex-col items-end p-8">
		<Chat.UserBubble content="Can you explain how Svelte 5 runes work?" />
	</div>
</Story>

<Story name="Conversation" asChild>
	<div class="flex max-w-2xl flex-col gap-6 p-8">
		<Chat.UserBubble content="What is the Etheric Control design system?" />
		<Chat.AiResult
			content="Etheric Control is a design system built on the principles of Organic Minimalism and a No-Line Philosophy. Instead of using hard borders, it uses subtle tonal shifts and ambient blooms to create visual hierarchy."
		/>
		<Chat.UserBubble content="What font does it use?" />
		<Chat.AiResult
			content="It uses **Manrope** — a modern geometric sans-serif that performs well at small sizes while maintaining a strong brand character at high weights."
			isStreaming
		/>
	</div>
</Story>

<Story name="SmartInput — Default" asChild>
	<div class="bg-canvas max-w-2xl pt-4">
		<Chat.SmartInput {models} selectedModel="claude-sonnet" />
	</div>
</Story>

<Story name="SmartInput — No Models" asChild>
	<div class="bg-canvas max-w-2xl pt-4">
		<Chat.SmartInput placeholder="Ask anything..." />
	</div>
</Story>

<Story name="Full Chat Layout" asChild>
	<div class="bg-canvas flex h-[600px] max-w-2xl flex-col">
		<Chat.Root>
			<Chat.Messages>
				<div class="flex flex-col gap-6">
					<Chat.UserBubble content="Hello Yori! Can you introduce yourself?" />
					<Chat.AiBubble
						content="Hi! I'm Yori, an AI assistant built with the Etheric Control design system. I'm here to help you with anything you need — from coding questions to creative writing. What would you like to explore today?"
					/>
					<Chat.UserBubble content="That sounds great! Can you help me write a Svelte component?" />
					<Chat.AiBubble isStreaming content="Of course! Let me put together an example for you..." />
				</div>
			</Chat.Messages>
			<Chat.Input>
				<Chat.SmartInput {models} selectedModel="claude-sonnet" />
			</Chat.Input>
		</Chat.Root>
	</div>
</Story>

<Story name="Empty Chat" asChild>
	<div class="bg-canvas flex h-[500px] max-w-2xl flex-col">
		<Chat.Root>
			<Chat.Messages>
				<Chat.Empty>
					<div class="flex flex-col items-center gap-3 text-center">
						<div class="bg-primary-bloom flex h-16 w-16 items-center justify-center rounded-lg">
							<span class="text-3xl">✦</span>
						</div>
						<h2 class="text-foreground text-xl font-bold tracking-tight">Start a conversation</h2>
						<p class="text-muted-foreground max-w-xs text-sm">
							Ask Yori anything — from creative ideas to technical questions.
						</p>
					</div>
				</Chat.Empty>
			</Chat.Messages>
			<Chat.Input>
				<Chat.SmartInput {models} selectedModel="claude-sonnet" />
			</Chat.Input>
		</Chat.Root>
	</div>
</Story>
