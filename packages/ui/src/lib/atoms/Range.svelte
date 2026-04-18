<script lang="ts">
	import { Slider } from 'bits-ui';

	interface Props {
		value?: number[];
		min?: number;
		max?: number;
		step?: number;
		disabled?: boolean;
		class?: string;
		onValueChange?: (value: number[]) => void;
	}

	let {
		value = $bindable([0]),
		min = 0,
		max = 100,
		step = 1,
		disabled = false,
		class: className = '',
		onValueChange
	}: Props = $props();
</script>

<Slider.Root
	bind:value
	type="multiple"
	{min}
	{max}
	{step}
	{disabled}
	{onValueChange}
	class="relative flex w-full touch-none items-center select-none {className}"
>
	{#snippet children({ thumbs })}
		<span class="bg-surface relative h-1.5 w-full grow overflow-hidden rounded-full">
			<Slider.Range class="bg-primary absolute h-full rounded-full" />
		</span>

		{#each thumbs as thumb (thumb)}
			<Slider.Thumb
				index={thumb}
				class="bg-primary ring-offset-canvas  focus-visible:ring-ring block
          h-4 w-4 rounded-full
          hover:scale-110 focus-visible:ring-2 focus-visible:ring-offset-2
          focus-visible:outline-none active:scale-95
          disabled:pointer-events-none disabled:opacity-40"
			/>
		{/each}
	{/snippet}
</Slider.Root>
