<script lang="ts">
	import '../app.css';
	import {init} from "$lib/index.js"
	import { onMount } from 'svelte';

	onMount(async () => {
		await init();
	});
</script>

<slot />
