<script lang="ts">
	import IconHome from '~icons/carbon/home';
	import IconSearch from '~icons/carbon/search';
	import IconDocument from '~icons/carbon/document';
	import IconSettings from '~icons/carbon/settings';
	import IconFolder from '~icons/carbon/folder';
	import IconBookmark from '~icons/carbon/bookmark';

	interface NavItem {
		id: string;
		icon: any;
		label: string;
		onClick?: () => void;
	}

	interface Props {
		items?: NavItem[];
		density?: 'compact' | 'comfortable' | 'spacious';
	}

	let {
		items = [
			{ id: 'home', icon: IconHome, label: 'Home' },
			{ id: 'search', icon: IconSearch, label: 'Search' },
			{ id: 'files', icon: IconFolder, label: 'Files' },
			{ id: 'bookmarks', icon: IconBookmark, label: 'Bookmarks' },
			{ id: 'documents', icon: IconDocument, label: 'Documents' },
			{ id: 'settings', icon: IconSettings, label: 'Settings' }
		],
		density = 'comfortable'
	}: Props = $props();

	let hoveredId = $state<string | null>(null);

	const densityConfig = {
		compact: {
			width: '2.5rem', // 40px
			gap: 'gap-1',
			padding: 'py-2 px-1',
			itemSize: 'h-8 w-8'
		},
		comfortable: {
			width: '3rem', // 48px
			gap: 'gap-2',
			padding: 'py-3 px-1.5',
			itemSize: 'h-10 w-10'
		},
		spacious: {
			width: '3.5rem', // 56px
			gap: 'gap-3',
			padding: 'py-4 px-2',
			itemSize: 'h-12 w-12'
		}
	};

	const config = $derived(densityConfig[density]);

	function handleItemClick(item: NavItem) {
		item.onClick?.();
	}

	function handleKeyDown(event: KeyboardEvent, item: NavItem) {
		if (event.key === 'Enter' || event.key === ' ') {
			event.preventDefault();
			handleItemClick(item);
		}
	}
</script>

<nav
	class="flex h-full min-h-0 shrink-0 flex-col items-center border-r border-surface-300-700 bg-surface-100-900 {config.gap} {config.padding}"
	style="width: {config.width};"
	data-density={density}
	aria-label="Main navigation"
>
	{#each items as item (item.id)}
		{@const isHovered = hoveredId === item.id}
		{@const IconComponent = item.icon}

		<div class="group/nav-item relative flex w-full items-center justify-center">
			<button
				type="button"
				class="
					flex items-center justify-center
					{config.itemSize}
					rounded-token
					text-surface-700-300
					transition-colors duration-200
					hover:bg-surface-200-800 hover:text-surface-900-100
					focus-visible:ring-2
					focus-visible:ring-primary-500 focus-visible:ring-offset-2 focus-visible:ring-offset-surface-100-900 focus-visible:outline-none
					active:bg-surface-300-700
				"
				onclick={() => handleItemClick(item)}
				onmouseenter={() => (hoveredId = item.id)}
				onmouseleave={() => (hoveredId = null)}
				onkeydown={(e) => handleKeyDown(e, item)}
				aria-label={item.label}
			>
				<IconComponent class="h-5 w-5" aria-hidden="true" />
			</button>

			<!-- Tooltip -->
			<div
				class="
					pointer-events-none
					opacity-0
					transition-opacity duration-150
					group-focus-within/nav-item:opacity-100
					group-hover/nav-item:opacity-100
				"
				role="tooltip"
				aria-hidden={!isHovered}
			>
				<div
					class="
						ribbon-tooltip
						rounded-token flex items-center
						gap-1
						bg-surface-900-100
						px-3 py-1.5
						text-sm font-medium
						whitespace-nowrap text-surface-100-900
						shadow-xl
					"
				>
					<div
						class="
							h-0 w-0
							border-y-4 border-r-4
							border-y-transparent border-r-surface-900-100
						"
						aria-hidden="true"
					></div>
					<span>{item.label}</span>
				</div>
			</div>
		</div>
	{/each}
</nav>

<style>
	/* Tooltip 定位：相对于父容器 */
	.ribbon-tooltip {
		position: absolute;
		left: calc(100% + 0.5rem);
		top: 50%;
		transform: translateY(-50%);
		z-index: 50;
	}

	@media (prefers-reduced-motion: reduce) {
		* {
			transition-duration: 0.01ms !important;
		}
	}
</style>
