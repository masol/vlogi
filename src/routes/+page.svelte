<script lang="ts">
	import Lightswitch from '$lib/comp/Lightswitch.svelte';
</script>

<h1>Welcome to your library project</h1>
<p>Create your package using @sveltejs/package and preview/showcase your work with SvelteKit</p>
<p>Visit <a href="/demo">svelte.dev/docs/kit</a> to read the documentation</p>

<div
	class="flex min-h-screen flex-col items-center justify-center bg-gradient-to-br from-green-500 to-red-900"
>
	<h1 class="text-brand mb-4 text-4xl font-bold">🎨 UnoCSS + SvelteKit Demo</h1>

	<p class="mt-4 text-lg">点击次数：{1}</p>

	<button on:click={() => alert('属性模式生效 🎉')}> 属性模式按钮 </button>
	<Lightswitch></Lightswitch>
</div>
