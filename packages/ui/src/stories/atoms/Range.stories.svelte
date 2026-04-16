<script module>
	import { defineMeta } from '@storybook/addon-svelte-csf';
	import Range from '$lib/atoms/Range.svelte';
	import Field from '$lib/atoms/Field.svelte';

	const { Story } = defineMeta({
		title: 'Atoms/Range',
		component: Range,
		tags: ['autodocs'],
		argTypes: {
			min: { control: 'number' },
			max: { control: 'number' },
			step: { control: 'number' },
			disabled: { control: 'boolean' }
		}
	});
</script>

<Story name="Default" asChild>
	<div class="max-w-sm p-6">
		<Range value={[40]} />
	</div>
</Story>

<Story name="In a Field" asChild>
	<div class="flex max-w-sm flex-col gap-6 p-6">
		<Field label="Temperature" hint="Controls randomness (0 = deterministic, 1 = creative)">
			<Range value={[0.7]} min={0} max={1} step={0.01} />
		</Field>
		<Field label="Max Tokens" hint="Maximum tokens in the response">
			<Range value={[2048]} min={256} max={8192} step={256} />
		</Field>
		<Field label="Top-P" hint="Nucleus sampling threshold">
			<Range value={[0.9]} min={0} max={1} step={0.01} disabled />
		</Field>
	</div>
</Story>
