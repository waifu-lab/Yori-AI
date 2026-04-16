<script module>
	import { defineMeta } from '@storybook/addon-svelte-csf';
	import Toolbar from '$lib/molecules/Toolbar.svelte';
	import Button from '$lib/atoms/Button.svelte';
	import Select from '$lib/atoms/Select.svelte';

	const { Story } = defineMeta({
		title: 'Molecules/Toolbar',
		component: Toolbar,
		tags: ['autodocs'],
		argTypes: {
			sticky: { control: 'boolean' }
		}
	});
</script>

<script lang="ts">
	const modelOptions = [
		{ value: 'claude-sonnet', label: 'Claude Sonnet 4.6' },
		{ value: 'claude-haiku', label: 'Claude Haiku 4.5' },
		{ value: 'gpt-4o', label: 'GPT-4o' }
	];
</script>

<Story name="Chat Toolbar" asChild>
	<div class="border-elevated w-full max-w-2xl rounded-md border">
		<Toolbar>
			{#snippet left()}
				<span class="text-foreground truncate text-sm font-semibold">Yori Assistant</span>
				<span class="bg-success h-1.5 w-1.5 shrink-0 rounded-full"></span>
			{/snippet}
			{#snippet center()}
				<Select options={modelOptions} value="claude-sonnet" size="sm" class="w-44" />
			{/snippet}
			{#snippet right()}
				<Button variant="ghost" size="sm">
					<span class="icon-[material-symbols--settings] text-base"></span>
				</Button>
				<Button variant="ghost" size="sm">
					<span class="icon-[material-symbols--more-vert] text-base"></span>
				</Button>
			{/snippet}
		</Toolbar>
	</div>
</Story>

<Story name="App Toolbar" asChild>
	<div class="border-elevated w-full max-w-3xl rounded-md border">
		<Toolbar>
			{#snippet left()}
				<div class="bg-primary flex h-7 w-7 items-center justify-center rounded-md text-xs font-bold text-white shadow">
					Y
				</div>
				<span class="text-foreground text-sm font-semibold">Yori AI</span>
			{/snippet}
			{#snippet right()}
				<Button variant="secondary" size="sm">New Chat</Button>
				<Button variant="ghost" size="sm">
					<span class="icon-[material-symbols--account-circle] text-lg"></span>
				</Button>
			{/snippet}
		</Toolbar>
	</div>
</Story>

<Story name="Sticky Toolbar" asChild>
	<div class="border-elevated h-48 w-full max-w-2xl overflow-y-auto rounded-md border">
		<Toolbar sticky>
			{#snippet left()}
				<span class="text-foreground text-sm font-semibold">Conversation</span>
			{/snippet}
			{#snippet right()}
				<Button variant="ghost" size="sm">Clear</Button>
				<Button variant="ghost" size="sm">Export</Button>
			{/snippet}
		</Toolbar>
		<div class="flex flex-col gap-3 p-4">
			{#each Array(6) as _, i}
				<div class="bg-surface text-muted-foreground rounded-md p-3 text-sm">
					Message {i + 1} — scroll down to test sticky behaviour.
				</div>
			{/each}
		</div>
	</div>
</Story>

<Story name="Left Only" asChild>
	<div class="border-elevated w-full max-w-2xl rounded-md border">
		<Toolbar>
			{#snippet left()}
				<Button variant="ghost" size="sm">
					<span class="icon-[material-symbols--arrow-back] text-base"></span>
				</Button>
				<span class="text-foreground text-sm font-semibold">Settings</span>
			{/snippet}
		</Toolbar>
	</div>
</Story>
