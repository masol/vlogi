<!-- FixedTabs.svelte -->
<script lang="ts">
	import IconMdiHome from '~icons/mdi/home';
	import IconMdiCog from '~icons/mdi/cog';

	interface Tab {
		id: string;
		label: string;
		icon: any;
		closable: boolean;
	}

	const fixedTabs: Tab[] = [
		{ id: 'home', label: 'Home', icon: IconMdiHome, closable: false },
		{ id: 'settings', label: 'Settings', icon: IconMdiCog, closable: false }
	];

	let activeTabId = $state<string>('home');

	function selectTab(tabId: string) {
		activeTabId = tabId;
	}

	function onTabKeydown(e: KeyboardEvent, tabId: string) {
		if (e.key === 'Enter' || e.key === ' ') {
			e.preventDefault();
			selectTab(tabId);
		}
	}
</script>

<div class="flex items-center gap-0.5" role="tablist" aria-label="Fixed navigation tabs">
	{#each fixedTabs as tab (tab.id)}
		<button
			class="flex items-center gap-1.5 rounded-md px-2.5 py-1 text-sm
			       transition-colors duration-150
			       {activeTabId === tab.id
				? 'bg-surface-200 text-primary-700 dark:bg-surface-700 dark:text-primary-400'
				: 'text-surface-600 hover:bg-surface-200/50 dark:text-surface-400 dark:hover:bg-surface-700/50'}
			       focus-visible:ring-2 focus-visible:ring-primary-500 focus-visible:ring-offset-2 focus-visible:ring-offset-surface-100 dark:focus-visible:ring-offset-surface-800"
			role="tab"
			aria-selected={activeTabId === tab.id}
			aria-controls="{tab.id}-panel"
			tabindex={activeTabId === tab.id ? 0 : -1}
			onclick={() => selectTab(tab.id)}
			onkeydown={(e) => onTabKeydown(e, tab.id)}
		>
			{#if tab.icon}
				<tab.icon class="size-4" aria-hidden="true" />
			{/if}
			<span class="truncate">{tab.label}</span>
		</button>
	{/each}
</div>
