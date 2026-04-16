<script lang="ts">
	let searchQuery = $state('');
	let activeTab = $state('All');

	const plugins = [
		{
			id: 'weather',
			name: 'Weather Data',
			author: 'Yori Team',
			version: '1.2.0',
			description: 'Real-time weather information integration.',
			installed: true,
			icon: 'icon-[material-symbols--sunny]'
		},
		{
			id: 'spotify',
			name: 'Spotify Connect',
			author: 'Community',
			version: '0.9.5',
			description: 'Control music playback and display current track.',
			installed: false,
			icon: 'icon-[material-symbols--music-note]'
		},
		{
			id: 'obs',
			name: 'OBS WebSocket',
			author: 'StreamTools',
			version: '2.1.0',
			description: 'Control OBS scenes and sources directly.',
			installed: false,
			icon: 'icon-[material-symbols--videocam]'
		},
		{
			id: 'memory',
			name: 'Long-term Memory',
			author: 'Yori Team',
			version: '1.0.0',
			description: 'Vector database integration for persistent memories.',
			installed: true,
			icon: 'icon-[material-symbols--history]'
		},
		{
			id: 'discord',
			name: 'Discord Bot',
			author: 'ChatDev',
			version: '1.5.2',
			description: 'Connect to Discord channels for chat integration.',
			installed: false,
			icon: 'icon-[material-symbols--chat]'
		}
	];
</script>

<div class="mx-auto max-w-6xl">
	<header class="mb-8">
		<h1 class="mb-2 text-3xl font-bold tracking-tight text-white">Plugin Store</h1>
		<p class="text-lg text-slate-400">Extend Yori with new capabilities and integrations.</p>
	</header>

	<!-- Search & Filter -->
	<div class="mb-8 flex flex-col gap-4 md:flex-row">
		<div class="relative flex-1">
			<span class="icon-[material-symbols--search] absolute top-1/2 left-4 h-5 w-5 -translate-y-1/2 text-slate-500"
			></span>
			<input
				type="text"
				bind:value={searchQuery}
				placeholder="Search plugins..."
				class="w-full rounded-full border border-slate-800 bg-slate-900 py-3 pr-6 pl-12 text-slate-200 transition-all outline-none focus:border-indigo-500 focus:ring-1 focus:ring-indigo-500"
			/>
		</div>
		<div class="flex rounded-full border border-slate-800 bg-slate-900 p-1">
			{#each ['All', 'Installed', 'Updates'] as tab}
				<button
					class="rounded-full px-6 py-2 text-sm font-medium transition-all {activeTab === tab
						? 'bg-indigo-500 text-white shadow-lg shadow-indigo-500/25'
						: 'text-slate-400 hover:text-white'}"
					onclick={() => (activeTab = tab)}
				>
					{tab}
				</button>
			{/each}
		</div>
	</div>

	<!-- Grid -->
	<div class="grid grid-cols-1 gap-6 md:grid-cols-2 lg:grid-cols-3">
		{#each plugins as plugin}
			<div
				class="flex h-full flex-col rounded-xl border border-slate-800 bg-slate-900 p-6 ring-1 ring-white/5 transition-all hover:border-indigo-500/30 hover:shadow-lg hover:shadow-indigo-500/5"
			>
				<div class="mb-4 flex items-start justify-between">
					<div
						class="flex h-12 w-12 items-center justify-center rounded-xl bg-slate-800 text-indigo-400 ring-1 ring-white/10"
					>
						<span class="{plugin.icon} h-6 w-6"></span>
					</div>
					{#if plugin.installed}
						<span
							class="rounded border border-green-500/20 bg-green-500/10 px-2 py-1 text-xs font-medium text-green-400"
							>Installed</span
						>
					{/if}
				</div>

				<h3 class="mb-1 text-lg font-bold text-white">{plugin.name}</h3>
				<p class="mb-3 text-xs text-slate-500">by {plugin.author} • v{plugin.version}</p>
				<p class="mb-6 flex-1 text-sm text-slate-400">{plugin.description}</p>

				<button
					class="w-full rounded-full py-2.5 text-sm font-medium transition-all {plugin.installed
						? 'bg-slate-800 text-slate-300 hover:bg-slate-700'
						: 'bg-indigo-600 text-white shadow-lg shadow-indigo-500/20 hover:bg-indigo-500'}"
				>
					{plugin.installed ? 'Configure' : 'Install'}
				</button>
			</div>
		{/each}
	</div>
</div>
