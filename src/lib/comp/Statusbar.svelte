<script lang="ts">
	import { slide } from 'svelte/transition';
	import IconChevronLeft from '~icons/lucide/chevron-left';
	import IconChevronRight from '~icons/lucide/chevron-right';
	import IconClock from '~icons/lucide/clock';
	import IconTarget from '~icons/lucide/target';

	import Lightswitch from './Lightswitch.svelte';
	import Repository from './Settings/Repo/Repository.svelte';

	// State
	let isExpanded = $state(false);

	// Static data
	const wordCount = 1234;
	const characterCount = 5678;
	const readingTime = '5 min';
	const status = 'Ready';

	// Toggle expansion
	function toggleExpanded() {
		isExpanded = !isExpanded;
	}

	// Handle keyboard interaction
	function handleKeydown(event: KeyboardEvent) {
		if (event.key === 'Enter' || event.key === ' ') {
			event.preventDefault();
			toggleExpanded();
		}
	}
</script>

<div class="fixed right-0 bottom-0" style="z-index: 100;" role="region" aria-label="Status bar">
	<!-- Backdrop上方遮挡区域 -->
	<div
		class="pointer-events-auto absolute right-0 bottom-full w-full"
		style="height: 12px;"
		aria-hidden="true"
	></div>

	<!-- 状态条主体 -->
	<div
		class="bg-surface-100-800 dark:bg-surface-800-100 rounded-l-container-token ring-surface-300-600 dark:ring-surface-600-300 pointer-events-auto flex h-10 items-center overflow-hidden shadow-lg ring-1"
	>
		<!-- Toggle button - always visible -->
		<button
			onclick={toggleExpanded}
			onkeydown={handleKeydown}
			class="hover:bg-surface-200-700 dark:hover:bg-surface-700-200 flex h-full w-10 shrink-0 items-center justify-center transition-colors focus:ring-2 focus:ring-primary-500 focus:outline-none focus:ring-inset dark:focus:ring-primary-400"
			aria-expanded={isExpanded}
			aria-label={isExpanded ? 'Collapse status bar' : 'Expand status bar'}
			type="button"
		>
			{#if isExpanded}
				<IconChevronRight
					class="text-surface-700-200 dark:text-surface-200-700 h-5 w-5"
					aria-hidden="true"
				/>
			{:else}
				<IconChevronLeft
					class="text-surface-700-200 dark:text-surface-200-700 h-5 w-5"
					aria-hidden="true"
				/>
			{/if}
		</button>

		<!-- Expanded content - slides in from right -->
		{#if isExpanded}
			<div
				transition:slide={{ axis: 'x', duration: 200 }}
				class="flex h-full items-center gap-4 px-4 whitespace-nowrap"
			>
				<Lightswitch></Lightswitch>

				<!-- Word count -->
				<div class="text-surface-700-200 dark:text-surface-200-700 flex items-center gap-2">
					<Repository></Repository>
				</div>

				<!-- Character count -->
				<div class="text-surface-700-200 dark:text-surface-200-700 flex items-center gap-2">
					<IconTarget class="h-4 w-4" aria-hidden="true" />
					<span class="text-sm font-medium">{characterCount.toLocaleString()} chars</span>
				</div>

				<!-- Reading time -->
				<div class="text-surface-700-200 dark:text-surface-200-700 flex items-center gap-2">
					<IconClock class="h-4 w-4" aria-hidden="true" />
					<span class="text-sm font-medium">{readingTime}</span>
				</div>

				<!-- Status -->
				<div
					class="border-surface-300-600 dark:border-surface-600-300 flex items-center gap-2 border-l pl-4"
				>
					<span class="h-2 w-2 rounded-full bg-success-500 dark:bg-success-400" aria-hidden="true"
					></span>
					<span class="text-surface-900-50 dark:text-surface-50-900 text-sm font-medium"
						>{status}</span
					>
				</div>
			</div>
		{/if}
	</div>

	<!-- Backdrop下方遮挡区域 -->
	<div
		class="pointer-events-auto absolute top-full right-0 w-full"
		style="height: 12px;"
		aria-hidden="true"
	></div>
</div>
