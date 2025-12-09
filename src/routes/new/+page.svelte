<script lang="ts">
    let name = $state('');
    let description = $state('');
    let template = $state('vtuber');
</script>

<div class="max-w-2xl mx-auto py-8">
    <header class="mb-8 text-center">
        <h1 class="text-3xl font-bold text-white mb-2 tracking-tight">Create New Persona</h1>
        <p class="text-slate-400">Start a new AI companion project from a template.</p>
    </header>

    <div class="bg-slate-900 rounded-xl p-8 border border-slate-800 shadow-xl ring-1 ring-white/5">
        <div class="space-y-6">
            <!-- Name -->
            <div>
                <label class="block text-sm font-medium text-slate-400 mb-2">
                    Persona Name
                    <input type="text" bind:value={name} placeholder="e.g. Yori" class="w-full mt-2 bg-slate-950 border border-slate-700 rounded-lg px-4 py-3 text-slate-200 focus:border-indigo-500 focus:ring-1 focus:ring-indigo-500 outline-none transition-all">
                </label>
            </div>

            <!-- Description -->
            <div>
                <label class="block text-sm font-medium text-slate-400 mb-2">
                    Description
                    <textarea bind:value={description} placeholder="A brief description of this persona..." class="w-full mt-2 bg-slate-950 border border-slate-700 rounded-lg px-4 py-3 text-slate-200 focus:border-indigo-500 focus:ring-1 focus:ring-indigo-500 outline-none transition-all min-h-[100px] resize-none"></textarea>
                </label>
            </div>

            <!-- Template Selection -->
            <div>
                <span class="block text-sm font-medium text-slate-400 mb-3">Starting Template</span>
                <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
                    <button 
                        class="p-4 rounded-xl border text-left transition-all {template === 'vtuber' ? 'bg-indigo-500/10 border-indigo-500 ring-1 ring-indigo-500/50' : 'bg-slate-950 border-slate-800 hover:border-slate-600'}"
                        onclick={() => template = 'vtuber'}
                    >
                        <span class="icon-[material-symbols--face] w-6 h-6 text-indigo-400 mb-2 block"></span>
                        <div class="font-medium text-white text-sm">VTuber</div>
                        <div class="text-xs text-slate-500 mt-1">Live2D + Voice</div>
                    </button>

                    <button 
                        class="p-4 rounded-xl border text-left transition-all {template === 'assistant' ? 'bg-indigo-500/10 border-indigo-500 ring-1 ring-indigo-500/50' : 'bg-slate-950 border-slate-800 hover:border-slate-600'}"
                        onclick={() => template = 'assistant'}
                    >
                        <span class="icon-[material-symbols--terminal] w-6 h-6 text-emerald-400 mb-2 block"></span>
                        <div class="font-medium text-white text-sm">Assistant</div>
                        <div class="text-xs text-slate-500 mt-1">Code & Logic</div>
                    </button>

                    <button 
                        class="p-4 rounded-xl border text-left transition-all {template === 'chatbot' ? 'bg-indigo-500/10 border-indigo-500 ring-1 ring-indigo-500/50' : 'bg-slate-950 border-slate-800 hover:border-slate-600'}"
                        onclick={() => template = 'chatbot'}
                    >
                        <span class="icon-[material-symbols--chat] w-6 h-6 text-rose-400 mb-2 block"></span>
                        <div class="font-medium text-white text-sm">Chatbot</div>
                        <div class="text-xs text-slate-500 mt-1">Text Only</div>
                    </button>
                </div>
            </div>

            <!-- Actions -->
            <div class="flex items-center gap-4 pt-6 mt-6 border-t border-slate-800">
                <a href="/project" class="flex-1 py-3 rounded-full font-medium text-slate-400 hover:text-white hover:bg-slate-800 transition-all text-center">Cancel</a>
                <button class="flex-1 bg-indigo-600 hover:bg-indigo-500 text-white py-3 rounded-full font-medium transition-all shadow-lg shadow-indigo-500/20">Create Persona</button>
            </div>
        </div>
    </div>
</div>
