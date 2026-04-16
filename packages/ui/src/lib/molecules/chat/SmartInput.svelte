<script lang="ts">
	interface ModelOption {
		value: string;
		label: string;
	}

	interface Props {
		value?: string;
		placeholder?: string;
		models?: ModelOption[];
		selectedModel?: string;
		disabled?: boolean;
		maxRows?: number;
		class?: string;
		onSend?: (value: string) => void;
		onAttach?: () => void;
		onModelChange?: (model: string) => void;
	}

	let {
		value = $bindable(''),
		placeholder = 'Message Yori...',
		models = [],
		selectedModel = $bindable(''),
		disabled = false,
		maxRows = 6,
		class: className = '',
		onSend,
		onAttach,
		onModelChange
	}: Props = $props();

	let textarea: HTMLTextAreaElement | undefined = $state();
	let showModelPicker = $state(false);

	const selectedModelLabel = $derived(models.find((m) => m.value === selectedModel)?.label ?? 'Model');

	function autoResize() {
		if (!textarea) return;
		textarea.style.height = 'auto';
		const lineHeight = 24;
		const maxHeight = lineHeight * maxRows;
		textarea.style.height = Math.min(textarea.scrollHeight, maxHeight) + 'px';
		textarea.style.overflowY = textarea.scrollHeight > maxHeight ? 'auto' : 'hidden';
	}

	function handleKeydown(e: KeyboardEvent) {
		if (e.key === 'Enter' && !e.shiftKey) {
			e.preventDefault();
			submit();
		}
	}

	function submit() {
		const trimmed = value.trim();
		if (!trimmed || disabled) return;
		onSend?.(trimmed);
		value = '';
		if (textarea) {
			textarea.style.height = 'auto';
		}
	}

	const canSend = $derived(value.trim().length > 0 && !disabled);
</script>

<div class="w-full px-4 pb-4 {className}">
	<div class="bg-surface shadow-float flex flex-col gap-2 rounded-lg p-3">
		<textarea
			bind:this={textarea}
			bind:value
			{placeholder}
			{disabled}
			rows={1}
			oninput={autoResize}
			onkeydown={handleKeydown}
			class="text-foreground placeholder:text-faint w-full resize-none overflow-hidden bg-transparent text-sm leading-6 focus:outline-none disabled:opacity-40"
			style="max-height: {maxRows * 24}px;"
		></textarea>

		<div class="flex items-center justify-between gap-2">
			<div class="flex items-center gap-1">
				<button
					onclick={() => onAttach?.()}
					{disabled}
					class="text-muted-foreground hover:bg-elevated hover:text-foreground duration-fast flex h-8 w-8 items-center justify-center rounded-md transition-colors disabled:pointer-events-none disabled:opacity-40"
					title="Attach file"
				>
					<svg class="h-4 w-4" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.5">
						<path
							d="M13.5 7.5l-5.5 5.5a3.5 3.5 0 0 1-4.95-4.95l5.5-5.5a2 2 0 0 1 2.83 2.83l-5.5 5.5a.5.5 0 0 1-.71-.71l5.5-5.5"
							stroke-linecap="round"
							stroke-linejoin="round"
						/>
					</svg>
				</button>

				<!-- Model selector -->
				{#if models.length > 0}
					<div class="relative">
						<button
							onclick={() => (showModelPicker = !showModelPicker)}
							class="text-muted-foreground hover:bg-elevated hover:text-foreground duration-fast flex h-8 items-center gap-1.5 rounded-md px-3 text-xs font-medium transition-colors"
						>
							{selectedModelLabel}
							<svg class="h-3 w-3" viewBox="0 0 12 12" fill="none" stroke="currentColor" stroke-width="1.5">
								<path d="M3 5l3 3 3-3" stroke-linecap="round" stroke-linejoin="round" />
							</svg>
						</button>

						{#if showModelPicker}
							<!-- svelte-ignore a11y_no_static_element_interactions -->
							<div
								class="bg-surface/90 shadow-float absolute bottom-full left-0 z-50 mb-2 min-w-[160px] rounded-lg p-1 backdrop-blur-[20px]"
								onmouseleave={() => (showModelPicker = false)}
							>
								{#each models as model (model.value)}
									<button
										onclick={() => {
											selectedModel = model.value;
											onModelChange?.(model.value);
											showModelPicker = false;
										}}
										class="duration-fast flex w-full items-center gap-2 rounded-sm px-3 py-2 text-sm transition-colors
                      {selectedModel === model.value
											? 'text-primary bg-elevated'
											: 'text-muted-foreground hover:bg-elevated hover:text-foreground'}"
									>
										{model.label}
										{#if selectedModel === model.value}
											<svg
												class="ml-auto h-3.5 w-3.5"
												viewBox="0 0 14 11"
												fill="none"
												stroke="currentColor"
												stroke-width="1.5"
											>
												<path d="M1 5.5L5 9.5L13 1" stroke-linecap="round" stroke-linejoin="round" />
											</svg>
										{/if}
									</button>
								{/each}
							</div>
						{/if}
					</div>
				{/if}
			</div>

			<button
				onclick={submit}
				disabled={!canSend}
				class="duration-fast flex h-8 w-8 items-center justify-center rounded-md transition-all
          {canSend
					? 'bg-primary shadow-mini text-white hover:brightness-110'
					: 'bg-elevated text-faint cursor-not-allowed'}"
				title="Send (Enter)"
			>
				<svg class="h-4 w-4" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.5">
					<path d="M8 12V4M4 8l4-4 4 4" stroke-linecap="round" stroke-linejoin="round" />
				</svg>
			</button>
		</div>
	</div>

	<p class="text-faint mt-2 text-center text-xs">
		Press <kbd class="bg-elevated text-muted-foreground rounded px-1 py-0.5 text-xs">Enter</kbd> to send,
		<kbd class="bg-elevated text-muted-foreground rounded px-1 py-0.5 text-xs">Shift+Enter</kbd> for new line
	</p>
</div>
