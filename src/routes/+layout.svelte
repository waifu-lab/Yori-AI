<script lang="ts">
    import '../app.css';
    
    let { children } = $props();
</script>

<svelte:head>
    <title>AI VTuber Launcher</title>
</svelte:head>

{@render children()}

<style>
    :global(body) {
        margin: 0;
        padding: 0;
        font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, Cantarell, sans-serif;
        background: #0f0f0f;
        color: #e4e4e7;
    }
</style>