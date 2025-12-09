<script lang="ts">
    let micVolume = $state(0.8);
    let sysVolume = $state(0.6);
    let noiseGate = $state(-40);
    let compressor = $state(0.5);
</script>

<div class="max-w-6xl mx-auto h-[calc(100vh-8rem)] flex flex-col">
    <header class="mb-8 flex-shrink-0">
        <h1 class="text-3xl font-bold text-white mb-2 tracking-tight">Channel Mixer</h1>
        <p class="text-slate-400 text-lg">Audio routing and input configuration.</p>
    </header>

    <div class="grid grid-cols-1 lg:grid-cols-3 gap-6 flex-1 min-h-0">
        <!-- Column 1: Inputs -->
        <div class="bg-slate-900 rounded-xl p-6 border border-slate-800 flex flex-col gap-6 ring-1 ring-white/5">
            <h2 class="text-xl font-semibold text-white flex items-center gap-2">
                <span class="icon-[material-symbols--mic] w-6 h-6 text-indigo-400"></span>
                Inputs
            </h2>
            
            <!-- Microphone -->
            <div class="bg-slate-950 rounded-lg p-4 border border-slate-800">
                <div class="flex justify-between items-center mb-4">
                    <label class="text-sm font-medium text-slate-200 w-full">
                        Microphone
                        <select class="w-full mt-2 bg-slate-900 border border-slate-700 rounded-lg px-3 py-2 text-sm text-slate-300 outline-none focus:border-indigo-500">
                            <option>Default Input (Focusrite USB)</option>
                            <option>Microphone (Realtek Audio)</option>
                        </select>
                    </label>
                    <span class="px-2 py-0.5 rounded bg-green-500/10 text-green-400 text-xs border border-green-500/20 ml-2 whitespace-nowrap">Active</span>
                </div>
                <div class="flex items-center gap-3">
                    <span class="icon-[material-symbols--volume-up] text-slate-500"></span>
                    <input type="range" min="0" max="1" step="0.01" bind:value={micVolume} aria-label="Microphone Volume" class="flex-1 h-1 bg-slate-800 rounded-full appearance-none cursor-pointer [&::-webkit-slider-thumb]:appearance-none [&::-webkit-slider-thumb]:w-3 [&::-webkit-slider-thumb]:h-3 [&::-webkit-slider-thumb]:rounded-full [&::-webkit-slider-thumb]:bg-indigo-500">
                </div>
            </div>

            <!-- System Audio -->
            <div class="bg-slate-950 rounded-lg p-4 border border-slate-800">
                <div class="flex justify-between items-center mb-4">
                    <span class="text-sm font-medium text-slate-200">System Audio</span>
                    <button class="w-8 h-5 rounded-full bg-slate-700 relative transition-colors hover:bg-slate-600" aria-label="Toggle System Audio">
                        <span class="absolute left-1 top-1 w-3 h-3 bg-white rounded-full"></span>
                    </button>
                </div>
                <div class="flex items-center gap-3">
                    <span class="icon-[material-symbols--volume-up] text-slate-500"></span>
                    <input type="range" min="0" max="1" step="0.01" bind:value={sysVolume} aria-label="System Audio Volume" class="flex-1 h-1 bg-slate-800 rounded-full appearance-none cursor-pointer [&::-webkit-slider-thumb]:appearance-none [&::-webkit-slider-thumb]:w-3 [&::-webkit-slider-thumb]:h-3 [&::-webkit-slider-thumb]:rounded-full [&::-webkit-slider-thumb]:bg-indigo-500">
                </div>
            </div>
        </div>

        <!-- Column 2: Processing -->
        <div class="bg-slate-900 rounded-xl p-6 border border-slate-800 flex flex-col gap-6 ring-1 ring-white/5">
            <h2 class="text-xl font-semibold text-white flex items-center gap-2">
                <span class="icon-[material-symbols--graphic-eq] w-6 h-6 text-indigo-400"></span>
                Processing
            </h2>

            <div class="space-y-8">
                <!-- Noise Gate -->
                <div>
                    <div class="flex justify-between mb-2">
                        <label class="text-sm font-medium text-slate-300" for="noise-gate">Noise Gate</label>
                        <span class="text-xs font-mono text-slate-500">{noiseGate}dB</span>
                    </div>
                    <input id="noise-gate" type="range" min="-60" max="0" step="1" bind:value={noiseGate} class="w-full h-1 bg-slate-800 rounded-full appearance-none cursor-pointer [&::-webkit-slider-thumb]:appearance-none [&::-webkit-slider-thumb]:w-3 [&::-webkit-slider-thumb]:h-3 [&::-webkit-slider-thumb]:rounded-full [&::-webkit-slider-thumb]:bg-indigo-500">
                </div>

                <!-- Compressor -->
                <div>
                    <div class="flex justify-between mb-2">
                        <label class="text-sm font-medium text-slate-300" for="compressor">Compressor</label>
                        <span class="text-xs font-mono text-slate-500">{compressor * 100}%</span>
                    </div>
                    <input id="compressor" type="range" min="0" max="1" step="0.01" bind:value={compressor} class="w-full h-1 bg-slate-800 rounded-full appearance-none cursor-pointer [&::-webkit-slider-thumb]:appearance-none [&::-webkit-slider-thumb]:w-3 [&::-webkit-slider-thumb]:h-3 [&::-webkit-slider-thumb]:rounded-full [&::-webkit-slider-thumb]:bg-indigo-500">
                </div>
            </div>
            
            <!-- Visualizer Placeholder -->
            <div class="flex-1 bg-slate-950 rounded-lg border border-slate-800 flex items-end justify-center gap-1 p-4 overflow-hidden">
                {#each Array(12) as _, i}
                    <div class="w-3 bg-indigo-500/50 rounded-t-sm animate-pulse" style="height: {Math.random() * 100}%"></div>
                {/each}
            </div>
        </div>

        <!-- Column 3: Outputs -->
        <div class="bg-slate-900 rounded-xl p-6 border border-slate-800 flex flex-col gap-6 ring-1 ring-white/5">
            <h2 class="text-xl font-semibold text-white flex items-center gap-2">
                <span class="icon-[material-symbols--speaker] w-6 h-6 text-indigo-400"></span>
                Outputs
            </h2>

            <div class="bg-slate-950 rounded-lg p-4 border border-slate-800">
                <span class="block text-sm font-medium text-slate-200 mb-3">Virtual Cable (To OBS)</span>
                <div class="h-2 bg-slate-800 rounded-full overflow-hidden">
                    <div class="h-full bg-gradient-to-r from-green-500 to-yellow-500 w-[70%]"></div>
                </div>
                <div class="flex justify-between mt-2 text-xs text-slate-500">
                    <span>L</span>
                    <span>-12dB</span>
                    <span>R</span>
                </div>
            </div>

            <div class="bg-slate-950 rounded-lg p-4 border border-slate-800">
                <label class="block text-sm font-medium text-slate-200 mb-3">
                    Monitor (Headphones)
                    <select class="w-full mt-2 bg-slate-900 border border-slate-700 rounded-lg px-3 py-2 text-sm text-slate-300 outline-none focus:border-indigo-500">
                        <option>Default Output</option>
                    </select>
                </label>
            </div>
        </div>
    </div>
</div>
