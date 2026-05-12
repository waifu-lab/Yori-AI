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
	const statusColor = $derived(
		status === 'running' ? 'text-primary' : status === 'success' ? 'text-success' : 'text-danger'
	);
</script>
<!-- TODO:目前看起來十分突兀，需要重新設計UI -->
<div
	class="border-border bg-elevated/40 max-w-[80%] overflow-hidden rounded-lg border text-sm {className}"
	role="group"
	aria-label="Tool call: {name}"
>
	<button
		type="button"
		onclick={() => (open = !open)}
		class="hover:bg-elevated/70 duration-fast flex w-full items-center gap-2 px-3 py-2 text-left transition-colors"
		aria-expanded={open}
	>
		<span class="flex h-4 w-4 flex-shrink-0 items-center justify-center" aria-hidden="true">
			{#if status === 'running'}
				<svg class="text-primary h-3.5 w-3.5 animate-spin" viewBox="0 0 16 16" fill="none">
					<circle cx="8" cy="8" r="6" stroke="currentColor" stroke-width="1.5" opacity="0.25" />
					<path d="M14 8a6 6 0 0 0-6-6" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" />
				</svg>
			{:else if status === 'success'}
				<svg class="text-success h-3.5 w-3.5" viewBox="0 0 16 16" fill="none">
					<path d="M3.5 8.5l3 3 6-6.5" stroke="currentColor" stroke-width="1.75" stroke-linecap="round" stroke-linejoin="round" />
				</svg>
			{:else}
				<svg class="text-danger h-3.5 w-3.5" viewBox="0 0 16 16" fill="none">
					<path d="M4 4l8 8M12 4l-8 8" stroke="currentColor" stroke-width="1.75" stroke-linecap="round" />
				</svg>
			{/if}
		</span>

		<svg
			class="text-muted-foreground h-3.5 w-3.5 flex-shrink-0"
			viewBox="0 0 16 16"
			fill="none"
			stroke="currentColor"
			stroke-width="1.5"
			aria-hidden="true"
		>
			<path d="M10.5 2.5l3 3-7 7-3.5.5.5-3.5 7-7z" stroke-linejoin="round" />
		</svg>

		<span class="text-foreground min-w-0 flex-1 truncate font-mono text-xs">{name}</span>

		<span class="text-[10px] uppercase tracking-wide {statusColor}">
			{status}
		</span>

		<svg
			class="text-muted-foreground duration-fast h-3 w-3 flex-shrink-0 transition-transform"
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
		<div class="border-border/60 border-t px-3 py-2.5">
			{#if hasArgs}
				<div class="mb-2">
					<div class="text-muted-foreground mb-1 text-[10px] uppercase tracking-wide">Arguments</div>
					<pre class="bg-canvas/60 text-foreground overflow-x-auto rounded-sm p-2 font-mono text-xs leading-relaxed"><code
						>{argsText}</code
					></pre>
				</div>
			{/if}

			{#if error}
				<div>
					<div class="text-danger mb-1 text-[10px] uppercase tracking-wide">Error</div>
					<pre class="bg-canvas/60 text-danger overflow-x-auto rounded-sm p-2 font-mono text-xs leading-relaxed"><code
						>{error}</code
					></pre>
				</div>
			{:else if hasResult}
				<div>
					<div class="text-muted-foreground mb-1 text-[10px] uppercase tracking-wide">Result</div>
					<pre class="bg-canvas/60 text-foreground overflow-x-auto rounded-sm p-2 font-mono text-xs leading-relaxed"><code
						>{resultText}</code
					></pre>
				</div>
			{:else if status === 'running'}
				<div class="text-muted-foreground text-xs italic">Waiting for result…</div>
			{/if}
		</div>
	{/if}
</div>
