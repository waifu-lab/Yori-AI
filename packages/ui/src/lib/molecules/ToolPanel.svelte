<script lang="ts">
	import type { Snippet } from 'svelte';

	interface Props {
		open: boolean;
		title?: string;
		onClose?: () => void;
		children?: Snippet;
		class?: string;
	}

	let { open = $bindable(false), title = '工具記錄', onClose, children, class: className = '' }: Props = $props();
</script>

<!-- Backdrop -->
{#if open}
	<div
		role="presentation"
		class="fixed inset-0 z-40 bg-black/20 backdrop-blur-[1px]"
		onclick={onClose}
	></div>
{/if}

<!-- Drawer -->
<div
	class="fixed right-0 top-0 z-50 flex h-full w-80 flex-col bg-canvas shadow-float
		transition-transform duration-slow ease-in-out
		{open ? 'translate-x-0' : 'translate-x-full'}
		{className}"
>
	<!-- Header -->
	<div class="border-border flex flex-shrink-0 items-center justify-between border-b px-4 py-3">
		<h2 class="text-foreground text-sm font-semibold">{title}</h2>
		<button
			type="button"
			onclick={onClose}
			class="duration-fast text-muted-foreground hover:bg-elevated hover:text-foreground flex h-7 w-7 items-center
				justify-center rounded-md transition-colors"
			aria-label="關閉面板"
		>
			<svg class="h-4 w-4" viewBox="0 0 16 16" fill="none">
				<path
					d="M4 4l8 8M12 4l-8 8"
					stroke="currentColor"
					stroke-width="1.5"
					stroke-linecap="round"
				/>
			</svg>
		</button>
	</div>

	<!-- Content -->
	<div class="flex-1 overflow-y-auto p-4">
		{#if children}
			{@render children()}
		{/if}
	</div>
</div>
