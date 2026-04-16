<script module>
	import { defineMeta } from '@storybook/addon-svelte-csf';
	import Button from '$lib/atoms/Button.svelte';

	const { Story } = defineMeta({
		title: 'Atoms/Button',
		component: Button,
		tags: ['autodocs'],
		argTypes: {
			variant: {
				control: 'select',
				options: ['primary', 'secondary', 'tertiary', 'ghost', 'danger']
			},
			size: {
				control: 'select',
				options: ['sm', 'md', 'lg']
			},
			loading: { control: 'boolean' },
			disabled: { control: 'boolean' }
		}
	});
</script>

<Story name="Primary" args={{ variant: 'primary', size: 'md' }}>
	{#snippet children()}
		Send Message
	{/snippet}
</Story>

<Story name="Ghost" args={{ variant: 'ghost', size: 'md' }}>
	{#snippet children()}
		Cancel
	{/snippet}
</Story>

<Story name="Secondary" args={{ variant: 'secondary', size: 'md' }}>
	{#snippet children()}
		Secondary
	{/snippet}
</Story>

<Story name="Tertiary" args={{ variant: 'tertiary', size: 'md' }}>
	{#snippet children()}
		Tertiary
	{/snippet}
</Story>

<Story name="Danger" args={{ variant: 'danger', size: 'md' }}>
	{#snippet children()}
		Delete Project
	{/snippet}
</Story>

<Story name="Loading" args={{ variant: 'primary', loading: true, size: 'md' }}>
	{#snippet children()}
		Saving...
	{/snippet}
</Story>

<Story name="Disabled" args={{ variant: 'primary', disabled: true, size: 'md' }}>
	{#snippet children()}
		Unavailable
	{/snippet}
</Story>

<Story name="Sizes" asChild>
	<div class="flex items-center gap-3 p-6">
		<Button size="sm">Small</Button>
		<Button size="md">Medium</Button>
		<Button size="lg">Large</Button>
	</div>
</Story>

<Story name="All Variants" asChild>
	<div class="flex flex-wrap gap-3 p-6">
		<Button variant="primary">Primary</Button>
		<Button variant="secondary">Secondary</Button>
		<Button variant="tertiary">Tertiary</Button>
		<Button variant="ghost">Ghost</Button>
		<Button variant="danger">Danger</Button>
	</div>
</Story>
