<!-- src/routes/+layout.svelte -->
<script lang="ts">
	import { onMount } from 'svelte';

	onMount(() => {
		import('virtual:uno.css');
		if (import.meta.env.DEV) {
			// @ts-ignore
			import('virtual:unocss-devtools');
		}
	});
</script>

<slot />
