<script module>
	import { defineMeta } from '@storybook/addon-svelte-csf';
	import SettingsLayout from '$lib/layouts/SettingsLayout.svelte';
	import SettingCard from '$lib/molecules/setting/SettingCard.svelte';
	import Button from '$lib/atoms/Button.svelte';
	import Select from '$lib/atoms/Select.svelte';
	import Input from '$lib/atoms/Input.svelte';
	import Field from '$lib/atoms/Field.svelte';

	const { Story } = defineMeta({
		title: 'Layouts/SettingsLayout',
		component: SettingsLayout,
		tags: ['autodocs']
	});
</script>

<Story name="General Settings" asChild>
	<div class="bg-canvas min-h-[600px]">
		<SettingsLayout title="Settings" description="Configure your Yori AI experience.">
			<p class="text-muted-foreground text-xs font-semibold tracking-widest uppercase">General</p>

			<SettingCard
				title="Display Name"
				description="Your name shown in conversations."
				icon="icon-[material-symbols--badge]"
			>
				{#snippet children()}
					<Input value="Rex Wu" placeholder="Your name" />
				{/snippet}
			</SettingCard>

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

			<SettingCard
				title="Default Model"
				description="Model used for new conversations."
				icon="icon-[material-symbols--smart-toy]"
			>
				{#snippet control()}
					<Select
						options={[
							{ value: 'claude-sonnet', label: 'Claude Sonnet 4.6' },
							{ value: 'gpt-4o', label: 'GPT-4o' }
						]}
						value="claude-sonnet"
						size="sm"
						class="w-44"
					/>
				{/snippet}
			</SettingCard>

			<p class="text-muted-foreground mt-4 text-xs font-semibold tracking-widest uppercase">Danger Zone</p>

			<SettingCard
				title="Delete All Data"
				description="Permanently removes all personas, conversations, and settings."
				icon="icon-[material-symbols--delete-forever]"
				variant="danger"
			>
				{#snippet control()}
					<Button variant="danger" size="sm">Delete Everything</Button>
				{/snippet}
			</SettingCard>
		</SettingsLayout>
	</div>
</Story>
