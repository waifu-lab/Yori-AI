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

<Story name="Tool Call — Running" asChild>
	<div class="max-w-xl p-8">
		<Chat.ToolCall
			name="screen.capture"
			status="running"
			args={{ region: 'primary', format: 'png' }}
			defaultOpen
		/>
	</div>
</Story>

<Story name="Tool Call — Success" asChild>
	<div class="max-w-xl p-8">
		<Chat.ToolCall
			name="exec.run"
			status="success"
			args={{ command: 'git status', cwd: '/repo' }}
			result={{ stdout: 'On branch v2\nnothing to commit, working tree clean\n', exit_code: 0 }}
			defaultOpen
		/>
	</div>
</Story>

<Story name="Tool Call — Error" asChild>
	<div class="max-w-xl p-8">
		<Chat.ToolCall
			name="sysinfo.read"
			status="error"
			args={{ key: 'unknown.metric' }}
			error="UnknownKey: 'unknown.metric' is not a registered sysinfo field"
			defaultOpen
		/>
	</div>
</Story>

<Story name="Tool Call — Collapsed" asChild>
	<div class="flex max-w-xl flex-col gap-2 p-8">
		<Chat.ToolCall name="screen.capture" status="success" args={{ region: 'primary' }} result={{ ok: true }} />
		<Chat.ToolCall name="exec.run" status="error" args={{ command: 'rm -rf /' }} error="Permission denied" />
		<Chat.ToolCall name="audio.play" status="running" args={{ url: 'tts://hello' }} />
	</div>
</Story>

<Story name="Conversation with Tool Calls" asChild>
	<div class="flex max-w-2xl flex-col gap-6 p-8">
		<Chat.UserBubble content="Take a screenshot and tell me what's open." />
		<Chat.ToolCall
			name="screen.capture"
			status="success"
			args={{ region: 'primary', format: 'png' }}
			result={{ width: 2560, height: 1440, bytes: 184293 }}
		/>
		<Chat.ToolCall
			name="sysinfo.windows"
			status="success"
			args={{}}
			result={[
				{ title: 'VS Code — Yori-AI', pid: 4821 },
				{ title: 'Storybook', pid: 9013 }
			]}
		/>
		<Chat.AiResult
			content="I captured your primary display (2560×1440). You currently have **VS Code** and **Storybook** in the foreground."
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
					<Chat.AiResult
						content="Hi! I'm Yori, an AI assistant built with the Etheric Control design system. I'm here to help you with anything you need — from coding questions to creative writing. What would you like to explore today?"
					/>
					<Chat.UserBubble content="That sounds great! Can you help me write a Svelte component?" />
					<Chat.AiResult isStreaming content="Of course! Let me put together an example for you..." />
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
