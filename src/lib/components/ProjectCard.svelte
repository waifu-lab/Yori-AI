<script lang="ts">
	import type { ProjectData } from '$lib/commands';

	let { project, onclick }: { project: ProjectData; onclick?: () => void } = $props();

	function formatDate(timestamp: number): string {
		return new Date(timestamp * 1000).toLocaleDateString('zh-TW', {
			year: 'numeric',
			month: '2-digit',
			day: '2-digit'
		});
	}

	function getModeLabel(mode: string): string {
		const modeLabels: Record<string, string> = {
			Streaming: '直播模式',
			Desktop: '桌面寵物',
			Discord: 'Discord'
		};
		return modeLabels[mode] || mode;
	}

	function getModeIcon(mode: string): string {
		const modeIcons: Record<string, string> = {
			Streaming: 'lucide--video',
			Desktop: 'lucide--monitor',
			Discord: 'lucide--message-circle'
		};
		return modeIcons[mode] || 'lucide--circle';
	}
</script>

<button
	{onclick}
	class="border-border bg-card text-card-foreground hover:bg-accent group relative flex flex-col overflow-hidden rounded-lg border text-left shadow-sm transition-colors"
>
	<div class="flex flex-1 flex-col space-y-1.5 p-6">
		<div class="mb-2 flex items-start justify-between gap-2">
			<h3 class="leading-none font-semibold tracking-tight">
				{project.name}
			</h3>
			<span
				class="icon-[lucide--chevron-right] text-muted-foreground h-4 w-4 transition-transform group-hover:translate-x-1"
			></span>
		</div>
		<p class="text-muted-foreground line-clamp-2 text-sm">
			{project.description || '無描述'}
		</p>
	</div>
	<div class="p-6 pt-0">
		<div class="flex items-center justify-between gap-4">
			<div class="text-muted-foreground flex items-center gap-4 text-xs">
				<div class="flex items-center gap-1.5">
					<span class="icon-[lucide--clock] h-3.5 w-3.5"></span>
					<span>{formatDate(project.last_opened)}</span>
				</div>
				{#if project.plugins.length > 0}
					<div class="flex items-center gap-1.5">
						<span class="icon-[lucide--puzzle] h-3.5 w-3.5"></span>
						<span>{project.plugins.length}</span>
					</div>
				{/if}
			</div>
			<div
				class="bg-secondary text-secondary-foreground inline-flex items-center gap-1 rounded-md border border-transparent px-2 py-0.5 text-xs font-semibold transition-colors"
			>
				<span class="icon-[{getModeIcon(project.mode)}] h-3 w-3"></span>
				<span>{getModeLabel(project.mode)}</span>
			</div>
		</div>
	</div>
</button>
