<script lang="ts">
	import { onMount } from 'svelte';
	import { Window } from '@tauri-apps/api/window';
	import PanelToggle from './Paneltoggle.svelte';
	import LeftTabs from './LeftTabs.svelte';
	import DynamicTabs from './DynamicTabs.svelte';
	import { leftPanel } from '../stores/panel.svelte'; // 假设你的 store 路径

	// 导入 iconify 图标
	import IconMdiWindowMinimize from '~icons/mdi/window-minimize';
	import IconMdiWindowMaximize from '~icons/mdi/window-maximize';
	import IconMdiWindowRestore from '~icons/mdi/window-restore';
	import IconMdiClose from '~icons/mdi/close';

	let appWinCache: Window | null = null;
	const appWindow = () => {
		if (!appWinCache) {
			appWinCache = Window.getCurrent();
		}
		return appWinCache;
	};

	let isMax = $state(true);
	let windowWidth = $state(0);

	// 计算 LeftTabs 的实际宽度（像素值）
	let leftTabsWidth = $derived(() => {
		if (!leftPanel.show) return 0;
		// (窗口宽度 - 48) * size百分比
		return (windowWidth - 48) * (leftPanel.size / 100);
	});

	// 双击检测相关
	let lastClickTime = 0;
	let lastClickX = 0;
	let lastClickY = 0;
	const DOUBLE_CLICK_THRESHOLD = 300;
	const POSITION_THRESHOLD = 10;

	async function syncMax() {
		isMax = await appWindow().isMaximized();
	}

	async function toggleMaximize() {
		console.log('toggleMaximize');
		const max = await appWindow().isMaximized();
		max ? await appWindow().unmaximize() : await appWindow().maximize();
		await syncMax();
	}

	function minimize() {
		appWindow().minimize();
	}

	function closeWin() {
		appWindow().close();
	}

	async function startDrag(e: MouseEvent) {
		const currentTime = Date.now();
		const timeDiff = currentTime - lastClickTime;
		const positionDiffX = Math.abs(e.clientX - lastClickX);
		const positionDiffY = Math.abs(e.clientY - lastClickY);

		if (
			timeDiff < DOUBLE_CLICK_THRESHOLD &&
			positionDiffX < POSITION_THRESHOLD &&
			positionDiffY < POSITION_THRESHOLD
		) {
			console.log('Double click detected manually');
			await toggleMaximize();
			lastClickTime = 0;
			lastClickX = 0;
			lastClickY = 0;
			return;
		}

		lastClickTime = currentTime;
		lastClickX = e.clientX;
		lastClickY = e.clientY;

		if (isMax) {
			console.log('Window is maximized, drag disabled');
			return;
		}

		console.log('mousedown detected, starting drag immediately');
		try {
			await appWindow().startDragging();
		} catch (err) {
			console.error('Drag failed:', err);
		}
		console.log('Drag completed');
	}

	function updateWindowWidth() {
		windowWidth = window.innerWidth;
	}

	onMount(() => {
		let unsubs: Array<() => void> = [];

		// 初始化窗口宽度
		updateWindowWidth();

		// 监听窗口大小变化
		const resizeObserver = new ResizeObserver(() => {
			updateWindowWidth();
		});
		resizeObserver.observe(document.documentElement);

		(async () => {
			await syncMax();
			unsubs.push(await appWindow().onResized(syncMax));
			unsubs.push(await appWindow().onMoved(syncMax));
		})();

		return () => {
			resizeObserver.disconnect();
			for (const u of unsubs) {
				try {
					u();
				} catch {}
			}
		};
	});

	function onKeyToggle(e: KeyboardEvent) {
		if (e.key === 'Enter' || e.key === ' ') {
			e.preventDefault();
			toggleMaximize();
		}
	}
</script>

<header
	class="relative flex h-[var(--tb-height)] w-full items-center justify-between gap-2 border-b
         border-surface-200 bg-surface-100/80
         px-2 select-none dark:border-surface-700 dark:bg-surface-800/80"
	aria-label="Window title bar"
>
	<!-- Draggable area - 覆盖整个 header 除了控制按钮 -->
	<div
		class="absolute inset-0 {isMax ? 'cursor-default' : 'cursor-move'}"
		role="button"
		tabindex="0"
		aria-label="Drag window. Double-click to maximize or restore."
		onmousedown={startDrag}
		onkeydown={onKeyToggle}
	></div>

	<!-- toggle Left panel & LeftTabs -->
	<div class="relative z-10 flex min-w-0 items-center gap-2 rounded px-1">
		<PanelToggle size="sm" />
		{#if leftPanel.show}
			<div class="overflow-hidden transition-all duration-200" style="width: {leftTabsWidth()}px;">
				<LeftTabs />
				<!-- 右侧视觉反馈竖条 -->
				<div
					class="absolute top-0 right-0 h-full w-px bg-surface-300/60 dark:bg-surface-600/60"
					aria-hidden="true"
				></div>
			</div>
		{/if}
	</div>

	<!-- Center: Dynamic tabs -->
	<div class="relative z-10 flex min-w-0 flex-1">
		<DynamicTabs />
	</div>

	<!-- Right: window controls -->
	<div class="relative z-10 flex items-center gap-1">
		<!-- toggle Right panel -->
		<PanelToggle right={true} size="sm" />

		<button
			class="variant-ghost btn grid size-8 place-items-center rounded-md btn-sm
             hover:bg-surface-200 dark:hover:bg-surface-700"
			aria-label="Minimize window"
			onclick={minimize}
		>
			<IconMdiWindowMinimize class="opacity-80" aria-hidden="true" />
		</button>

		<button
			class="variant-ghost btn grid size-8 place-items-center rounded-md btn-sm
             hover:bg-surface-200 dark:hover:bg-surface-700"
			aria-label={isMax ? 'Restore window' : 'Maximize window'}
			onclick={toggleMaximize}
		>
			{#if isMax}
				<IconMdiWindowRestore class="opacity-80" aria-hidden="true" />
			{:else}
				<IconMdiWindowMaximize class="opacity-80" aria-hidden="true" />
			{/if}
		</button>

		<button
			class="variant-ghost btn grid size-8 place-items-center rounded-md btn-sm
             hover:bg-error-500/90 hover:text-white"
			aria-label="Close window"
			onclick={closeWin}
		>
			<IconMdiClose aria-hidden="true" />
		</button>
	</div>
</header>
