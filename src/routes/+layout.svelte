<script lang="ts">
	import '../app.css';

	let { children } = $props();
</script>

<svelte:head>
	<title>Yori AI</title>
</svelte:head>

{@render children()}