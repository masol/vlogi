<script lang="ts">
	import IconMdiPlus from '~icons/mdi/plus';
	import IconMdiClose from '~icons/mdi/close';

	interface Tab {
		id: string;
		label: string;
		closable: boolean;
	}

	let dynamicTabs = $state<Tab[]>([
		{ id: 'tab-1', label: 'Document 1123123213123123123', closable: true },
		{ id: 'tab-2', label: 'Document 2', closable: true }
	]);

	let activeTabId = $state<string>('tab-1');

	function selectTab(tabId: string) {
		activeTabId = tabId;
	}

	function closeTab(tabId: string, e: MouseEvent | KeyboardEvent) {
		e.stopPropagation();
		dynamicTabs = dynamicTabs.filter((tab) => tab.id !== tabId);

		if (activeTabId === tabId) {
			if (dynamicTabs.length > 0) {
				activeTabId = dynamicTabs[dynamicTabs.length - 1].id;
			} else {
				activeTabId = '';
			}
		}
	}

	function addNewTab() {
		const newId = `tab-${Date.now()}`;
		const newTab: Tab = {
			id: newId,
			label: `Document ${dynamicTabs.length + 1}`,
			closable: true
		};
		dynamicTabs = [...dynamicTabs, newTab];
		activeTabId = newId;
	}

	function onTabKeydown(e: KeyboardEvent, tabId: string) {
		if (e.key === 'Enter' || e.key === ' ') {
			e.preventDefault();
			selectTab(tabId);
		}
	}

	function onCloseKeydown(e: KeyboardEvent, tabId: string) {
		if (e.key === 'Enter' || e.key === ' ') {
			e.preventDefault();
			e.stopPropagation();
			closeTab(tabId, e);
		} else if (e.key === 'Delete') {
			e.preventDefault();
			e.stopPropagation();
			closeTab(tabId, e);
		}
	}

	function onAddKeydown(e: KeyboardEvent) {
		if (e.key === 'Enter' || e.key === ' ') {
			e.preventDefault();
			addNewTab();
		}
	}
</script>

<div
	class="flex min-w-0 flex-1 items-center gap-0.5 overflow-x-auto px-2"
	role="tablist"
	aria-label="Document tabs"
>
	{#each dynamicTabs as tab (tab.id)}
		<div
			class="group flex shrink-0 items-center gap-1.5 rounded-md px-2.5 py-1
			       text-sm transition-colors duration-150
			       {activeTabId === tab.id
				? 'bg-surface-200 text-primary-700 dark:bg-surface-700 dark:text-primary-400'
				: 'text-surface-600 hover:bg-surface-200/50 dark:text-surface-400 dark:hover:bg-surface-700/50'}
			       focus-within:ring-2 focus-within:ring-primary-500 focus-within:ring-offset-2 focus-within:ring-offset-surface-100 dark:focus-within:ring-offset-surface-800"
			role="tab"
			aria-selected={activeTabId === tab.id}
			aria-controls="{tab.id}-panel"
			tabindex={activeTabId === tab.id ? 0 : -1}
			onclick={() => selectTab(tab.id)}
			onkeydown={(e) => onTabKeydown(e, tab.id)}
		>
			<span class="max-w-32 truncate">{tab.label}</span>
			{#if tab.closable}
				<span
					class="grid size-4 place-items-center rounded opacity-0 transition-opacity
					       group-hover:opacity-70 hover:bg-surface-300 hover:opacity-100
					       focus-visible:opacity-100
					       focus-visible:ring-1 focus-visible:ring-primary-500 dark:hover:bg-surface-600"
					role="button"
					aria-label="Close {tab.label}"
					tabindex={0}
					onclick={(e) => closeTab(tab.id, e)}
					onkeydown={(e) => onCloseKeydown(e, tab.id)}
				>
					<IconMdiClose class="size-3" aria-hidden="true" />
				</span>
			{/if}
		</div>
	{/each}

	<button
		class="grid size-7 shrink-0 place-items-center rounded-md
		       text-surface-600 hover:bg-surface-200
		       focus-visible:ring-2 focus-visible:ring-primary-500
		       focus-visible:ring-offset-2 focus-visible:ring-offset-surface-100 dark:text-surface-400 dark:hover:bg-surface-700 dark:focus-visible:ring-offset-surface-800"
		aria-label="Add new tab"
		onclick={addNewTab}
		onkeydown={onAddKeydown}
	>
		<IconMdiPlus class="size-4" aria-hidden="true" />
	</button>
</div>
