<script lang="ts">
	import { onMount, onDestroy } from 'svelte';
	import { listen, type UnlistenFn } from '@tauri-apps/api/event';
	import { Setting } from '@waifu/ui';

	let db = $state(-100);
	// Map dB range [-60, 0] to [0, 100]
	const dbToPercent = (v: number) => Math.max(0, Math.min(100, ((v + 60) / 60) * 100));
	const volumePercent = $derived(dbToPercent(db));
	const volumeColor = $derived(volumePercent > 80 ? '#ef4444' : volumePercent > 50 ? '#f59e0b' : '#22c55e');

	interface LogEntry {
		time: string;
		msg: string;
	}
	const MAX_LOGS = 200;
	let logs = $state<LogEntry[]>([]);
	let logEl = $state<HTMLDivElement | null>(null);
	let autoScroll = $state(true);

	function pushLog(msg: string) {
		const now = new Date();
		const time = `${now.getHours().toString().padStart(2, '0')}:${now.getMinutes().toString().padStart(2, '0')}:${now.getSeconds().toString().padStart(2, '0')}.${now.getMilliseconds().toString().padStart(3, '0')}`;
		logs = [{ time, msg }, ...logs].slice(0, MAX_LOGS);
	}

	$effect(() => {
		if (autoScroll && logEl) {
			logEl.scrollTop = 0;
		}
	});

	let unlistenVolume: UnlistenFn | null = null;
	let unlistenLog: UnlistenFn | null = null;

	onMount(async () => {
		unlistenVolume = await listen<number>('vad-debug-volume', (e) => {
			db = e.payload;
		});
		unlistenLog = await listen<string>('vad-debug-log', (e) => {
			pushLog(e.payload);
		});
	});

	onDestroy(() => {
		unlistenVolume?.();
		unlistenLog?.();
	});
</script>

<div class="flex flex-col gap-3">
	<Setting.Card
		title="麥克風音量"
		icon="icon-[material-symbols--graphic-eq-rounded]"
		description="即時顯示麥克風輸入音量（需先開啟語音偵測）"
	>
		<div class="flex flex-col gap-2">
			<div class="flex items-center gap-3">
				<div class="bg-surface-raised h-4 flex-1 overflow-hidden rounded-full">
					<div
						class="h-full rounded-full transition-all duration-75"
						style="width: {volumePercent}%; background-color: {volumeColor};"
					></div>
				</div>
				<span class="text-muted-foreground w-16 text-right font-mono text-xs">
					{db > -99 ? `${db.toFixed(1)} dB` : '-∞ dB'}
				</span>
			</div>
			<div class="text-muted-foreground flex justify-between text-xs">
				<span>-60 dB</span>
				<span>-30 dB</span>
				<span>0 dB</span>
			</div>
		</div>
	</Setting.Card>

	<Setting.Card
		title="VAD Log"
		icon="icon-[material-symbols--terminal-rounded]"
		description="語音活動偵測事件記錄（最多 {MAX_LOGS} 筆，最新在上）"
	>
		<div class="flex items-center justify-between pb-1">
			<label class="text-muted-foreground flex items-center gap-1.5 text-xs">
				<input type="checkbox" bind:checked={autoScroll} class="h-3 w-3" />
				自動捲動
			</label>
			<button class="text-muted-foreground hover:text-foreground text-xs transition-colors" onclick={() => (logs = [])}>
				清除
			</button>
		</div>
		<div bind:this={logEl} class="bg-surface-raised h-64 overflow-y-auto rounded-md p-2 font-mono text-xs">
			{#if logs.length === 0}
				<p class="text-muted-foreground italic">等待事件...</p>
			{:else}
				{#each logs as entry (entry.time + entry.msg)}
					<div class="flex gap-2 leading-5">
						<span class="text-muted-foreground shrink-0">{entry.time}</span>
						<span
							class:text-green-400={entry.msg.startsWith('[SPEECH')}
							class:text-yellow-400={entry.msg.startsWith('[UTTERANCE')}
							class:text-zinc-400={entry.msg.startsWith('[SILENCE')}>{entry.msg}</span
						>
					</div>
				{/each}
			{/if}
		</div>
	</Setting.Card>
</div>
