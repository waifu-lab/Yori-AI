<script module>
	import { defineMeta } from '@storybook/addon-svelte-csf';
	import GalDialogue from '$lib/molecules/GalDialogue.svelte';
	import Button from '$lib/atoms/Button.svelte';

	const { Story } = defineMeta({
		title: 'Molecules/GalDialogue',
		component: GalDialogue,
		tags: ['autodocs'],
		argTypes: {
			open: { control: 'boolean' },
			name: { control: 'text' },
			text: { control: 'text' },
			typingSpeed: { control: { type: 'range', min: 10, max: 150, step: 5 } }
		}
	});
</script>

<script>
	let interactiveOpen = $state(false);
	let interactiveIndex = $state(0);

	const lines = [
		{ name: 'Yori', text: "Oh, you're here! I've been waiting for you." },
		{ name: 'Yori', text: 'I made some tea. Would you like a cup?' },
		{ name: 'Yori', text: "Don't worry, I'll always be here for you." }
	];

	function interactiveNext() {
		if (interactiveIndex < lines.length - 1) {
			interactiveIndex++;
		} else {
			interactiveOpen = false;
			interactiveIndex = 0;
		}
	}

	function startInteractive() {
		interactiveIndex = 0;
		interactiveOpen = true;
	}
</script>

<Story name="Default" asChild>
	<div style="height: 200px; position: relative;">
		<GalDialogue
			open={true}
			name="Yori"
			text="Hey, you seem a bit tired today. Want to take a break? I can keep you company for a while."
			typingSpeed={40}
		/>
	</div>
</Story>

<Story name="No Name" asChild>
	<div style="height: 200px; position: relative;">
		<GalDialogue
			open={true}
			name=""
			text="A mysterious voice echoes through the silence..."
			typingSpeed={60}
		/>
	</div>
</Story>

<Story name="Long Text" asChild>
	<div style="height: 220px; position: relative;">
		<GalDialogue
			open={true}
			name="Yori"
			text="Today was a long day, wasn't it? I noticed you've been working really hard lately. Sometimes it's important to step back and appreciate how far you've come. You've been doing great, and I'm proud of you."
			typingSpeed={30}
		/>
	</div>
</Story>

<Story name="Interactive" asChild>
	<div style="padding: 2rem; display: flex; flex-direction: column; align-items: center; gap: 1rem; height: 280px; position: relative;">
		<Button onclick={startInteractive}>Start Dialogue</Button>
		<GalDialogue
			bind:open={interactiveOpen}
			name={lines[interactiveIndex].name}
			text={lines[interactiveIndex].text}
			onDismiss={interactiveNext}
		/>
	</div>
</Story>

<Story name="Fast Typing" asChild>
	<div style="height: 200px; position: relative;">
		<GalDialogue
			open={true}
			name="System"
			text="Initiating connection to neural interface... please stand by."
			typingSpeed={15}
		/>
	</div>
</Story>
