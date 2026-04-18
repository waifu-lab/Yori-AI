<script module>
	import { defineMeta } from '@storybook/addon-svelte-csf';
	import SettingCard from '$lib/molecules/setting/SettingCard.svelte';
	import Button from '$lib/atoms/Button.svelte';
	import Select from '$lib/atoms/Select.svelte';
	import Input from '$lib/atoms/Input.svelte';

	const { Story } = defineMeta({
		title: 'Molecules/SettingCard',
		component: SettingCard,
		tags: ['autodocs']
	});
</script>

<Story name="Basic" asChild>
	<div class="flex max-w-lg flex-col gap-3 p-6">
		<SettingCard
			title="Display Name"
			description="The name shown to users in conversation."
			icon="icon-[material-symbols--badge]"
		>
			{#snippet children()}
				<Input placeholder="e.g. Yori" value="Yori" />
			{/snippet}
		</SettingCard>
	</div>
</Story>

<Story name="With Control Slot" asChild>
	<div class="flex max-w-lg flex-col gap-3 p-6">
		<SettingCard
			title="Auto-save Conversations"
			description="Automatically save chat history to local storage."
			icon="icon-[material-symbols--save]"
		>
			{#snippet control()}
				<!-- Simulated toggle switch -->
				<button
					class="bg-primary relative inline-flex h-5 w-9 items-center rounded-full transition-colors"
					role="switch"
					aria-checked="true"
					aria-label="Toggle auto-save conversations"
				>
					<span class="inline-block h-3.5 w-3.5 translate-x-5 rounded-full bg-white shadow transition-transform"></span>
				</button>
			{/snippet}
		</SettingCard>

		<SettingCard
			title="Model"
			description="Default AI model for new conversations."
			icon="icon-[material-symbols--smart-toy]"
		>
			{#snippet control()}
				<Select
					options={[
						{ value: 'claude-sonnet', label: 'Claude Sonnet' },
						{ value: 'gpt-4o', label: 'GPT-4o' }
					]}
					value="claude-sonnet"
					size="sm"
					class="w-40"
				/>
			{/snippet}
		</SettingCard>
	</div>
</Story>

<Story name="Danger Variant" asChild>
	<div class="flex max-w-lg flex-col gap-3 p-6">
		<SettingCard
			title="Delete All Conversations"
			description="Permanently removes all chat history. This cannot be undone."
			icon="icon-[material-symbols--delete-forever]"
			variant="danger"
		>
			{#snippet control()}
				<Button variant="danger" size="sm">Delete</Button>
			{/snippet}
		</SettingCard>

		<SettingCard
			title="Reset to Defaults"
			description="Resets all settings to their factory defaults."
			icon="icon-[material-symbols--settings-backup-restore]"
			variant="danger"
		>
			{#snippet control()}
				<Button variant="danger" size="sm">Reset</Button>
			{/snippet}
		</SettingCard>
	</div>
</Story>

<Story name="Settings Section" asChild>
	<div class="flex max-w-lg flex-col gap-2 p-6">
		<p class="text-muted-foreground mb-2 text-xs font-semibold tracking-widest uppercase">General</p>
		<SettingCard title="Language" description="Interface language." icon="icon-[material-symbols--language]">
			{#snippet control()}
				<Select
					options={[
						{ value: 'en', label: 'English' },
						{ value: 'zh', label: '繁體中文' }
					]}
					value="en"
					size="sm"
					class="w-36"
				/>
			{/snippet}
		</SettingCard>
		<SettingCard title="Theme" description="Light or dark interface." icon="icon-[material-symbols--dark-mode]">
			{#snippet control()}
				<Select
					options={[
						{ value: 'dark', label: 'Dark' },
						{ value: 'light', label: 'Light' }
					]}
					value="dark"
					size="sm"
					class="w-28"
				/>
			{/snippet}
		</SettingCard>

		<p class="text-muted-foreground mt-4 mb-2 text-xs font-semibold tracking-widest uppercase">Danger Zone</p>
		<SettingCard title="Delete Account" description="Permanently delete your account and all data." variant="danger">
			{#snippet control()}
				<Button variant="danger" size="sm">Delete</Button>
			{/snippet}
		</SettingCard>
	</div>
</Story>
