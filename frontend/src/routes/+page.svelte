<h1>Welcome to your library project</h1>
<p>Create your package using @sveltejs/package and preview/showcase your work with SvelteKit</p>
<p>Visit <a href="https://svelte.dev/docs/kit">svelte.dev/docs/kit</a> to read the documentation</p>

<div class="min-h-screen flex flex-col items-center justify-center bg-gradient-to-br from-purple-100 to-indigo-700">
  <h1 class="text-4xl font-bold text-brand mb-4">🎨 UnoCSS + SvelteKit Demo</h1>

  <p class="mt-4 text-lg">点击次数：{1}</p>

  <button
    bg="brand hover:red-100"
    text="white"
    p="x-4 y-2"
    rounded
    hover="scale-105"
    transition="all"
    mt="6"
    on:click={() => alert('属性模式生效 🎉')}
  >
    属性模式按钮
  </button>
</div>