<script lang="ts">
	type Status = 'running' | 'success' | 'error';

	interface Props {
		name: string;
		status?: Status;
		args?: string | Record<string, unknown> | null;
		result?: unknown;
		error?: string;
		defaultOpen?: boolean;
		class?: string;
	}

	let {
		name,
		status = 'running',
		args = null,
		result = undefined,
		error = undefined,
		defaultOpen = false,
		class: className = ''
	}: Props = $props();

	let open = $state(defaultOpen);

	function format(value: unknown): string {
		if (value == null) return '';
		if (typeof value === 'string') {
			try {
				return JSON.stringify(JSON.parse(value), null, 2);
			} catch {
				return value;
			}
		}
		try {
			return JSON.stringify(value, null, 2);
		} catch {
			return String(value);
		}
	}

	const argsText = $derived(format(args));
	const resultText = $derived(format(result));
	const hasArgs = $derived(argsText.length > 0);
	const hasResult = $derived(resultText.length > 0 || !!error);
</script>

<div class="my-1 max-w-[80%] text-sm {className}" role="group" aria-label="Tool call: {name}">
	<button
		type="button"
		onclick={() => (open = !open)}
		class="text-muted-foreground hover:text-foreground duration-fast flex items-center gap-1.5 text-left transition-colors"
		aria-expanded={open}
	>
		<span class="flex h-3.5 w-3.5 flex-shrink-0 items-center justify-center" aria-hidden="true">
			{#if status === 'running'}
				<svg class="text-muted-foreground h-3 w-3 animate-spin" viewBox="0 0 16 16" fill="none">
					<circle cx="8" cy="8" r="6" stroke="currentColor" stroke-width="1.5" opacity="0.3" />
					<path d="M14 8a6 6 0 0 0-6-6" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" />
				</svg>
			{:else if status === 'success'}
				<svg class="text-muted-foreground h-3 w-3" viewBox="0 0 16 16" fill="none">
					<path
						d="M3.5 8.5l3 3 6-6.5"
						stroke="currentColor"
						stroke-width="1.75"
						stroke-linecap="round"
						stroke-linejoin="round"
					/>
				</svg>
			{:else}
				<svg class="text-danger/70 h-3 w-3" viewBox="0 0 16 16" fill="none">
					<path d="M4 4l8 8M12 4l-8 8" stroke="currentColor" stroke-width="1.75" stroke-linecap="round" />
				</svg>
			{/if}
		</span>

		<span class="font-mono text-xs">{name}</span>

		<svg
			class="duration-fast h-3 w-3 flex-shrink-0 opacity-50 transition-transform"
			class:rotate-180={open}
			viewBox="0 0 16 16"
			fill="none"
			stroke="currentColor"
			stroke-width="1.5"
			aria-hidden="true"
		>
			<path d="M4 6l4 4 4-4" stroke-linecap="round" stroke-linejoin="round" />
		</svg>
	</button>

	{#if open}
		<div class="bg-surface mt-1.5 rounded-md px-3 py-2.5">
			{#if hasArgs}
				<div class:mb-3={hasResult || !!error}>
					<div class="text-faint mb-1 text-[10px] tracking-wide uppercase">Arguments</div>
					<pre class="text-muted-foreground overflow-x-auto font-mono text-xs leading-relaxed"><code>{argsText}</code
						></pre>
				</div>
			{/if}

			{#if error}
				<div>
					<div class="text-faint mb-1 text-[10px] tracking-wide uppercase">Error</div>
					<pre class="text-danger/80 overflow-x-auto font-mono text-xs leading-relaxed"><code>{error}</code></pre>
				</div>
			{:else if hasResult}
				<div>
					<div class="text-faint mb-1 text-[10px] tracking-wide uppercase">Result</div>
					<pre class="text-muted-foreground overflow-x-auto font-mono text-xs leading-relaxed"><code>{resultText}</code
						></pre>
				</div>
			{:else if status === 'running'}
				<span class="text-faint text-xs italic">Running…</span>
			{/if}
		</div>
	{/if}
</div>
