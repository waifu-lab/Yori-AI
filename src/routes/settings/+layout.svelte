<script lang="ts">
	import type { LayoutProps } from './$types';
	import { page } from '$app/state';
	import { AppShell, Sidebar, SettingsLayout } from '@waifu/ui';

	const settingsNav = [
		{ href: '/settings', label: '一般', icon: 'icon-[material-symbols--settings-outline-rounded]' },
		{ href: '/settings/live2d', label: 'Live2D', icon: 'icon-[material-symbols--person-outline-rounded]' },
		{ href: '/settings/ai', label: 'AI 模型', icon: 'icon-[material-symbols--smart-toy-outline-rounded]' },
		{ href: '/settings/voice', label: '語音', icon: 'icon-[material-symbols--mic-outline-rounded]' },
		{ href: '/settings/mcp', label: 'MCP', icon: 'icon-[material-symbols--hub-outline]' },
		{ href: '/settings/plugins', label: '外掛', icon: 'icon-[material-symbols--extension-outline-rounded]' },
		{ href: '/settings/skills', label: '技能', icon: 'icon-[material-symbols--auto-awesome-outline-rounded]' },
		{ href: '/settings/appearance', label: '外觀', icon: 'icon-[material-symbols--palette-outline]' },
		{ href: '/settings/debug', label: 'Debug', icon: 'icon-[material-symbols--bug-report-outline-rounded]' },
		{ href: '/settings/about', label: '關於', icon: 'icon-[material-symbols--info-outline-rounded]' }
	];

	let { children }: LayoutProps = $props();

	const currentNav = $derived(settingsNav.find((n) => n.href === page.url.pathname) ?? settingsNav[0]);
</script>

<AppShell.Root>
	<AppShell.Sidebar class="w-[200px]">
		<Sidebar items={settingsNav} currentPath={page.url.pathname} />
	</AppShell.Sidebar>

	<AppShell.Content class="overflow-y-auto">
		<SettingsLayout.Root title={currentNav.label} icon={currentNav.icon}>
			{@render children()}
		</SettingsLayout.Root>
	</AppShell.Content>
</AppShell.Root>
