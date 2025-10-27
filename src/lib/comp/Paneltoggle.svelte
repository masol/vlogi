<script lang="ts">
	import IconMenuLeftClose from '~icons/lucide/panel-left';
	import IconMenuLeft from '~icons/lucide/panel-left-close';
	import IconMenuRightClose from '~icons/lucide/panel-right';
	import IconMenuRight from '~icons/lucide/panel-right-close';

	// Props 定义
	let {
		show = $bindable(false),
		width = $bindable(10),
		right = false,
		size = 'md',
		ariaLabel
	}: {
		show?: boolean;
		right?: boolean;
        width: number;
		size?: 'sm' | 'md' | 'lg';
		ariaLabel?: string;
	} = $props();

	// 尺寸映射
	const sizeClasses = {
		sm: 'w-6 h-6 text-sm',
		md: 'w-8 h-8 text-base',
		lg: 'w-10 h-10 text-lg'
	};

	// 根据方向选择图标
	const OpenIcon = $derived(right ? IconMenuRight : IconMenuLeft);
	const CloseIcon = $derived(right ? IconMenuRightClose : IconMenuLeftClose);

	// 计算 ARIA 标签
	const computedAriaLabel = $derived(
		ariaLabel ?? (right ? 'Toggle right panel' : 'Toggle left panel')
	);

	// 切换面板显示状态
	function togglePanel() {
		if (!show && width < 10) {
            width = 20;
		}
		show = !show;
	}

	// 键盘事件处理
	function handleKeyDown(e: KeyboardEvent) {
		if (e.key === 'Enter' || e.key === ' ') {
			e.preventDefault();
			togglePanel();
		}
	}
</script>

<button
	type="button"
	onclick={togglePanel}
	onkeydown={handleKeyDown}
	aria-label={computedAriaLabel}
	aria-pressed={show}
	class="
    {sizeClasses[size]}
    rounded-token
    text-surface-600-300-token
    hover:bg-surface-hover-token
    focus-visible:ring-offset-surface-50-900-token
    inline-flex
    items-center
    justify-center
    bg-transparent
    transition-colors
    duration-200
    hover:text-primary-500
    focus-visible:ring-2
    focus-visible:ring-primary-500
    focus-visible:ring-offset-2
    focus-visible:outline-none
    active:scale-95
    disabled:cursor-not-allowed
    disabled:opacity-50
  "
>
	{#if show}
		<OpenIcon class="h-full w-full" />
	{:else}
		<CloseIcon class="h-full w-full" />
	{/if}
</button>
