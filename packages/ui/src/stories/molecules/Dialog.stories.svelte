<script module>
	import { defineMeta } from '@storybook/addon-svelte-csf';
	import Dialog from '$lib/molecules/Dialog.svelte';
	import Button from '$lib/atoms/Button.svelte';
	import Select from '$lib/atoms/Select.svelte';
	import Field from '$lib/atoms/Field.svelte';

	const { Story } = defineMeta({
		title: 'Molecules/Dialog',
		component: Dialog,
		tags: ['autodocs']
	});

	const modelOptions = [
		{ value: 'claude-sonnet', label: 'Claude Sonnet 4.6' },
		{ value: 'claude-haiku', label: 'Claude Haiku 4.5' },
		{ value: 'gpt-4o', label: 'GPT-4o' },
		{ value: 'gpt-4o-mini', label: 'GPT-4o Mini' }
	];
</script>

<Story name="Basic" asChild>
	<div class="p-6">
		<Dialog title="Confirm Action" description="Are you sure you want to proceed?">
			{#snippet trigger()}
				<Button>Open Dialog</Button>
			{/snippet}
			{#snippet children()}
				<p class="text-muted-foreground">This action cannot be undone. All data will be permanently removed.</p>
			{/snippet}
			{#snippet footer()}
				<Button variant="ghost">Cancel</Button>
				<Button variant="danger">Delete</Button>
			{/snippet}
		</Dialog>
	</div>
</Story>

<Story name="Model Selector" asChild>
	<div class="p-6">
		<Dialog title="Select Model" description="Choose the AI model for this conversation." size="md">
			{#snippet trigger()}
				<Button variant="secondary">Change Model</Button>
			{/snippet}
			{#snippet children()}
				<Field label="AI Model" hint="Performance and cost vary between models">
					<Select options={modelOptions} value="claude-sonnet" />
				</Field>
			{/snippet}
			{#snippet footer()}
				<Button variant="ghost">Cancel</Button>
				<Button>Apply</Button>
			{/snippet}
		</Dialog>
	</div>
</Story>

<Story name="Large" asChild>
	<div class="p-6">
		<Dialog title="Attach Files" description="Upload documents, images, or files to include as context." size="lg">
			{#snippet trigger()}
				<Button variant="secondary">Attach Files</Button>
			{/snippet}
			{#snippet children()}
				<div
					class="bg-elevated border-elevated hover:bg-canvas flex h-40 cursor-pointer flex-col items-center justify-center rounded-md border-2 border-dashed transition-colors"
				>
					<p class="text-muted-foreground text-sm">Drag & drop files here</p>
					<p class="text-faint mt-1 text-xs">PDF, DOCX, TXT, PNG — up to 50MB</p>
				</div>
			{/snippet}
			{#snippet footer()}
				<Button variant="ghost">Cancel</Button>
				<Button>Upload</Button>
			{/snippet}
		</Dialog>
	</div>
</Story>
