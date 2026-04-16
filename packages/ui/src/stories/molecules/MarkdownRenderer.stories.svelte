<script module>
	import { defineMeta } from '@storybook/addon-svelte-csf';
	import MarkdownRenderer from '$lib/molecules/MarkdownRenderer.svelte';

	const { Story } = defineMeta({
		title: 'Molecules/MarkdownRenderer',
		component: MarkdownRenderer,
		tags: ['autodocs']
	});
</script>

<script lang="ts">
	const sampleMarkdown = `# Etheric Control

Welcome to **Yori AI** — your intelligent companion.

## Features

- Natural language understanding
- Multi-modal input support
- Plugin extensibility

## Code Example

\`\`\`typescript
const yori = new YoriAI({
  model: 'claude-sonnet-4-6',
  temperature: 0.7,
});

const response = await yori.chat('Hello!');
\`\`\`

> "The best interface is one you forget you're using."

### Parameters

| Name | Type | Default | Description |
|------|------|---------|-------------|
| model | string | claude-sonnet | AI model ID |
| temperature | number | 0.7 | Randomness |
| maxTokens | number | 2048 | Token limit |

<details>
<summary>Want to see something cool?</summary>
Here's a hidden surprise! 🎉
</details>

Happy coding! <span style="color: hotpink">♥</span>

`;
</script>

<Story name="Full Document" asChild>
	<div class="max-w-2xl p-8">
		<MarkdownRenderer source={sampleMarkdown} />
	</div>
</Story>

<Story name="AI Response" asChild>
	<div class="max-w-xl p-8">
		<MarkdownRenderer
			source={`Sure! Here's how to get started:

1. **Install** the package with \`pnpm install\`
2. **Configure** your API key in settings
3. **Create** your first persona

Let me know if you need help with any of these steps!`}
		/>
	</div>
</Story>
