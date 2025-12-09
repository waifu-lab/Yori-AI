<script lang="ts">
    let llmProvider = $state('OpenAI');
    let systemPrompt = $state('You are Yori, a helpful AI assistant...');
    let creativity = $state(0.7);
    let ttsEngine = $state('VITS (Local)');
    let speed = $state(1.0);
    let pitch = $state(1.0);
    
    // Plugin Config
    let enableWeather = $state(false);
    let cacheDir = $state('');
    let apiKey = $state('');
</script>

<div class="max-w-6xl mx-auto">
    <!-- Header -->
    <header class="mb-8">
        <h1 class="text-3xl font-bold text-white mb-2 tracking-tight">Persona Settings</h1>
        <p class="text-slate-400 text-lg">Configure the soul, voice, and appearance of your AI companion.</p>
    </header>

    <div class="grid grid-cols-12 gap-6 mb-8">
        <!-- Column A: Soul Configuration -->
        <div class="col-span-12 lg:col-span-8 space-y-6">
            <div class="bg-slate-900 rounded-xl p-6 border border-slate-800 shadow-sm ring-1 ring-white/5">
                <h2 class="text-xl font-semibold text-white mb-6 flex items-center gap-2">
                    <span class="icon-[material-symbols--psychology] w-6 h-6 text-indigo-400"></span>
                    Brain & Logic
                </h2>
                
                <div class="space-y-6">
                    <!-- LLM Provider -->
                    <div>
                        <label class="block text-sm font-medium text-slate-400 mb-2">Model Provider</label>
                        <div class="relative">
                            <select bind:value={llmProvider} class="w-full bg-slate-950 border border-slate-700 rounded-full px-6 py-3 text-slate-200 focus:border-indigo-500 focus:ring-1 focus:ring-indigo-500 outline-none appearance-none transition-all cursor-pointer">
                                <option>OpenAI</option>
                                <option>Anthropic</option>
                                <option>Local (Ollama)</option>
                            </select>
                            <div class="absolute right-6 top-1/2 -translate-y-1/2 pointer-events-none text-slate-500">
                                <span class="icon-[material-symbols--expand-more] w-5 h-5"></span>
                            </div>
                        </div>
                    </div>

                    <!-- System Prompt -->
                    <div>
                        <label class="block text-sm font-medium text-slate-400 mb-2">Identity Definition (System Prompt)</label>
                        <textarea bind:value={systemPrompt} class="w-full bg-slate-950 border border-slate-700 rounded-2xl px-6 py-4 text-slate-200 font-mono text-sm focus:border-indigo-500 focus:ring-1 focus:ring-indigo-500 outline-none transition-all min-h-[150px] resize-y leading-relaxed"></textarea>
                    </div>

                    <!-- Creativity Slider -->
                    <div>
                        <div class="flex justify-between mb-2">
                            <label class="text-sm font-medium text-slate-400">Temperature (Creativity)</label>
                            <span class="text-sm font-mono text-indigo-400">{creativity}</span>
                        </div>
                        <input type="range" min="0" max="2" step="0.1" bind:value={creativity} class="w-full h-1 bg-slate-800 rounded-full appearance-none cursor-pointer [&::-webkit-slider-thumb]:appearance-none [&::-webkit-slider-thumb]:w-4 [&::-webkit-slider-thumb]:h-4 [&::-webkit-slider-thumb]:rounded-full [&::-webkit-slider-thumb]:bg-indigo-500 [&::-webkit-slider-thumb]:shadow-lg [&::-webkit-slider-thumb]:shadow-indigo-500/50 hover:[&::-webkit-slider-thumb]:scale-110 transition-all">
                        <div class="flex justify-between mt-1 text-xs text-slate-600 font-medium">
                            <span>Precise</span>
                            <span>Balanced</span>
                            <span>Creative</span>
                        </div>
                    </div>
                </div>
            </div>
        </div>

        <!-- Column B: Voice & Audio -->
        <div class="col-span-12 lg:col-span-4 space-y-6">
            <div class="bg-slate-900 rounded-xl p-6 border border-slate-800 shadow-sm ring-1 ring-white/5 h-full">
                <h2 class="text-xl font-semibold text-white mb-6 flex items-center gap-2">
                    <span class="icon-[material-symbols--record-voice-over] w-6 h-6 text-indigo-400"></span>
                    Voice Synthesis
                </h2>

                <div class="space-y-6">
                    <!-- TTS Engine -->
                    <div>
                        <label class="block text-sm font-medium text-slate-400 mb-3">TTS Engine</label>
                        <div class="space-y-2">
                            {#each ['VITS (Local)', 'Piper', 'System'] as engine}
                                <label class="flex items-center p-3 border border-slate-800 rounded-lg cursor-pointer hover:bg-slate-800/50 transition-all has-[:checked]:border-indigo-500 has-[:checked]:bg-indigo-500/10 has-[:checked]:ring-1 has-[:checked]:ring-indigo-500/20">
                                    <input type="radio" name="tts" value={engine} bind:group={ttsEngine} class="w-4 h-4 text-indigo-500 border-slate-600 focus:ring-indigo-500 bg-slate-900">
                                    <span class="ml-3 text-sm font-medium text-slate-200">{engine}</span>
                                </label>
                            {/each}
                        </div>
                    </div>

                    <!-- Speed & Pitch -->
                    <div class="space-y-4 pt-2">
                        <div>
                            <div class="flex justify-between mb-2">
                                <label class="text-xs font-medium text-slate-400">Speed</label>
                                <span class="text-xs font-mono text-slate-300">{speed}x</span>
                            </div>
                            <input type="range" min="0.5" max="2" step="0.1" bind:value={speed} class="w-full h-1 bg-slate-800 rounded-full appearance-none cursor-pointer [&::-webkit-slider-thumb]:appearance-none [&::-webkit-slider-thumb]:w-3 [&::-webkit-slider-thumb]:h-3 [&::-webkit-slider-thumb]:rounded-full [&::-webkit-slider-thumb]:bg-slate-400 hover:[&::-webkit-slider-thumb]:bg-indigo-400 transition-all">
                        </div>
                        <div>
                            <div class="flex justify-between mb-2">
                                <label class="text-xs font-medium text-slate-400">Pitch</label>
                                <span class="text-xs font-mono text-slate-300">{pitch}</span>
                            </div>
                            <input type="range" min="0.5" max="2" step="0.1" bind:value={pitch} class="w-full h-1 bg-slate-800 rounded-full appearance-none cursor-pointer [&::-webkit-slider-thumb]:appearance-none [&::-webkit-slider-thumb]:w-3 [&::-webkit-slider-thumb]:h-3 [&::-webkit-slider-thumb]:rounded-full [&::-webkit-slider-thumb]:bg-slate-400 hover:[&::-webkit-slider-thumb]:bg-indigo-400 transition-all">
                        </div>
                    </div>

                    <!-- Preview Button -->
                    <button class="w-full flex items-center justify-center gap-2 bg-indigo-500/10 hover:bg-indigo-500/20 text-indigo-300 font-medium py-3 px-4 rounded-full transition-all active:scale-95 border border-indigo-500/20">
                        <span class="icon-[material-symbols--play-arrow] w-5 h-5"></span>
                        Preview Voice
                    </button>
                </div>
            </div>
        </div>
    </div>

    <!-- Dynamic Plugin Config -->
    <div class="bg-slate-900 rounded-xl p-6 border border-slate-800 shadow-sm ring-1 ring-white/5 mb-8">
        <div class="flex items-center justify-between mb-6">
            <h2 class="text-xl font-semibold text-white flex items-center gap-2">
                <span class="icon-[material-symbols--extension] w-6 h-6 text-indigo-400"></span>
                Plugin Configuration
            </h2>
            <span class="px-2 py-1 rounded bg-slate-800 text-xs font-mono text-slate-400 border border-slate-700">schema: weather-plugin-v1</span>
        </div>

        <div class="grid grid-cols-1 md:grid-cols-2 gap-8">
            <!-- Toggle -->
            <div class="flex items-center justify-between p-4 bg-slate-950/50 rounded-lg border border-slate-800/50">
                <div>
                    <label class="block text-sm font-medium text-slate-200">Enable Weather Plugin</label>
                    <p class="text-xs text-slate-500 mt-1">Allow Yori to access real-time weather data.</p>
                </div>
                <button 
                    class="relative inline-flex h-6 w-11 items-center rounded-full transition-colors focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2 focus:ring-offset-slate-900"
                    class:bg-indigo-500={enableWeather}
                    class:bg-slate-700={!enableWeather}
                    onclick={() => enableWeather = !enableWeather}
                >
                    <span class="sr-only">Enable weather</span>
                    <span 
                        class="inline-block h-4 w-4 transform rounded-full bg-white transition-transform"
                        class:translate-x-6={enableWeather}
                        class:translate-x-1={!enableWeather}
                    ></span>
                </button>
            </div>

            <!-- Path Input -->
            <div class="space-y-2">
                <label class="block text-sm font-medium text-slate-400">Cache Directory</label>
                <div class="flex gap-2">
                    <input type="text" bind:value={cacheDir} placeholder="/var/tmp/yori/cache" class="flex-1 bg-slate-950 border border-slate-700 rounded-full px-6 py-2 text-slate-200 text-sm focus:border-indigo-500 focus:ring-1 focus:ring-indigo-500 outline-none font-mono">
                    <button class="px-6 py-2 bg-slate-800 hover:bg-slate-700 text-slate-200 text-sm font-medium rounded-full border border-slate-700 transition-colors">
                        Browse...
                    </button>
                </div>
            </div>

            <!-- API Key -->
            <div class="space-y-2 md:col-span-2">
                <label class="block text-sm font-medium text-slate-400">API Key</label>
                <div class="relative">
                    <input type="password" bind:value={apiKey} placeholder="sk-..." class="w-full bg-slate-950 border border-slate-700 rounded-full px-6 py-2 text-slate-200 text-sm focus:border-indigo-500 focus:ring-1 focus:ring-indigo-500 outline-none font-mono tracking-widest">
                    <div class="absolute right-4 top-1/2 -translate-y-1/2 text-slate-500">
                        <span class="icon-[material-symbols--key] w-4 h-4"></span>
                    </div>
                </div>
                <p class="text-xs text-slate-500">Your key is encrypted locally using AES-256.</p>
            </div>
        </div>
    </div>
</div>
