<script lang="ts">
	import { onMount } from 'svelte';
	import { Setting, Badge } from '@waifu/ui';
	import { getStatus } from '$lib/backend/http';

	type StatusData = Record<string, unknown>;

	let status = $state<StatusData | null>(null);
	let statusError = $state<string | null>(null);
	let loading = $state(true);

	onMount(async () => {
		try {
			status = await getStatus();
		} catch (e) {
			statusError = String(e);
		} finally {
			loading = false;
		}
	});
</script>

<div class="flex flex-col gap-3">
	<Setting.Card
		title="系統狀態"
		icon="icon-[material-symbols--monitor-heart-outline-rounded]"
		description="後端服務的即時運行狀態"
	>
		{#if loading}
			<p class="text-muted-foreground text-xs">載入中…</p>
		{:else if statusError}
			<p class="text-danger text-xs">無法取得狀態：{statusError}</p>
		{:else if status}
			<div class="flex flex-col gap-1.5">
				{#each Object.entries(status) as [key, value]}
					<div class="flex items-center justify-between gap-2">
						<span class="text-muted-foreground text-xs">{key}</span>
						<Badge
							variant={value === true || value === 'ok' || value === 'running'
								? 'primary'
								: value === false || value === 'error'
									? 'danger'
									: 'muted'}
						>
							{String(value)}
						</Badge>
					</div>
				{/each}
			</div>
		{/if}
	</Setting.Card>
</div>
