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
		files?: File[];
		accept?: string;
		multiple?: boolean;
		onSend?: (value: string, files?: File[]) => void;
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
		files = $bindable([]),
		accept = '',
		multiple = true,
		onSend,
		onModelChange
	}: Props = $props();

	let textarea: HTMLTextAreaElement | undefined = $state();
	let fileInput: HTMLInputElement | undefined = $state();
	let showModelPicker = $state(false);
	let isDragging = $state(false);
	let dragDepth = 0;

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
		if ((!trimmed && files.length === 0) || disabled) return;
		onSend?.(trimmed, files.length ? files.slice() : undefined);
		value = '';
		files = [];
		if (textarea) {
			textarea.style.height = 'auto';
		}
	}

	function addFiles(incoming: FileList | File[] | null | undefined) {
		if (!incoming) return;
		const arr = Array.from(incoming);
		if (!arr.length) return;
		files = multiple ? [...files, ...arr] : [arr[0]];
	}

	function removeFile(index: number) {
		files = files.filter((_, i) => i !== index);
	}

	function openPicker() {
		fileInput?.click();
	}

	function onFileInputChange(e: Event) {
		const input = e.target as HTMLInputElement;
		addFiles(input.files);
		input.value = '';
	}

	function onDragEnter(e: DragEvent) {
		if (disabled) return;
		if (!e.dataTransfer?.types.includes('Files')) return;
		dragDepth++;
		isDragging = true;
	}

	function onDragOver(e: DragEvent) {
		if (disabled) return;
		if (!e.dataTransfer?.types.includes('Files')) return;
		e.preventDefault();
		if (e.dataTransfer) e.dataTransfer.dropEffect = 'copy';
	}

	function onDragLeave() {
		dragDepth = Math.max(0, dragDepth - 1);
		if (dragDepth === 0) isDragging = false;
	}

	function onDrop(e: DragEvent) {
		if (disabled) return;
		e.preventDefault();
		dragDepth = 0;
		isDragging = false;
		addFiles(e.dataTransfer?.files);
	}

	function formatSize(bytes: number) {
		if (bytes < 1024) return `${bytes} B`;
		if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`;
		return `${(bytes / (1024 * 1024)).toFixed(1)} MB`;
	}

	const canSend = $derived((value.trim().length > 0 || files.length > 0) && !disabled);
</script>

<div class="w-full px-4 pb-4 {className}">
	<!-- svelte-ignore a11y_no_static_element_interactions -->
	<div
		class="bg-surface shadow-float relative flex flex-col gap-2 rounded-lg p-3 {isDragging
			? 'ring-primary ring-2'
			: ''}"
		ondragenter={onDragEnter}
		ondragover={onDragOver}
		ondragleave={onDragLeave}
		ondrop={onDrop}
	>
		{#if isDragging}
			<div
				class="bg-surface/80 text-primary pointer-events-none absolute inset-0 z-10 flex items-center justify-center rounded-lg text-sm font-medium backdrop-blur-sm"
			>
				放開以加入檔案
			</div>
		{/if}

		{#if files.length > 0}
			<div class="flex flex-wrap gap-2">
				{#each files as file, i (i + '-' + file.name)}
					<div
						class="bg-elevated text-foreground flex items-center gap-2 rounded-md px-2 py-1 text-xs"
						title={file.name}
					>
						<svg class="text-muted-foreground h-3.5 w-3.5 flex-shrink-0" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.5">
							<path d="M9 1.5H3.5v13H12.5V5L9 1.5z" stroke-linejoin="round" />
							<path d="M9 1.5V5h3.5" stroke-linejoin="round" />
						</svg>
						<span class="max-w-[160px] truncate">{file.name}</span>
						<span class="text-faint">{formatSize(file.size)}</span>
						<button
							type="button"
							onclick={() => removeFile(i)}
							class="text-muted-foreground hover:text-foreground duration-fast flex h-4 w-4 items-center justify-center rounded-sm transition-colors"
							title="移除"
							aria-label="移除附件 {file.name}"
						>
							<svg class="h-3 w-3" viewBox="0 0 12 12" fill="none" stroke="currentColor" stroke-width="1.5">
								<path d="M3 3l6 6M9 3l-6 6" stroke-linecap="round" />
							</svg>
						</button>
					</div>
				{/each}
			</div>
		{/if}

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

		<input
			bind:this={fileInput}
			type="file"
			{accept}
			{multiple}
			class="hidden"
			onchange={onFileInputChange}
		/>

		<div class="flex items-center justify-between gap-2">
			<div class="flex items-center gap-1">
				<button
					onclick={openPicker}
					{disabled}
					type="button"
					class="text-muted-foreground hover:bg-elevated hover:text-foreground duration-fast flex h-8 w-8 items-center justify-center rounded-md transition-colors disabled:pointer-events-none disabled:opacity-40"
					title="附加檔案"
					aria-label="附加檔案"
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
				type="button"
				class="duration-fast flex h-8 w-8 items-center justify-center rounded-md transition-all
        {canSend
					? 'bg-primary shadow-mini text-white hover:brightness-110'
					: 'bg-elevated text-faint cursor-not-allowed'}"
				title="送出 (Enter)"
				aria-label="送出"
			>
				<svg class="h-4 w-4" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.5">
					<path d="M8 12V4M4 8l4-4 4 4" stroke-linecap="round" stroke-linejoin="round" />
				</svg>
			</button>
		</div>
	</div>
</div>
