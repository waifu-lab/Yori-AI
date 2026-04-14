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

<div class="mx-auto max-w-6xl">
	<!-- Header -->
	<header class="mb-8">
		<h1 class="mb-2 text-3xl font-bold tracking-tight text-white">Persona Settings</h1>
		<p class="text-lg text-slate-400">Configure the soul, voice, and appearance of your AI companion.</p>
	</header>

	<div class="mb-8 grid grid-cols-12 gap-6">
		<!-- Column A: Soul Configuration -->
		<div class="col-span-12 space-y-6 lg:col-span-8">
			<div class="rounded-xl border border-slate-800 bg-slate-900 p-6 shadow-sm ring-1 ring-white/5">
				<h2 class="mb-6 flex items-center gap-2 text-xl font-semibold text-white">
					<span class="icon-[material-symbols--psychology] h-6 w-6 text-indigo-400"></span>
					Brain & Logic
				</h2>

				<div class="space-y-6">
					<!-- LLM Provider -->
					<div>
						<label class="mb-2 block text-sm font-medium text-slate-400">Model Provider</label>
						<div class="relative">
							<select
								bind:value={llmProvider}
								class="w-full cursor-pointer appearance-none rounded-full border border-slate-700 bg-slate-950 px-6 py-3 text-slate-200 transition-all outline-none focus:border-indigo-500 focus:ring-1 focus:ring-indigo-500"
							>
								<option>OpenAI</option>
								<option>Anthropic</option>
								<option>Local (Ollama)</option>
							</select>
							<div class="pointer-events-none absolute top-1/2 right-6 -translate-y-1/2 text-slate-500">
								<span class="icon-[material-symbols--expand-more] h-5 w-5"></span>
							</div>
						</div>
					</div>

					<!-- System Prompt -->
					<div>
						<label class="mb-2 block text-sm font-medium text-slate-400">Identity Definition (System Prompt)</label>
						<textarea
							bind:value={systemPrompt}
							class="min-h-[150px] w-full resize-y rounded-2xl border border-slate-700 bg-slate-950 px-6 py-4 font-mono text-sm leading-relaxed text-slate-200 transition-all outline-none focus:border-indigo-500 focus:ring-1 focus:ring-indigo-500"
						></textarea>
					</div>

					<!-- Creativity Slider -->
					<div>
						<div class="mb-2 flex justify-between">
							<label class="text-sm font-medium text-slate-400">Temperature (Creativity)</label>
							<span class="font-mono text-sm text-indigo-400">{creativity}</span>
						</div>
						<input
							type="range"
							min="0"
							max="2"
							step="0.1"
							bind:value={creativity}
							class="h-1 w-full cursor-pointer appearance-none rounded-full bg-slate-800 transition-all [&::-webkit-slider-thumb]:h-4 [&::-webkit-slider-thumb]:w-4 [&::-webkit-slider-thumb]:appearance-none [&::-webkit-slider-thumb]:rounded-full [&::-webkit-slider-thumb]:bg-indigo-500 [&::-webkit-slider-thumb]:shadow-lg [&::-webkit-slider-thumb]:shadow-indigo-500/50 hover:[&::-webkit-slider-thumb]:scale-110"
						/>
						<div class="mt-1 flex justify-between text-xs font-medium text-slate-600">
							<span>Precise</span>
							<span>Balanced</span>
							<span>Creative</span>
						</div>
					</div>
				</div>
			</div>
		</div>

		<!-- Column B: Voice & Audio -->
		<div class="col-span-12 space-y-6 lg:col-span-4">
			<div class="h-full rounded-xl border border-slate-800 bg-slate-900 p-6 shadow-sm ring-1 ring-white/5">
				<h2 class="mb-6 flex items-center gap-2 text-xl font-semibold text-white">
					<span class="icon-[material-symbols--record-voice-over] h-6 w-6 text-indigo-400"></span>
					Voice Synthesis
				</h2>

				<div class="space-y-6">
					<!-- TTS Engine -->
					<div>
						<label class="mb-3 block text-sm font-medium text-slate-400">TTS Engine</label>
						<div class="space-y-2">
							{#each ['VITS (Local)', 'Piper', 'System'] as engine}
								<label
									class="flex cursor-pointer items-center rounded-lg border border-slate-800 p-3 transition-all hover:bg-slate-800/50 has-[:checked]:border-indigo-500 has-[:checked]:bg-indigo-500/10 has-[:checked]:ring-1 has-[:checked]:ring-indigo-500/20"
								>
									<input
										type="radio"
										name="tts"
										value={engine}
										bind:group={ttsEngine}
										class="h-4 w-4 border-slate-600 bg-slate-900 text-indigo-500 focus:ring-indigo-500"
									/>
									<span class="ml-3 text-sm font-medium text-slate-200">{engine}</span>
								</label>
							{/each}
						</div>
					</div>

					<!-- Speed & Pitch -->
					<div class="space-y-4 pt-2">
						<div>
							<div class="mb-2 flex justify-between">
								<label class="text-xs font-medium text-slate-400">Speed</label>
								<span class="font-mono text-xs text-slate-300">{speed}x</span>
							</div>
							<input
								type="range"
								min="0.5"
								max="2"
								step="0.1"
								bind:value={speed}
								class="h-1 w-full cursor-pointer appearance-none rounded-full bg-slate-800 transition-all [&::-webkit-slider-thumb]:h-3 [&::-webkit-slider-thumb]:w-3 [&::-webkit-slider-thumb]:appearance-none [&::-webkit-slider-thumb]:rounded-full [&::-webkit-slider-thumb]:bg-slate-400 hover:[&::-webkit-slider-thumb]:bg-indigo-400"
							/>
						</div>
						<div>
							<div class="mb-2 flex justify-between">
								<label class="text-xs font-medium text-slate-400">Pitch</label>
								<span class="font-mono text-xs text-slate-300">{pitch}</span>
							</div>
							<input
								type="range"
								min="0.5"
								max="2"
								step="0.1"
								bind:value={pitch}
								class="h-1 w-full cursor-pointer appearance-none rounded-full bg-slate-800 transition-all [&::-webkit-slider-thumb]:h-3 [&::-webkit-slider-thumb]:w-3 [&::-webkit-slider-thumb]:appearance-none [&::-webkit-slider-thumb]:rounded-full [&::-webkit-slider-thumb]:bg-slate-400 hover:[&::-webkit-slider-thumb]:bg-indigo-400"
							/>
						</div>
					</div>

					<!-- Preview Button -->
					<button
						class="flex w-full items-center justify-center gap-2 rounded-full border border-indigo-500/20 bg-indigo-500/10 px-4 py-3 font-medium text-indigo-300 transition-all hover:bg-indigo-500/20 active:scale-95"
					>
						<span class="icon-[material-symbols--play-arrow] h-5 w-5"></span>
						Preview Voice
					</button>
				</div>
			</div>
		</div>
	</div>

	<!-- Dynamic Plugin Config -->
	<div class="mb-8 rounded-xl border border-slate-800 bg-slate-900 p-6 shadow-sm ring-1 ring-white/5">
		<div class="mb-6 flex items-center justify-between">
			<h2 class="flex items-center gap-2 text-xl font-semibold text-white">
				<span class="icon-[material-symbols--extension] h-6 w-6 text-indigo-400"></span>
				Plugin Configuration
			</h2>
			<span class="rounded border border-slate-700 bg-slate-800 px-2 py-1 font-mono text-xs text-slate-400"
				>schema: weather-plugin-v1</span
			>
		</div>

		<div class="grid grid-cols-1 gap-8 md:grid-cols-2">
			<!-- Toggle -->
			<div class="flex items-center justify-between rounded-lg border border-slate-800/50 bg-slate-950/50 p-4">
				<div>
					<label class="block text-sm font-medium text-slate-200">Enable Weather Plugin</label>
					<p class="mt-1 text-xs text-slate-500">Allow Yori to access real-time weather data.</p>
				</div>
				<button
					class="relative inline-flex h-6 w-11 items-center rounded-full transition-colors focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2 focus:ring-offset-slate-900 focus:outline-none"
					class:bg-indigo-500={enableWeather}
					class:bg-slate-700={!enableWeather}
					onclick={() => (enableWeather = !enableWeather)}
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
					<input
						type="text"
						bind:value={cacheDir}
						placeholder="/var/tmp/yori/cache"
						class="flex-1 rounded-full border border-slate-700 bg-slate-950 px-6 py-2 font-mono text-sm text-slate-200 outline-none focus:border-indigo-500 focus:ring-1 focus:ring-indigo-500"
					/>
					<button
						class="rounded-full border border-slate-700 bg-slate-800 px-6 py-2 text-sm font-medium text-slate-200 transition-colors hover:bg-slate-700"
					>
						Browse...
					</button>
				</div>
			</div>

			<!-- API Key -->
			<div class="space-y-2 md:col-span-2">
				<label class="block text-sm font-medium text-slate-400">API Key</label>
				<div class="relative">
					<input
						type="password"
						bind:value={apiKey}
						placeholder="sk-..."
						class="w-full rounded-full border border-slate-700 bg-slate-950 px-6 py-2 font-mono text-sm tracking-widest text-slate-200 outline-none focus:border-indigo-500 focus:ring-1 focus:ring-indigo-500"
					/>
					<div class="absolute top-1/2 right-4 -translate-y-1/2 text-slate-500">
						<span class="icon-[material-symbols--key] h-4 w-4"></span>
					</div>
				</div>
				<p class="text-xs text-slate-500">Your key is encrypted locally using AES-256.</p>
			</div>
		</div>
	</div>
</div>
