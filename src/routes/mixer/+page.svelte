<script lang="ts">
	let micVolume = $state(0.8);
	let sysVolume = $state(0.6);
	let noiseGate = $state(-40);
	let compressor = $state(0.5);
</script>

<div class="mx-auto flex h-[calc(100vh-8rem)] max-w-6xl flex-col">
	<header class="mb-8 flex-shrink-0">
		<h1 class="mb-2 text-3xl font-bold tracking-tight text-white">Channel Mixer</h1>
		<p class="text-lg text-slate-400">Audio routing and input configuration.</p>
	</header>

	<div class="grid min-h-0 flex-1 grid-cols-1 gap-6 lg:grid-cols-3">
		<!-- Column 1: Inputs -->
		<div class="flex flex-col gap-6 rounded-xl border border-slate-800 bg-slate-900 p-6 ring-1 ring-white/5">
			<h2 class="flex items-center gap-2 text-xl font-semibold text-white">
				<span class="icon-[material-symbols--mic] h-6 w-6 text-indigo-400"></span>
				Inputs
			</h2>

			<!-- Microphone -->
			<div class="rounded-lg border border-slate-800 bg-slate-950 p-4">
				<div class="mb-4 flex items-center justify-between">
					<label class="w-full text-sm font-medium text-slate-200">
						Microphone
						<select
							class="mt-2 w-full rounded-lg border border-slate-700 bg-slate-900 px-3 py-2 text-sm text-slate-300 outline-none focus:border-indigo-500"
						>
							<option>Default Input (Focusrite USB)</option>
							<option>Microphone (Realtek Audio)</option>
						</select>
					</label>
					<span
						class="ml-2 rounded border border-green-500/20 bg-green-500/10 px-2 py-0.5 text-xs whitespace-nowrap text-green-400"
						>Active</span
					>
				</div>
				<div class="flex items-center gap-3">
					<span class="icon-[material-symbols--volume-up] text-slate-500"></span>
					<input
						type="range"
						min="0"
						max="1"
						step="0.01"
						bind:value={micVolume}
						aria-label="Microphone Volume"
						class="h-1 flex-1 cursor-pointer appearance-none rounded-full bg-slate-800 [&::-webkit-slider-thumb]:h-3 [&::-webkit-slider-thumb]:w-3 [&::-webkit-slider-thumb]:appearance-none [&::-webkit-slider-thumb]:rounded-full [&::-webkit-slider-thumb]:bg-indigo-500"
					/>
				</div>
			</div>

			<!-- System Audio -->
			<div class="rounded-lg border border-slate-800 bg-slate-950 p-4">
				<div class="mb-4 flex items-center justify-between">
					<span class="text-sm font-medium text-slate-200">System Audio</span>
					<button
						class="relative h-5 w-8 rounded-full bg-slate-700 transition-colors hover:bg-slate-600"
						aria-label="Toggle System Audio"
					>
						<span class="absolute top-1 left-1 h-3 w-3 rounded-full bg-white"></span>
					</button>
				</div>
				<div class="flex items-center gap-3">
					<span class="icon-[material-symbols--volume-up] text-slate-500"></span>
					<input
						type="range"
						min="0"
						max="1"
						step="0.01"
						bind:value={sysVolume}
						aria-label="System Audio Volume"
						class="h-1 flex-1 cursor-pointer appearance-none rounded-full bg-slate-800 [&::-webkit-slider-thumb]:h-3 [&::-webkit-slider-thumb]:w-3 [&::-webkit-slider-thumb]:appearance-none [&::-webkit-slider-thumb]:rounded-full [&::-webkit-slider-thumb]:bg-indigo-500"
					/>
				</div>
			</div>
		</div>

		<!-- Column 2: Processing -->
		<div class="flex flex-col gap-6 rounded-xl border border-slate-800 bg-slate-900 p-6 ring-1 ring-white/5">
			<h2 class="flex items-center gap-2 text-xl font-semibold text-white">
				<span class="icon-[material-symbols--graphic-eq] h-6 w-6 text-indigo-400"></span>
				Processing
			</h2>

			<div class="space-y-8">
				<!-- Noise Gate -->
				<div>
					<div class="mb-2 flex justify-between">
						<label class="text-sm font-medium text-slate-300" for="noise-gate">Noise Gate</label>
						<span class="font-mono text-xs text-slate-500">{noiseGate}dB</span>
					</div>
					<input
						id="noise-gate"
						type="range"
						min="-60"
						max="0"
						step="1"
						bind:value={noiseGate}
						class="h-1 w-full cursor-pointer appearance-none rounded-full bg-slate-800 [&::-webkit-slider-thumb]:h-3 [&::-webkit-slider-thumb]:w-3 [&::-webkit-slider-thumb]:appearance-none [&::-webkit-slider-thumb]:rounded-full [&::-webkit-slider-thumb]:bg-indigo-500"
					/>
				</div>

				<!-- Compressor -->
				<div>
					<div class="mb-2 flex justify-between">
						<label class="text-sm font-medium text-slate-300" for="compressor">Compressor</label>
						<span class="font-mono text-xs text-slate-500">{compressor * 100}%</span>
					</div>
					<input
						id="compressor"
						type="range"
						min="0"
						max="1"
						step="0.01"
						bind:value={compressor}
						class="h-1 w-full cursor-pointer appearance-none rounded-full bg-slate-800 [&::-webkit-slider-thumb]:h-3 [&::-webkit-slider-thumb]:w-3 [&::-webkit-slider-thumb]:appearance-none [&::-webkit-slider-thumb]:rounded-full [&::-webkit-slider-thumb]:bg-indigo-500"
					/>
				</div>
			</div>

			<!-- Visualizer Placeholder -->
			<div
				class="flex flex-1 items-end justify-center gap-1 overflow-hidden rounded-lg border border-slate-800 bg-slate-950 p-4"
			>
				{#each Array(12) as _, i}
					<div class="w-3 animate-pulse rounded-t-sm bg-indigo-500/50" style="height: {Math.random() * 100}%"></div>
				{/each}
			</div>
		</div>

		<!-- Column 3: Outputs -->
		<div class="flex flex-col gap-6 rounded-xl border border-slate-800 bg-slate-900 p-6 ring-1 ring-white/5">
			<h2 class="flex items-center gap-2 text-xl font-semibold text-white">
				<span class="icon-[material-symbols--speaker] h-6 w-6 text-indigo-400"></span>
				Outputs
			</h2>

			<div class="rounded-lg border border-slate-800 bg-slate-950 p-4">
				<span class="mb-3 block text-sm font-medium text-slate-200">Virtual Cable (To OBS)</span>
				<div class="h-2 overflow-hidden rounded-full bg-slate-800">
					<div class="h-full w-[70%] bg-gradient-to-r from-green-500 to-yellow-500"></div>
				</div>
				<div class="mt-2 flex justify-between text-xs text-slate-500">
					<span>L</span>
					<span>-12dB</span>
					<span>R</span>
				</div>
			</div>

			<div class="rounded-lg border border-slate-800 bg-slate-950 p-4">
				<label class="mb-3 block text-sm font-medium text-slate-200">
					Monitor (Headphones)
					<select
						class="mt-2 w-full rounded-lg border border-slate-700 bg-slate-900 px-3 py-2 text-sm text-slate-300 outline-none focus:border-indigo-500"
					>
						<option>Default Output</option>
					</select>
				</label>
			</div>
		</div>
	</div>
</div>
