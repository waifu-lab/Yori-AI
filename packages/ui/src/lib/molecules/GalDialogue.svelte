<script lang="ts">
	import { onDestroy } from 'svelte';

	interface Props {
		open?: boolean;
		name?: string;
		text?: string;
		typingSpeed?: number;
		onDismiss?: () => void;
		class?: string;
	}

	let {
		open = $bindable(false),
		name = '',
		text = '',
		typingSpeed = 40,
		onDismiss,
		class: className = ''
	}: Props = $props();

	let displayed = $state('');
	let done = $state(false);
	let timer: ReturnType<typeof setTimeout> | null = null;

	function clearTimer() {
		if (timer !== null) {
			clearTimeout(timer);
			timer = null;
		}
	}

	function startTyping(fullText: string) {
		clearTimer();
		displayed = '';
		done = false;
		let i = 0;

		function tick() {
			if (i < fullText.length) {
				displayed = fullText.slice(0, ++i);
				timer = setTimeout(tick, typingSpeed);
			} else {
				done = true;
			}
		}
		tick();
	}

	function finish() {
		clearTimer();
		displayed = text;
		done = true;
	}

	function handleClick() {
		if (!done) {
			finish();
		} else {
			open = false;
			onDismiss?.();
		}
	}

	function handleKeydown(e: KeyboardEvent) {
		if (e.key === 'Enter' || e.key === ' ') {
			e.preventDefault();
			handleClick();
		} else if (e.key === 'Escape') {
			open = false;
			onDismiss?.();
		}
	}

	$effect(() => {
		if (open && text) {
			startTyping(text);
		} else if (!open) {
			clearTimer();
			displayed = '';
			done = false;
		}
	});

	onDestroy(clearTimer);
</script>

{#if open}
	<!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
	<!-- svelte-ignore a11y_no_noninteractive_tabindex -->
	<div
		role="dialog"
		aria-live="polite"
		tabindex="0"
		class="gal-dialogue {className}"
		onclick={handleClick}
		onkeydown={handleKeydown}
	>
		{#if name}
			<div class="gal-name">
				<span>{name}</span>
			</div>
		{/if}

		<div class="gal-body">
			<p class="gal-text">{displayed}<span class="gal-cursor" class:hidden={done}>▌</span></p>
			{#if done}
				<span class="gal-advance">▶</span>
			{/if}
		</div>
	</div>
{/if}

<style>
	/* Dark mode defaults (galgame is inherently dark-first) */
	.gal-dialogue {
		--gal-bg-start: rgba(4, 4, 12, 0.88);
		--gal-bg-end: rgba(8, 8, 20, 0.92);
		--gal-border: color-mix(in srgb, var(--color-primary) 35%, rgba(255, 255, 255, 0.08));
		--gal-inset: rgba(255, 255, 255, 0.04);
		--gal-shadow-drop: rgba(0, 0, 0, 0.5);
		--gal-gloss: rgba(255, 255, 255, 0.03);
		--gal-text: rgba(255, 255, 255, 0.92);
		--gal-text-shadow: rgba(0, 0, 0, 0.6);

		position: fixed;
		bottom: 2rem;
		left: 50%;
		transform: translateX(-50%);
		width: min(760px, calc(100vw - 3rem));
		z-index: 100;
		cursor: pointer;
		user-select: none;
		outline: none;
		animation: gal-in 0.2s ease-out both;
	}

	/* Light mode overrides */
	:global([data-theme='light']) .gal-dialogue {
		--gal-bg-start: rgba(252, 252, 255, 0.88);
		--gal-bg-end: rgba(245, 245, 252, 0.93);
		--gal-border: color-mix(in srgb, var(--color-primary) 30%, rgba(0, 0, 0, 0.1));
		--gal-inset: rgba(255, 255, 255, 0.7);
		--gal-shadow-drop: rgba(0, 0, 0, 0.14);
		--gal-gloss: rgba(255, 255, 255, 0.5);
		--gal-text: rgba(10, 10, 20, 0.9);
		--gal-text-shadow: rgba(0, 0, 0, 0.08);
	}

	@keyframes gal-in {
		from {
			opacity: 0;
			transform: translateX(-50%) translateY(8px);
		}
		to {
			opacity: 1;
			transform: translateX(-50%) translateY(0);
		}
	}

	.gal-name {
		display: inline-flex;
		align-items: center;
		padding: 0.25rem 0.875rem;
		margin-bottom: -1px;
		background: linear-gradient(
			135deg,
			color-mix(in srgb, var(--color-primary) 80%, #000) 0%,
			color-mix(in srgb, var(--color-primary) 50%, #000) 100%
		);
		border: 1px solid color-mix(in srgb, var(--color-primary) 60%, transparent);
		border-bottom: none;
		border-radius: 6px 12px 0 0;
		box-shadow:
			0 0 12px color-mix(in srgb, var(--color-primary) 30%, transparent),
			inset 0 1px 0 rgba(255, 255, 255, 0.15);
		position: relative;
		z-index: 1;
	}

	.gal-name span {
		font-family: 'Space Grotesk', ui-sans-serif, sans-serif;
		font-size: 0.8125rem;
		font-weight: 600;
		letter-spacing: 0.08em;
		color: #fff;
		text-shadow: 0 0 8px color-mix(in srgb, var(--color-primary) 60%, transparent);
	}

	.gal-body {
		position: relative;
		background: linear-gradient(160deg, var(--gal-bg-start) 0%, var(--gal-bg-end) 100%);
		border: 1px solid var(--gal-border);
		border-radius: 0 12px 12px 12px;
		padding: 1.125rem 1.375rem 1rem;
		backdrop-filter: blur(24px) saturate(1.4);
		-webkit-backdrop-filter: blur(24px) saturate(1.4);
		box-shadow:
			0 0 0 1px var(--gal-inset) inset,
			0 2px 32px var(--gal-shadow-drop),
			0 0 48px color-mix(in srgb, var(--color-primary) 10%, transparent);
		min-height: 5.5rem;
		display: flex;
		flex-direction: column;
		justify-content: space-between;
		gap: 0.5rem;
	}

	.gal-body::before {
		content: '';
		position: absolute;
		inset: 0;
		border-radius: inherit;
		background: linear-gradient(160deg, var(--gal-gloss) 0%, transparent 60%);
		pointer-events: none;
	}

	.gal-text {
		font-family: 'Newsreader', ui-serif, Georgia, serif;
		font-size: 1rem;
		line-height: 1.75;
		color: var(--gal-text);
		text-shadow: 0 1px 3px var(--gal-text-shadow);
		margin: 0;
		min-height: 3.5rem;
		white-space: pre-wrap;
	}

	.gal-cursor {
		color: var(--color-primary);
		animation: blink 0.8s step-end infinite;
		font-size: 0.9em;
	}

	.gal-cursor.hidden {
		display: none;
	}

	@keyframes blink {
		50% { opacity: 0; }
	}

	.gal-advance {
		align-self: flex-end;
		color: var(--color-primary);
		font-size: 0.625rem;
		opacity: 0.8;
		animation: bounce 1s ease-in-out infinite;
	}

	@keyframes bounce {
		0%, 100% { transform: translateY(0); }
		50% { transform: translateY(2px); }
	}
</style>
