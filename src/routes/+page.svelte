<script lang="ts">
	import { getCurrentWindow } from '@tauri-apps/api/window';
	import { onMount } from 'svelte';
	import { openLive2dWindow } from '$lib/commands';

	onMount(async () => {
		await openLive2dWindow().catch(console.error);
		await getCurrentWindow().hide().catch(console.error);
	});
</script>
