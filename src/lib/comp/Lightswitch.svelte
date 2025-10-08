<script lang="ts">
	import { fade } from 'svelte/transition';
	import { onMount } from 'svelte';
	import IconSun from '~icons/mdi/weather-sunny';
	import IconMoon from '~icons/mdi/weather-night';

	let checked = $state(false);

	onMount(() => {
		const mode = localStorage.getItem('mode') || 'light';
		checked = mode === 'dark';
		document.documentElement.setAttribute('data-mode', mode);
	});

	$effect(() => {
		const mode = checked ? 'dark' : 'light';
		document.documentElement.setAttribute('data-mode', mode);
		localStorage.setItem('mode', mode);
	});

	const toggleMode = () => {
		checked = !checked;
	};
</script>

<button class="btn-icon" aria-pressed={checked} onclick={toggleMode}>
	{#if checked}
		<span in:fade={{ duration: 200 }}>
			<IconMoon class="h-5 w-5" />
		</span>
	{:else}
		<span in:fade={{ duration: 200 }}>
			<IconSun class="h-5 w-5" />
		</span>
	{/if}
</button>
