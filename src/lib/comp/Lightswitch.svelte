<script lang="ts">
	import { fade } from 'svelte/transition';
	// import { onMount } from 'svelte';
	import IconSun from '~icons/mdi/weather-sunny';
	import IconMoon from '~icons/mdi/weather-night';
	import { lightStore } from '$lib/stores/config/ipc/light.svelte';

	let checked = $derived(lightStore.mode === 'dark');

	// onMount(() => {
	// 	document.documentElement.setAttribute('data-mode', lightStore.mode);
	// });

	// $effect(() => {
	// 	const mode = checked ? 'dark' : 'light';
	// 	document.documentElement.setAttribute('data-mode', mode);
	// });

	const toggleMode = () => {
		checked = !checked;
		const mode = checked ? 'dark' : 'light';
		document.documentElement.setAttribute('data-mode', mode);
		lightStore.setMode(mode);
	};
</script>

<button
	class="variant-ghost btn-icon btn grid size-6 place-items-center rounded-md btn-sm
             hover:bg-surface-500/10 hover:shadow-md"
	aria-pressed={checked}
	onclick={toggleMode}
>
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
