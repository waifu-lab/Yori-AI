<script module>
	import { defineMeta } from '@storybook/addon-svelte-csf';
	import * as OnboardingLayout from '$lib/layouts/OnboardingLayout/index.js';
	import Button from '$lib/atoms/Button.svelte';
	import Field from '$lib/atoms/Field.svelte';
	import Input from '$lib/atoms/Input.svelte';
	import Select from '$lib/atoms/Select.svelte';

	const { Story } = defineMeta({
		title: 'Layouts/OnboardingLayout',
		component: OnboardingLayout.Root,
		tags: ['autodocs']
	});

	const steps = [
		{ label: 'Welcome', description: 'Get started' },
		{ label: 'Create Persona', description: 'Define your AI' },
		{ label: 'Connect Model', description: 'Link an API' },
		{ label: "You're Ready!", description: 'Start chatting' }
	];
</script>

<Story name="Step 1 — Welcome" asChild>
	<div class="bg-canvas flex h-[550px] max-w-2xl flex-col">
		<OnboardingLayout.Root {steps} current={0} title="Welcome to Yori AI">
			<div class="flex flex-col items-center gap-6 pt-8 text-center">
				<div class="bg-primary-bloom flex h-20 w-20 items-center justify-center rounded-lg text-5xl shadow-xl">✦</div>
				<div>
					<h2 class="text-foreground text-2xl font-bold">Your AI companion awaits</h2>
					<p class="text-muted-foreground mt-2 max-w-sm text-sm">
						Yori lets you create, customise, and converse with AI personas tailored to your workflow.
					</p>
				</div>
			</div>
		</OnboardingLayout.Root>
		<OnboardingLayout.Footer>
			<div></div>
			<Button>Get Started →</Button>
		</OnboardingLayout.Footer>
	</div>
</Story>

<Story name="Step 2 — Create Persona" asChild>
	<div class="bg-canvas flex h-[550px] max-w-2xl flex-col">
		<OnboardingLayout.Root {steps} current={1} title="Create Your Persona">
			<div class="flex max-w-md flex-col gap-5">
				<Field label="Persona Name" hint="Give your AI a memorable name">
					<Input placeholder="e.g. Yori" />
				</Field>
				<Field label="Personality" hint="Describe how your AI should behave">
					<Input placeholder="e.g. Friendly, concise, technical" />
				</Field>
			</div>
		</OnboardingLayout.Root>
		<OnboardingLayout.Footer>
			<Button variant="ghost">← Back</Button>
			<Button>Next →</Button>
		</OnboardingLayout.Footer>
	</div>
</Story>

<Story name="Step 3 — Connect Model" asChild>
	<div class="bg-canvas flex h-[550px] max-w-2xl flex-col">
		<OnboardingLayout.Root {steps} current={2} title="Connect an AI Model">
			<div class="flex max-w-md flex-col gap-5">
				<Field label="Model Provider">
					<Select
						options={[
							{ value: 'anthropic', label: 'Anthropic (Claude)' },
							{ value: 'openai', label: 'OpenAI (GPT)' },
							{ value: 'google', label: 'Google (Gemini)' }
						]}
						placeholder="Select provider..."
					/>
				</Field>
				<Field label="API Key" hint="Your key is stored locally and never sent to our servers">
					<Input placeholder="sk-ant-..." type="password" />
				</Field>
			</div>
		</OnboardingLayout.Root>
		<OnboardingLayout.Footer>
			<Button variant="ghost">← Back</Button>
			<Button>Verify & Continue →</Button>
		</OnboardingLayout.Footer>
	</div>
</Story>

<Story name="Step 4 — Complete" asChild>
	<div class="bg-canvas flex h-[550px] max-w-2xl flex-col">
		<OnboardingLayout.Root {steps} current={3} title="You're All Set!">
			<div class="flex flex-col items-center gap-6 pt-8 text-center">
				<div class="bg-success/15 flex h-20 w-20 items-center justify-center rounded-full text-4xl">✓</div>
				<div>
					<h2 class="text-foreground text-2xl font-bold">Setup complete</h2>
					<p class="text-muted-foreground mt-2 text-sm">Your persona is ready. Start your first conversation now!</p>
				</div>
			</div>
		</OnboardingLayout.Root>
		<OnboardingLayout.Footer>
			<div></div>
			<Button>Start Chatting ✦</Button>
		</OnboardingLayout.Footer>
	</div>
</Story>
