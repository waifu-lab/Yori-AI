<script lang="ts">
	import { Dialog } from 'bits-ui';
	import type { Snippet } from 'svelte';

	type Size = 'sm' | 'md' | 'lg';

	interface Props {
		open?: boolean;
		title?: string;
		description?: string;
		size?: Size;
		trigger?: Snippet;
		children: Snippet;
		footer?: Snippet;
		onOpenChange?: (open: boolean) => void;
	}

	let {
		open = $bindable(false),
		title,
		description,
		size = 'md',
		trigger,
		children,
		footer,
		onOpenChange
	}: Props = $props();

	const sizes: Record<Size, string> = {
		sm: 'max-w-sm',
		md: 'max-w-lg',
		lg: 'max-w-2xl'
	};
</script>

<Dialog.Root bind:open {onOpenChange}>
	{#if trigger}
		<Dialog.Trigger>{@render trigger()}</Dialog.Trigger>
	{/if}

	<Dialog.Portal>
		<Dialog.Overlay
			class="data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=open]:fade-in-0 data-[state=closed]:fade-out-0 fixed
        inset-0 z-50
        bg-black/60 backdrop-blur-[24px]"
		/>

		<Dialog.Content
			class="fixed top-1/2 left-1/2 z-50 w-full -translate-x-1/2 -translate-y-1/2 p-4
        {sizes[size]}
        data-[state=open]:animate-in data-[state=closed]:animate-out
        data-[state=open]:fade-in-0 data-[state=closed]:fade-out-0
        data-[state=open]:zoom-in-95 data-[state=closed]:zoom-out-95
        data-[state=open]:slide-in-from-top-2 data-[state=closed]:slide-out-to-top-2
        focus:outline-none"
		>
			<div class="bg-surface/90 shadow-float flex flex-col gap-4 rounded-lg p-6 backdrop-blur-[20px]">
				{#if title || description}
					<div class="flex items-start justify-between gap-4">
						<div class="flex flex-col gap-1">
							{#if title}
								<Dialog.Title class="text-foreground font-[Space_Grotesk] text-lg font-bold tracking-[-0.04em]">
									{title}
								</Dialog.Title>
							{/if}
							{#if description}
								<Dialog.Description class="text-muted-foreground font-[Newsreader] text-sm leading-relaxed">
									{description}
								</Dialog.Description>
							{/if}
						</div>
						<Dialog.Close
							class="text-muted-foreground hover:bg-elevated hover:text-secondary duration-fast flex h-7 w-7
                shrink-0 items-center justify-center rounded-md transition-colors"
						>
							<svg class="h-4 w-4" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.5">
								<path d="M4 4l8 8M12 4l-8 8" stroke-linecap="round" />
							</svg>
						</Dialog.Close>
					</div>
				{/if}

				<div class="text-foreground text-sm">{@render children()}</div>

				{#if footer}
					<div class="flex items-center justify-end gap-2 pt-2">{@render footer()}</div>
				{/if}
			</div>
		</Dialog.Content>
	</Dialog.Portal>
</Dialog.Root>
