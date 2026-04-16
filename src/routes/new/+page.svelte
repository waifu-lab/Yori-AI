<script lang="ts">
	let name = $state('');
	let description = $state('');
	let template = $state('vtuber');
</script>

<div class="mx-auto max-w-2xl py-8">
	<header class="mb-8 text-center">
		<h1 class="mb-2 text-3xl font-bold tracking-tight text-white">Create New Persona</h1>
		<p class="text-slate-400">Start a new AI companion project from a template.</p>
	</header>

	<div class="rounded-xl border border-slate-800 bg-slate-900 p-8 shadow-xl ring-1 ring-white/5">
		<div class="space-y-6">
			<!-- Name -->
			<div>
				<label class="mb-2 block text-sm font-medium text-slate-400">
					Persona Name
					<input
						type="text"
						bind:value={name}
						placeholder="e.g. Yori"
						class="mt-2 w-full rounded-lg border border-slate-700 bg-slate-950 px-4 py-3 text-slate-200 transition-all outline-none focus:border-indigo-500 focus:ring-1 focus:ring-indigo-500"
					/>
				</label>
			</div>

			<!-- Description -->
			<div>
				<label class="mb-2 block text-sm font-medium text-slate-400">
					Description
					<textarea
						bind:value={description}
						placeholder="A brief description of this persona..."
						class="mt-2 min-h-[100px] w-full resize-none rounded-lg border border-slate-700 bg-slate-950 px-4 py-3 text-slate-200 transition-all outline-none focus:border-indigo-500 focus:ring-1 focus:ring-indigo-500"
					></textarea>
				</label>
			</div>

			<!-- Template Selection -->
			<div>
				<span class="mb-3 block text-sm font-medium text-slate-400">Starting Template</span>
				<div class="grid grid-cols-1 gap-4 md:grid-cols-3">
					<button
						class="rounded-xl border p-4 text-left transition-all {template === 'vtuber'
							? 'border-indigo-500 bg-indigo-500/10 ring-1 ring-indigo-500/50'
							: 'border-slate-800 bg-slate-950 hover:border-slate-600'}"
						onclick={() => (template = 'vtuber')}
					>
						<span class="icon-[material-symbols--face] mb-2 block h-6 w-6 text-indigo-400"></span>
						<div class="text-sm font-medium text-white">VTuber</div>
						<div class="mt-1 text-xs text-slate-500">Live2D + Voice</div>
					</button>

					<button
						class="rounded-xl border p-4 text-left transition-all {template === 'assistant'
							? 'border-indigo-500 bg-indigo-500/10 ring-1 ring-indigo-500/50'
							: 'border-slate-800 bg-slate-950 hover:border-slate-600'}"
						onclick={() => (template = 'assistant')}
					>
						<span class="icon-[material-symbols--terminal] mb-2 block h-6 w-6 text-emerald-400"></span>
						<div class="text-sm font-medium text-white">Assistant</div>
						<div class="mt-1 text-xs text-slate-500">Code & Logic</div>
					</button>

					<button
						class="rounded-xl border p-4 text-left transition-all {template === 'chatbot'
							? 'border-indigo-500 bg-indigo-500/10 ring-1 ring-indigo-500/50'
							: 'border-slate-800 bg-slate-950 hover:border-slate-600'}"
						onclick={() => (template = 'chatbot')}
					>
						<span class="icon-[material-symbols--chat] mb-2 block h-6 w-6 text-rose-400"></span>
						<div class="text-sm font-medium text-white">Chatbot</div>
						<div class="mt-1 text-xs text-slate-500">Text Only</div>
					</button>
				</div>
			</div>

			<!-- Actions -->
			<div class="mt-6 flex items-center gap-4 border-t border-slate-800 pt-6">
				<a
					href="/project"
					class="flex-1 rounded-full py-3 text-center font-medium text-slate-400 transition-all hover:bg-slate-800 hover:text-white"
					>Cancel</a
				>
				<button
					class="flex-1 rounded-full bg-indigo-600 py-3 font-medium text-white shadow-lg shadow-indigo-500/20 transition-all hover:bg-indigo-500"
					>Create Persona</button
				>
			</div>
		</div>
	</div>
</div>
