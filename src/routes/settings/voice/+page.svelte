<script lang="ts">
	import { onMount } from 'svelte';
	import { Setting, Button, Badge, Select, Field } from '@waifu/ui';
	import { vadStore } from '$lib/stores/vad.svelte';
	import { listInputDevices, vadGetDevice, vadSetDevice } from '$lib/commands';

	let devices = $state<string[]>([]);
	let selectedDevice = $state<string>('');

	const deviceOptions = $derived([
		{ value: '', label: '系統預設' },
		...devices.map((d) => ({ value: d, label: d }))
	]);

	async function loadDevices() {
		devices = await listInputDevices().catch(() => []);
		const current = await vadGetDevice().catch(() => null);
		selectedDevice = current ?? '';
	}

	async function onDeviceChange(value: string) {
		selectedDevice = value;
		await vadSetDevice(value === '' ? null : value).catch(() => {});
	}

	onMount(async () => {
		await Promise.all([vadStore.init(), loadDevices()]);
	});
</script>

<div class="flex flex-col gap-3">
	<Setting.Card
		title="語音活動偵測 (VAD)"
		icon="icon-[material-symbols--mic-outline-rounded]"
		description="啟用後，系統將持續監聽麥克風，自動偵測語音片段並傳送至 AI 後端"
	>
		<div class="flex flex-col gap-3">
			<div class="flex items-center justify-between">
				<span class="text-muted-foreground text-xs">目前狀態</span>
				<Badge variant={vadStore.active ? 'primary' : 'muted'}>{vadStore.active ? '偵測中' : '已停止'}</Badge>
			</div>

			<Button
				variant={vadStore.active ? 'danger' : 'primary'}
				onclick={() => vadStore.toggle()}
				disabled={vadStore.busy}
			>
				{vadStore.active ? '停止偵測' : '開始偵測'}
			</Button>

			{#if vadStore.error}
				<p class="text-destructive text-xs">{vadStore.error}</p>
			{/if}
		</div>
	</Setting.Card>

	<Setting.Card
		title="麥克風設備"
		icon="icon-[material-symbols--settings-input-component-outline-rounded]"
		description="選擇語音偵測使用的輸入設備，預設使用系統設定的麥克風"
	>
		<Field label="輸入設備">
			<Select
				options={deviceOptions}
				bind:value={selectedDevice}
				placeholder="系統預設"
				onValueChange={onDeviceChange}
			/>
		</Field>
	</Setting.Card>
</div>
