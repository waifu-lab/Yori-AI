<script lang="ts">
	import Checkbox from './Checkbox.svelte';

	interface Option {
		value: string;
		label: string;
		disabled?: boolean;
	}

	interface Props {
		options: Option[];
		value?: string[];
		disabled?: boolean;
		orientation?: 'horizontal' | 'vertical';
		class?: string;
		onChange?: (value: string[]) => void;
	}

	let {
		options,
		value = $bindable([]),
		disabled = false,
		orientation = 'vertical',
		class: className = '',
		onChange
	}: Props = $props();

	function toggle(optionValue: string) {
		const next = value.includes(optionValue) ? value.filter((v) => v !== optionValue) : [...value, optionValue];
		value = next;
		onChange?.(next);
	}
</script>

<div class="flex gap-3 {orientation === 'horizontal' ? 'flex-row flex-wrap' : 'flex-col'} {className}" role="group">
	{#each options as option (option.value)}
		<Checkbox
			id="checkbox-{option.value}"
			checked={value.includes(option.value)}
			disabled={disabled || option.disabled}
			label={option.label}
			onCheckedChange={() => toggle(option.value)}
		/>
	{/each}
</div>
