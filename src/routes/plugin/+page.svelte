<script lang="ts">
    let searchQuery = $state('');
    let activeTab = $state('All');
    
    const plugins = [
        { id: 'weather', name: 'Weather Data', author: 'Yori Team', version: '1.2.0', description: 'Real-time weather information integration.', installed: true, icon: 'icon-[material-symbols--sunny]' },
        { id: 'spotify', name: 'Spotify Connect', author: 'Community', version: '0.9.5', description: 'Control music playback and display current track.', installed: false, icon: 'icon-[material-symbols--music-note]' },
        { id: 'obs', name: 'OBS WebSocket', author: 'StreamTools', version: '2.1.0', description: 'Control OBS scenes and sources directly.', installed: false, icon: 'icon-[material-symbols--videocam]' },
        { id: 'memory', name: 'Long-term Memory', author: 'Yori Team', version: '1.0.0', description: 'Vector database integration for persistent memories.', installed: true, icon: 'icon-[material-symbols--history]' },
        { id: 'discord', name: 'Discord Bot', author: 'ChatDev', version: '1.5.2', description: 'Connect to Discord channels for chat integration.', installed: false, icon: 'icon-[material-symbols--chat]' },
    ];
</script>

<div class="max-w-6xl mx-auto">
    <header class="mb-8">
        <h1 class="text-3xl font-bold text-white mb-2 tracking-tight">Plugin Store</h1>
        <p class="text-slate-400 text-lg">Extend Yori with new capabilities and integrations.</p>
    </header>

    <!-- Search & Filter -->
    <div class="flex flex-col md:flex-row gap-4 mb-8">
        <div class="relative flex-1">
            <span class="absolute left-4 top-1/2 -translate-y-1/2 text-slate-500 icon-[material-symbols--search] w-5 h-5"></span>
            <input type="text" bind:value={searchQuery} placeholder="Search plugins..." class="w-full bg-slate-900 border border-slate-800 rounded-full pl-12 pr-6 py-3 text-slate-200 focus:border-indigo-500 focus:ring-1 focus:ring-indigo-500 outline-none transition-all">
        </div>
        <div class="flex bg-slate-900 rounded-full p-1 border border-slate-800">
            {#each ['All', 'Installed', 'Updates'] as tab}
                <button 
                    class="px-6 py-2 rounded-full text-sm font-medium transition-all {activeTab === tab ? 'bg-indigo-500 text-white shadow-lg shadow-indigo-500/25' : 'text-slate-400 hover:text-white'}"
                    onclick={() => activeTab = tab}
                >
                    {tab}
                </button>
            {/each}
        </div>
    </div>

    <!-- Grid -->
    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
        {#each plugins as plugin}
            <div class="bg-slate-900 rounded-xl p-6 border border-slate-800 hover:border-indigo-500/30 transition-all hover:shadow-lg hover:shadow-indigo-500/5 ring-1 ring-white/5 flex flex-col h-full">
                <div class="flex items-start justify-between mb-4">
                    <div class="w-12 h-12 rounded-xl bg-slate-800 flex items-center justify-center text-indigo-400 ring-1 ring-white/10">
                        <span class="{plugin.icon} w-6 h-6"></span>
                    </div>
                    {#if plugin.installed}
                        <span class="px-2 py-1 rounded bg-green-500/10 text-green-400 text-xs font-medium border border-green-500/20">Installed</span>
                    {/if}
                </div>
                
                <h3 class="text-lg font-bold text-white mb-1">{plugin.name}</h3>
                <p class="text-xs text-slate-500 mb-3">by {plugin.author} • v{plugin.version}</p>
                <p class="text-slate-400 text-sm mb-6 flex-1">{plugin.description}</p>
                
                <button class="w-full py-2.5 rounded-full font-medium text-sm transition-all {plugin.installed ? 'bg-slate-800 text-slate-300 hover:bg-slate-700' : 'bg-indigo-600 text-white hover:bg-indigo-500 shadow-lg shadow-indigo-500/20'}">
                    {plugin.installed ? 'Configure' : 'Install'}
                </button>
            </div>
        {/each}
    </div>
</div>
