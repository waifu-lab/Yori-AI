<script lang="ts">
    import '../app.css';
    
    let { children } = $props();
</script>

<svelte:head>
    <title>AI VTuber Launcher</title>
</svelte:head>

{@render children()}