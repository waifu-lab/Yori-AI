<script lang="ts">
    import '../app.css';
    
    let { children } = $props();
</script>

<svelte:head>
    <title>Yori AI</title>
</svelte:head>

<div class="flex h-screen w-full bg-slate-950 text-slate-200 font-sans overflow-hidden">
    <!-- Sidebar -->
    <aside class="w-[260px] flex-shrink-0 flex flex-col bg-slate-900 border-r border-slate-800 p-4">
        <!-- Header -->
        <div class="flex items-center gap-3 px-4 py-4 mb-6">
            <div class="w-8 h-8 bg-indigo-500 rounded-lg flex items-center justify-center text-white shadow-lg shadow-indigo-500/20">
                 <span class="icon-[material-symbols--smart-toy] w-5 h-5"></span>
            </div>
            <span class="text-xl font-bold text-white tracking-tight">Yori AI</span>
        </div>

        <!-- Navigation -->
        <nav class="flex-1 space-y-1">
            <a href="/project" class="flex items-center gap-3 px-4 py-3 rounded-full text-slate-400 hover:bg-slate-800 hover:text-slate-200 transition-colors group">
                <span class="icon-[material-symbols--folder] w-5 h-5 group-hover:text-white transition-colors"></span>
                <span class="font-medium">Projects</span>
            </a>
            
            <a href="/" class="flex items-center gap-3 px-4 py-3 rounded-full text-slate-400 hover:bg-slate-800 hover:text-slate-200 transition-colors group">
                <span class="icon-[material-symbols--auto-awesome] w-5 h-5 group-hover:text-white transition-colors"></span>
                <span class="font-medium">Persona & Soul</span>
            </a>

            <a href="/mixer" class="flex items-center gap-3 px-4 py-3 rounded-full text-slate-400 hover:bg-slate-800 hover:text-slate-200 transition-colors group">
                <span class="icon-[material-symbols--settings-input-component] w-5 h-5 group-hover:text-white transition-colors"></span>
                <span class="font-medium">Channel Mixer</span>
            </a>

            <a href="/plugin" class="flex items-center gap-3 px-4 py-3 rounded-full text-slate-400 hover:bg-slate-800 hover:text-slate-200 transition-colors group">
                <span class="icon-[material-symbols--extension] w-5 h-5 group-hover:text-white transition-colors"></span>
                <span class="font-medium">Plugin Store</span>
            </a>
        </nav>

        <!-- Footer -->
        <div class="mt-auto pt-4 border-t border-slate-800">
            <div class="flex items-center gap-3 px-2 py-2 rounded-xl hover:bg-slate-800 cursor-pointer transition-colors group">
                <div class="w-8 h-8 rounded-full bg-slate-700 flex items-center justify-center text-xs font-bold text-white ring-2 ring-slate-800 group-hover:ring-slate-700 transition-all">
                    RW
                </div>
                <div class="flex-1 min-w-0">
                    <p class="text-sm font-medium text-white truncate">Rex Wu</p>
                    <p class="text-xs text-slate-500 truncate">Pro Plan</p>
                </div>
                <button class="text-slate-400 hover:text-white p-1 rounded-full hover:bg-slate-700 transition-colors">
                    <span class="icon-[material-symbols--settings] w-5 h-5"></span>
                </button>
            </div>
        </div>
    </aside>

    <!-- Main Content -->
    <main class="flex-1 flex flex-col min-w-0 overflow-hidden bg-slate-950">
        <div class="flex-1 overflow-y-auto p-8 scrollbar-thin scrollbar-thumb-slate-800 scrollbar-track-transparent">
             {@render children()}
        </div>
    </main>
</div>

<style>
    :global(body) {
        margin: 0;
        padding: 0;
        background: #020617;
        color: #e2e8f0;
    }
</style>