<script lang="ts">
	type StepStatus = 'pending' | 'active' | 'completed';
	type Orientation = 'horizon' | 'nadir';

	interface Step {
		label: string;
		description?: string;
	}

	interface Props {
		steps: Step[];
		current?: number;
		orientation?: Orientation;
		class?: string;
	}

	let { steps, current = 0, orientation = 'horizon', class: className = '' }: Props = $props();

	function getStatus(index: number): StepStatus {
		if (index < current) return 'completed';
		if (index === current) return 'active';
		return 'pending';
	}

	const circleClass = (status: StepStatus) =>
		'w-8 h-8 rounded-full flex items-center justify-center text-xs font-[Inter] font-bold shrink-0 transition-all duration-base ' +
		(status === 'completed'
			? 'bg-primary text-white'
			: status === 'active'
				? 'bg-primary text-white ring-4 ring-primary/20'
				: 'bg-transparent border text-faint');

	const labelClass = (status: StepStatus) =>
		'text-xs font-[Inter] font-semibold whitespace-nowrap ' +
		(status === 'active' ? 'text-foreground' : status === 'completed' ? 'text-primary' : 'text-muted-foreground');

	const labelClassV = (status: StepStatus) =>
		'text-sm font-[Inter] font-semibold ' +
		(status === 'active' ? 'text-foreground' : status === 'completed' ? 'text-primary' : 'text-muted-foreground');
</script>

{#if orientation === 'horizon'}
	<div class="flex w-full items-center {className}">
		{#each steps as step, i (i)}
			{@const status = getStatus(i)}
			{@const isLast = i === steps.length - 1}

			<div class="flex items-center {isLast ? 'flex-none' : 'flex-1'}">
				<div class="flex flex-col items-center gap-1.5">
					<div class={circleClass(status)}>
						{#if status === 'completed'}
							<svg class="h-4 w-4" viewBox="0 0 16 12" fill="none" stroke="currentColor" stroke-width="2">
								<path d="M1 6L5.5 10.5L15 1" stroke-linecap="round" stroke-linejoin="round" />
							</svg>
						{:else}
							{i + 1}
						{/if}
					</div>
					<div class="text-center">
						<p class={labelClass(status)}>{step.label}</p>
						{#if step.description}
							<p class="text-faint hidden text-xs whitespace-nowrap sm:block">{step.description}</p>
						{/if}
					</div>
				</div>

				{#if !isLast}
					<div class="bg-elevated mx-3 h-px flex-1 overflow-hidden rounded-full">
						<div
							class="bg-primary duration-slow h-full transition-all"
							style="width: {status === 'completed' ? '100%' : '0%'};"
						></div>
					</div>
				{/if}
			</div>
		{/each}
	</div>
{:else}
	<div class="flex flex-col {className}">
		{#each steps as step, i (i)}
			{@const status = getStatus(i)}
			{@const isLast = i === steps.length - 1}

			<div class="flex items-start gap-4">
				<div class="flex flex-col items-center">
					<div class={circleClass(status)}>
						{#if status === 'completed'}
							<svg class="h-4 w-4" viewBox="0 0 16 12" fill="none" stroke="currentColor" stroke-width="2">
								<path d="M1 6L5.5 10.5L15 1" stroke-linecap="round" stroke-linejoin="round" />
							</svg>
						{:else}
							{i + 1}
						{/if}
					</div>
					{#if !isLast}
						<div class="bg-elevated my-1 min-h-[32px] w-px flex-1 overflow-hidden rounded-full">
							<div
								class="bg-primary duration-slow w-full transition-all"
								style="height: {status === 'completed' ? '100%' : '0%'};"
							></div>
						</div>
					{/if}
				</div>

				<div class="min-w-0 pt-1 pb-6">
					<p class={labelClassV(status)}>{step.label}</p>
					{#if step.description}
						<p class="text-muted-foreground mt-0.5 text-xs leading-relaxed">{step.description}</p>
					{/if}
				</div>
			</div>
		{/each}
	</div>
{/if}
