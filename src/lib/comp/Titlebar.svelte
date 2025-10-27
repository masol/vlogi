<script lang="ts">
	import { onMount } from 'svelte';
	import { Window } from '@tauri-apps/api/window';
	import Lightswitch from './Lightswitch.svelte';
	import PanelToggle from './Paneltoggle.svelte';
	import { leftPanel, rightPanel } from '../stores/panel.svelte';

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

	// 双击检测相关
	let lastClickTime = 0;
	let lastClickX = 0;
	let lastClickY = 0;
	const DOUBLE_CLICK_THRESHOLD = 300; // 两次点击间隔小于 200ms
	const POSITION_THRESHOLD = 10; // 位置偏差小于 5 像素

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

		// 检测是否为双击
		if (
			timeDiff < DOUBLE_CLICK_THRESHOLD &&
			positionDiffX < POSITION_THRESHOLD &&
			positionDiffY < POSITION_THRESHOLD
		) {
			console.log('Double click detected manually');
			// 手动触发双击事件 - toggle 最大化状态
			await toggleMaximize();
			// 重置，避免三击被识别为又一次双击
			lastClickTime = 0;
			lastClickX = 0;
			lastClickY = 0;
			return;
		}

		// 更新上次点击信息
		lastClickTime = currentTime;
		lastClickX = e.clientX;
		lastClickY = e.clientY;

		// 如果已最大化，不启动拖拽
		if (isMax) {
			console.log('Window is maximized, drag disabled');
			return;
		}

		// 立即开始拖拽
		console.log('mousedown detected, starting drag immediately');
		try {
			await appWindow().startDragging();
		} catch (err) {
			console.error('Drag failed:', err);
		}
		console.log('Drag completed');
	}

	onMount(() => {
		let unsubs: Array<() => void> = [];
		(async () => {
			await syncMax();
			unsubs.push(await appWindow().onResized(syncMax));
			unsubs.push(await appWindow().onMoved(syncMax));
		})();

		return () => {
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

	<!-- Left: app title -->
	<div class="relative z-10 flex min-w-0 items-center gap-2 rounded px-1">
		<PanelToggle bind:show={leftPanel.show} bind:width={leftPanel.size} size="sm" />
	</div>

	<!-- Right: window controls -->
	<div class="relative z-10 flex items-center gap-1">
		<PanelToggle bind:show={rightPanel.show} bind:width={rightPanel.size} right={true} size="sm" />

		<Lightswitch></Lightswitch>
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
