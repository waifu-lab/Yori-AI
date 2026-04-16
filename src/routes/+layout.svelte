<script lang="ts">
	import '../app.css';

	let { children } = $props();
</script>

<svelte:head>
	<title>Yori AI</title>
</svelte:head>

<div class="flex h-screen w-full overflow-hidden bg-slate-950 font-sans text-slate-200">
	<!-- Sidebar -->
	<aside class="flex w-[260px] flex-shrink-0 flex-col border-r border-slate-800 bg-slate-900 p-4">
		<!-- Header -->
		<div class="mb-6 flex items-center gap-3 px-4 py-4">
			<div
				class="flex h-8 w-8 items-center justify-center rounded-lg bg-indigo-500 text-white shadow-lg shadow-indigo-500/20"
			>
				<span class="icon-[material-symbols--smart-toy] h-5 w-5"></span>
			</div>
			<span class="text-xl font-bold tracking-tight text-white">Yori AI</span>
		</div>

		<!-- Navigation -->
		<nav class="flex-1 space-y-1">
			<a
				href="/project"
				class="group flex items-center gap-3 rounded-full px-4 py-3 text-slate-400 transition-colors hover:bg-slate-800 hover:text-slate-200"
			>
				<span class="icon-[material-symbols--folder] h-5 w-5 transition-colors group-hover:text-white"></span>
				<span class="font-medium">Projects</span>
			</a>

			<a
				href="/"
				class="group flex items-center gap-3 rounded-full px-4 py-3 text-slate-400 transition-colors hover:bg-slate-800 hover:text-slate-200"
			>
				<span class="icon-[material-symbols--auto-awesome] h-5 w-5 transition-colors group-hover:text-white"></span>
				<span class="font-medium">Persona & Soul</span>
			</a>

			<a
				href="/mixer"
				class="group flex items-center gap-3 rounded-full px-4 py-3 text-slate-400 transition-colors hover:bg-slate-800 hover:text-slate-200"
			>
				<span class="icon-[material-symbols--settings-input-component] h-5 w-5 transition-colors group-hover:text-white"
				></span>
				<span class="font-medium">Channel Mixer</span>
			</a>

			<a
				href="/plugin"
				class="group flex items-center gap-3 rounded-full px-4 py-3 text-slate-400 transition-colors hover:bg-slate-800 hover:text-slate-200"
			>
				<span class="icon-[material-symbols--extension] h-5 w-5 transition-colors group-hover:text-white"></span>
				<span class="font-medium">Plugin Store</span>
			</a>
		</nav>

		<!-- Footer -->
		<div class="mt-auto border-t border-slate-800 pt-4">
			<div
				class="group flex cursor-pointer items-center gap-3 rounded-xl px-2 py-2 transition-colors hover:bg-slate-800"
			>
				<div
					class="flex h-8 w-8 items-center justify-center rounded-full bg-slate-700 text-xs font-bold text-white ring-2 ring-slate-800 transition-all group-hover:ring-slate-700"
				>
					RW
				</div>
				<div class="min-w-0 flex-1">
					<p class="truncate text-sm font-medium text-white">Rex Wu</p>
					<p class="truncate text-xs text-slate-500">Pro Plan</p>
				</div>
				<button class="rounded-full p-1 text-slate-400 transition-colors hover:bg-slate-700 hover:text-white">
					<span class="icon-[material-symbols--settings] h-5 w-5"></span>
				</button>
			</div>
		</div>
	</aside>

	<!-- Main Content -->
	<main class="flex min-w-0 flex-1 flex-col overflow-hidden bg-slate-950">
		<div class="scrollbar-thin scrollbar-thumb-slate-800 scrollbar-track-transparent flex-1 overflow-y-auto p-8">
			{@render children()}
		</div>
	</main>
</div>

<style>
	:global(body) {
		margin: 0;
		padding: 0;
		background: #020617;
		color: #e2e8f0;
	}
</style>
