<script lang="ts">
	import { setLocale } from '$lib/paraglide/runtime.js';
	// import { page } from '$app/state';
	// import { goto } from '$app/navigation';
	import { m } from '$lib/paraglide/messages.js';
	import { fade } from 'svelte/transition';

	let localeKey = $state(0); // 用于强制刷新
	async function changeLocale(locale: string) {
		// @ts-ignore
		await setLocale(locale, { reload: false });
		localeKey++; // 触发响应式更新
	}
</script>

{#key localeKey}
	<h1 in:fade={{ duration: 250 }}>{m.hello_world({ name: 'SvelteKit User' })}</h1>
	<h1 in:fade={{ duration: 250 }}>{m.hello_wld2({ name: 'SvelteKit User' })}</h1>
	<p in:fade={{ duration: 300 }}>{m.helpful_mean_jurgen_roam()}</p>
{/key}

<div>
	<button onclick={() => changeLocale('en')}>en</button>
	<button onclick={() => changeLocale('es')}>es</button>
	<button onclick={() => changeLocale('zh-cn')}>zh-cn</button>
	<button onclick={() => changeLocale('zh-tw')}>zh-tw</button>
</div>
<p>
	If you use VSCode, install the <a
		href="https://marketplace.visualstudio.com/items?itemName=inlang.vs-code-extension"
		target="_blank">Sherlock i18n extension</a
	> for a better i18n experience.
</p>
