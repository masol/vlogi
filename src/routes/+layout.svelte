<script lang="ts">
	import '../app.css';
	import { init } from '$lib/index.js';
	import { onMount, setContext } from 'svelte';
	import Titlebar from '$lib/comp/Titlebar.svelte';
	import Sidebar from '$lib/comp/Sidebar.svelte';
	import Statusbar from '$lib/comp/Statusbar.svelte';
	import { Toast, createToaster } from '@skeletonlabs/skeleton-svelte';

	const toaster = createToaster({
		placement: 'top'
	});
	setContext('toaster', toaster);

	let { children } = $props();

	onMount(async () => {
		window.addEventListener('contextmenu', (e) => {
			e.preventDefault();
		});
		await init();
	});
</script>

<div class="flex h-screen w-screen flex-col overflow-hidden">
	<Titlebar />
	<Statusbar></Statusbar>
	<div class="flex min-h-0 flex-1 overflow-hidden">
		<aside class="relative z-10 flex-shrink-0">
			<Sidebar />
		</aside>

		{#if children}
			{@render children()}
		{/if}
	</div>

	<Toast.Group {toaster}>
		{#snippet children(toast)}
			<Toast {toast}>
				<Toast.Message>
					<Toast.Title>{toast.title}</Toast.Title>
					<Toast.Description>{toast.description}</Toast.Description>
				</Toast.Message>
				{#if toast.action}
					<Toast.ActionTrigger>{toast.action.label}</Toast.ActionTrigger>
				{/if}
				{#if toast.closable !== false}
					<Toast.CloseTrigger />
				{/if}
			</Toast>
		{/snippet}
	</Toast.Group>
</div>
