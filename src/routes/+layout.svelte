<script lang="ts">
	import '../app.css';
	import { init } from '$lib/index.js';
	import { onMount } from 'svelte';
	import Titlebar from '$lib/comp/Titlebar.svelte';
	import Sidebar from '$lib/comp/Sidebar.svelte';
	import { Pane, Splitpanes } from 'svelte-splitpanes';
	import { leftPanel, rightPanel, calcMainSize, type PanelStore } from '$lib/stores/panel.svelte';

	// 定义 children snippet
	let { children } = $props();

	onMount(async () => {
		await init();
	});

	const hideOnDrag = (panel: PanelStore) => {
		if (panel.show) {
			if (panel.size < 3) {
				setTimeout(() => {
					panel.show = false;
				}, 300);
			}
		}
	};

	$effect(() => {
		hideOnDrag(leftPanel);
		hideOnDrag(rightPanel);
	});
</script>

<div class="flex h-screen w-screen flex-col overflow-hidden">
	<Titlebar />

	<div class="flex min-h-0 flex-1 overflow-hidden">
		<!-- 固定侧边栏 -->
		<aside class="relative z-10 flex-shrink-0">
			<Sidebar />
		</aside>

		<!-- 主内容区域 - 使用 Splitpanes 实现三列布局 -->
		<div class="min-h-0 flex-1 overflow-hidden">
			<Splitpanes class="h-full" theme="modern-theme">
				<!-- 左侧面板 - 条件渲染 -->
				{#if leftPanel.show}
					<Pane minSize={1} maxSize={50} bind:size={leftPanel.size} class="overflow-hidden">
						<aside
							class="h-full overflow-auto border-l border-surface-200 bg-surface-100 dark:border-surface-700 dark:bg-surface-800"
						>
							<!-- 右侧面板内容 -->
							<div class="p-4">
								<h2 class="text-lg font-semibold text-surface-900 dark:text-surface-100">
									Left Panel
								</h2>
								<p class="mt-2 text-sm text-surface-600 dark:text-surface-400">
									Additional content goes here
								</p>
							</div>
						</aside>
					</Pane>
				{/if}

				<!-- 中间主面板 - 始终显示 -->
				<Pane minSize={30} size={calcMainSize()} class="overflow-hidden">
					<main class="h-full overflow-auto bg-surface-50 dark:bg-surface-900">
						{#if children}
							{@render children()}
						{/if}
					</main>
				</Pane>

				<!-- 右侧面板 - 条件渲染 -->
				{#if rightPanel.show}
					<Pane minSize={1} maxSize={50} bind:size={rightPanel.size} class="overflow-hidden">
						<aside
							class="h-full overflow-auto border-l border-surface-200 bg-surface-100 dark:border-surface-700 dark:bg-surface-800"
						>
							<!-- 右侧面板内容 -->
							<div class="p-4">
								<h2 class="text-lg font-semibold text-surface-900 dark:text-surface-100">
									Right Panel
								</h2>
								<p class="mt-2 text-sm text-surface-600 dark:text-surface-400">
									Additional content goes here
								</p>
							</div>
						</aside>
					</Pane>
				{/if}
			</Splitpanes>
		</div>
	</div>
</div>

<style>
	/* Splitpanes 容器样式 */
	:global(.modern-theme.splitpanes) {
		background-color: transparent;
	}

	:global(.modern-theme .splitpanes__pane) {
		background-color: transparent;
	}

	/* 分隔器核心样式 - 极细的视觉边界 */
	:global(.modern-theme .splitpanes__splitter) {
		background-color: rgb(var(--color-surface-300));
		width: 1px;
		position: relative;
		transition:
			background-color 200ms cubic-bezier(0.4, 0, 0.2, 1),
			width 200ms cubic-bezier(0.4, 0, 0.2, 1);
		cursor: col-resize !important;
		border: none;
		margin: 0;
		flex-shrink: 0;
		z-index: 10;
	}

	:global(.dark .modern-theme .splitpanes__splitter) {
		background-color: rgb(var(--color-surface-600));
	}

	/* 分隔器悬停状态 - Obsidian 风格的明显视觉反馈 */
	:global(.modern-theme .splitpanes__splitter:hover) {
		background-color: rgb(var(--color-primary-500));
		width: 3px;
		box-shadow: 0 0 8px rgb(var(--color-primary-500) / 0.3);
	}

	:global(.dark .modern-theme .splitpanes__splitter:hover) {
		background-color: rgb(var(--color-primary-400));
		box-shadow: 0 0 8px rgb(var(--color-primary-400) / 0.4);
	}

	/* 分隔器拖拽激活状态 - 更强烈的视觉反馈 */
	:global(.modern-theme .splitpanes__splitter.splitpanes__splitter__active) {
		background-color: rgb(var(--color-primary-600));
		width: 1px;
		box-shadow: 0 0 12px rgb(var(--color-primary-600) / 0.5);
	}

	:global(.dark .modern-theme .splitpanes__splitter.splitpanes__splitter__active) {
		background-color: rgb(var(--color-primary-300));
		box-shadow: 0 0 12px rgb(var(--color-primary-300) / 0.6);
	}

	/* 扩大交互区域 - 创建隐形的大触摸区域 */
	:global(.modern-theme .splitpanes__splitter::before) {
		content: '';
		position: absolute;
		left: -11px;
		top: 0;
		width: 24px;
		height: 100%;
		cursor: col-resize !important;
		z-index: 1;
		background-color: transparent;
	}

	/* 可视化抓手指示器 - Obsidian 风格 */
	:global(.modern-theme .splitpanes__splitter::after) {
		content: '';
		position: absolute;
		left: 50%;
		top: 50%;
		transform: translate(-50%, -50%);
		width: 6px;
		height: 48px;
		background-color: rgb(var(--color-surface-50));
		border-radius: 3px;
		border: 2px solid rgb(var(--color-primary-500));
		box-shadow:
			0 0 0 4px rgb(var(--color-primary-500) / 0.15),
			0 2px 8px rgb(var(--color-surface-900) / 0.2);
		transition: all 200ms cubic-bezier(0.4, 0, 0.2, 1);
		pointer-events: none;
		opacity: 0;
		z-index: 2;
	}

	:global(.dark .modern-theme .splitpanes__splitter::after) {
		background-color: rgb(var(--color-surface-800));
		border-color: rgb(var(--color-primary-400));
		box-shadow:
			0 0 0 4px rgb(var(--color-primary-400) / 0.2),
			0 2px 8px rgb(var(--color-surface-950) / 0.4);
	}

	/* 悬停时显示抓手 - Obsidian 风格增强 */
	:global(.modern-theme .splitpanes__splitter:hover::after) {
		opacity: 1;
		height: 64px;
		width: 8px;
		border-width: 2px;
		box-shadow:
			0 0 0 6px rgb(var(--color-primary-500) / 0.2),
			0 4px 12px rgb(var(--color-surface-900) / 0.25);
	}

	:global(.dark .modern-theme .splitpanes__splitter:hover::after) {
		box-shadow:
			0 0 0 6px rgb(var(--color-primary-400) / 0.25),
			0 4px 12px rgb(var(--color-surface-950) / 0.5);
	}

	/* 拖拽时增强抓手 */
	:global(.modern-theme .splitpanes__splitter.splitpanes__splitter__active::after) {
		opacity: 1;
		height: 80px;
		width: 10px;
		border-width: 3px;
		border-color: rgb(var(--color-primary-600));
		box-shadow:
			0 0 0 8px rgb(var(--color-primary-600) / 0.25),
			0 6px 16px rgb(var(--color-surface-900) / 0.3);
	}

	:global(.dark .modern-theme .splitpanes__splitter.splitpanes__splitter__active::after) {
		border-color: rgb(var(--color-primary-300));
		box-shadow:
			0 0 0 8px rgb(var(--color-primary-300) / 0.3),
			0 6px 16px rgb(var(--color-surface-950) / 0.6);
	}

	/* 焦点可见性 - 键盘导航支持 */
	:global(.modern-theme .splitpanes__splitter:focus-visible) {
		outline: 2px solid rgb(var(--color-primary-500));
		outline-offset: 2px;
		z-index: 30;
	}

	:global(.modern-theme .splitpanes__splitter:focus-visible::after) {
		opacity: 1;
	}

	/* 覆盖 splitpanes 默认光标样式 */
	:global(.modern-theme .splitpanes__splitter),
	:global(.modern-theme .splitpanes__splitter *) {
		cursor: col-resize !important;
	}

	/* 确保在拖拽过程中整个页面使用调整大小光标 */
	:global(body.splitpanes-dragging),
	:global(body.splitpanes-dragging *) {
		cursor: col-resize !important;
		user-select: none !important;
	}

	/* 防止拖拽时文本选中 */
	:global(.modern-theme .splitpanes__splitter.splitpanes__splitter__active ~ .splitpanes__pane) {
		user-select: none !important;
		pointer-events: none !important;
	}
</style>
