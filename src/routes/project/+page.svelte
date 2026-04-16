<script lang="ts">
	let projects = [
		{
			id: 1,
			name: 'Yori (Default)',
			description: 'The default AI companion personality.',
			status: 'Active',
			lastEdited: '2 mins ago',
			avatar: 'bg-indigo-500'
		},
		{
			id: 2,
			name: 'Code Assistant',
			description: 'Specialized in Rust and Svelte development.',
			status: 'Inactive',
			lastEdited: '2 days ago',
			avatar: 'bg-emerald-500'
		},
		{
			id: 3,
			name: 'Gaming Buddy',
			description: 'Casual chat and game commentary.',
			status: 'Inactive',
			lastEdited: '5 days ago',
			avatar: 'bg-rose-500'
		}
	];
</script>

<div class="mx-auto max-w-6xl">
	<header class="mb-8 flex items-center justify-between">
		<div>
			<h1 class="mb-2 text-3xl font-bold tracking-tight text-white">Projects</h1>
			<p class="text-lg text-slate-400">Manage your AI personas and configurations.</p>
		</div>
		<a
			href="/new"
			class="flex items-center gap-2 rounded-full bg-indigo-600 px-6 py-3 font-medium text-white shadow-lg shadow-indigo-500/20 transition-all hover:bg-indigo-500 hover:shadow-indigo-500/30"
		>
			<span class="icon-[material-symbols--add] h-5 w-5"></span>
			New Project
		</a>
	</header>

	<div class="grid grid-cols-1 gap-6 md:grid-cols-2 lg:grid-cols-3">
		{#each projects as project}
			<div
				class="group rounded-xl border border-slate-800 bg-slate-900 p-6 ring-1 ring-white/5 transition-all hover:border-indigo-500/50 hover:shadow-xl hover:shadow-indigo-500/5"
			>
				<div class="mb-4 flex items-start justify-between">
					<div
						class="h-12 w-12 rounded-full {project.avatar} flex items-center justify-center text-lg font-bold text-white shadow-lg"
					>
						{project.name[0]}
					</div>
					<button
						class="rounded-full p-2 text-slate-500 transition-colors hover:bg-slate-800 hover:text-white"
						aria-label="Project options"
					>
						<span class="icon-[material-symbols--more-vert] h-5 w-5"></span>
					</button>
				</div>

				<h3 class="mb-2 text-xl font-bold text-white transition-colors group-hover:text-indigo-400">{project.name}</h3>
				<p class="mb-6 line-clamp-2 text-sm text-slate-400">{project.description}</p>

				<div class="flex items-center justify-between border-t border-slate-800 pt-4">
					<div class="flex items-center gap-2">
						<span
							class="h-2 w-2 rounded-full {project.status === 'Active'
								? 'bg-green-500 shadow-[0_0_8px_rgba(34,197,94,0.5)]'
								: 'bg-slate-600'}"
						></span>
						<span class="text-xs font-medium text-slate-400">{project.status}</span>
					</div>
					<span class="text-xs text-slate-500">{project.lastEdited}</span>
				</div>
			</div>
		{/each}

		<!-- New Project Placeholder Card -->
		<a
			href="/new"
			class="group flex h-full min-h-[200px] cursor-pointer flex-col items-center justify-center rounded-xl border-2 border-dashed border-slate-800 p-6 transition-all hover:border-indigo-500/50 hover:bg-slate-900/50"
		>
			<div
				class="mb-4 flex h-12 w-12 items-center justify-center rounded-full bg-slate-800 text-slate-400 transition-colors group-hover:bg-indigo-500/20 group-hover:text-indigo-400"
			>
				<span class="icon-[material-symbols--add] h-6 w-6"></span>
			</div>
			<span class="font-medium text-slate-400 transition-colors group-hover:text-indigo-300">Create New Persona</span>
		</a>
	</div>
</div>
