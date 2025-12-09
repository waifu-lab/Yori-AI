<script lang="ts">
    let projects = [
        { id: 1, name: 'Yori (Default)', description: 'The default AI companion personality.', status: 'Active', lastEdited: '2 mins ago', avatar: 'bg-indigo-500' },
        { id: 2, name: 'Code Assistant', description: 'Specialized in Rust and Svelte development.', status: 'Inactive', lastEdited: '2 days ago', avatar: 'bg-emerald-500' },
        { id: 3, name: 'Gaming Buddy', description: 'Casual chat and game commentary.', status: 'Inactive', lastEdited: '5 days ago', avatar: 'bg-rose-500' }
    ];
</script>

<div class="max-w-6xl mx-auto">
    <header class="flex items-center justify-between mb-8">
        <div>
            <h1 class="text-3xl font-bold text-white mb-2 tracking-tight">Projects</h1>
            <p class="text-slate-400 text-lg">Manage your AI personas and configurations.</p>
        </div>
        <a href="/new" class="flex items-center gap-2 bg-indigo-600 hover:bg-indigo-500 text-white px-6 py-3 rounded-full font-medium transition-all shadow-lg shadow-indigo-500/20 hover:shadow-indigo-500/30">
            <span class="icon-[material-symbols--add] w-5 h-5"></span>
            New Project
        </a>
    </header>

    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
        {#each projects as project}
            <div class="group bg-slate-900 rounded-xl p-6 border border-slate-800 hover:border-indigo-500/50 transition-all hover:shadow-xl hover:shadow-indigo-500/5 ring-1 ring-white/5">
                <div class="flex items-start justify-between mb-4">
                    <div class="w-12 h-12 rounded-full {project.avatar} flex items-center justify-center text-white font-bold text-lg shadow-lg">
                        {project.name[0]}
                    </div>
                    <button class="text-slate-500 hover:text-white p-2 rounded-full hover:bg-slate-800 transition-colors" aria-label="Project options">
                        <span class="icon-[material-symbols--more-vert] w-5 h-5"></span>
                    </button>
                </div>
                
                <h3 class="text-xl font-bold text-white mb-2 group-hover:text-indigo-400 transition-colors">{project.name}</h3>
                <p class="text-slate-400 text-sm mb-6 line-clamp-2">{project.description}</p>
                
                <div class="flex items-center justify-between pt-4 border-t border-slate-800">
                    <div class="flex items-center gap-2">
                        <span class="w-2 h-2 rounded-full {project.status === 'Active' ? 'bg-green-500 shadow-[0_0_8px_rgba(34,197,94,0.5)]' : 'bg-slate-600'}"></span>
                        <span class="text-xs font-medium text-slate-400">{project.status}</span>
                    </div>
                    <span class="text-xs text-slate-500">{project.lastEdited}</span>
                </div>
            </div>
        {/each}
        
        <!-- New Project Placeholder Card -->
        <a href="/new" class="flex flex-col items-center justify-center p-6 rounded-xl border-2 border-dashed border-slate-800 hover:border-indigo-500/50 hover:bg-slate-900/50 transition-all group cursor-pointer h-full min-h-[200px]">
            <div class="w-12 h-12 rounded-full bg-slate-800 group-hover:bg-indigo-500/20 flex items-center justify-center text-slate-400 group-hover:text-indigo-400 mb-4 transition-colors">
                <span class="icon-[material-symbols--add] w-6 h-6"></span>
            </div>
            <span class="text-slate-400 font-medium group-hover:text-indigo-300 transition-colors">Create New Persona</span>
        </a>
    </div>
</div>
