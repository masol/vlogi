<script lang="ts">
	import { localeStore } from '$lib/stores/config/ipc/i18n.svelte.js';
	// import { page } from '$app/state';
	// import { goto } from '$app/navigation';
	import { m } from '$lib/paraglide/messages.js';
	import { fade } from 'svelte/transition';
</script>

{#key localeStore.lang}
	<h1 in:fade={{ duration: 250 }}>{m.hello_world({ name: 'SvelteKit User' })}</h1>
	<h1 in:fade={{ duration: 250 }}>{m.hello_wld2({ name: 'SvelteKit User' })}</h1>
	<p in:fade={{ duration: 300 }}>{m.helpful_mean_jurgen_roam()}</p>
{/key}

<div>
	<button onclick={() => localeStore.setLocale('en')}>en</button>
	<button onclick={() => localeStore.setLocale('es')}>es</button>
	<button onclick={() => localeStore.setLocale('zh-cn')}>zh-cn</button>
	<button onclick={() => localeStore.setLocale('zh-tw')}>zh-tw</button>
</div>
<p>
	If you use VSCode, install the <a
		href="https://marketplace.visualstudio.com/items?itemName=inlang.vs-code-extension"
		target="_blank">Sherlock i18n extension</a
	> for a better i18n experiencesetLocale.
</p>
