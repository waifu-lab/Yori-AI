<script module>
	import { defineMeta } from '@storybook/addon-svelte-csf';
	import StepIndicator from '$lib/molecules/StepIndicator.svelte';

	const { Story } = defineMeta({
		title: 'Molecules/StepIndicator',
		component: StepIndicator,
		tags: ['autodocs'],
		argTypes: {
			current: { control: { type: 'number', min: 0, max: 3 } },
			orientation: { control: 'select', options: ['horizon', 'nadir'] }
		}
	});
</script>

<script lang="ts">
	const onboardingSteps = [
		{ label: 'Welcome', description: 'Get started' },
		{ label: 'Create Persona', description: 'Define your AI' },
		{ label: 'Connect Model', description: 'Link an API' },
		{ label: 'Start Chatting', description: "You're ready!" }
	];
</script>

<Story name="Horizon — Step 1" args={{ orientation: 'horizon', current: 0 }} asChild>
	<div class="p-8">
		<StepIndicator steps={onboardingSteps} current={0} orientation="horizon" />
	</div>
</Story>

<Story name="Horizon — Step 2" args={{ orientation: 'horizon', current: 1 }} asChild>
	<div class="p-8">
		<StepIndicator steps={onboardingSteps} current={1} orientation="horizon" />
	</div>
</Story>

<Story name="Horizon — Complete" args={{ orientation: 'horizon', current: 3 }} asChild>
	<div class="p-8">
		<StepIndicator steps={onboardingSteps} current={3} orientation="horizon" />
	</div>
</Story>

<Story name="Nadir — Step 2" args={{ orientation: 'nadir', current: 1 }} asChild>
	<div class="max-w-xs p-8">
		<StepIndicator steps={onboardingSteps} current={1} orientation="nadir" />
	</div>
</Story>
