<script lang="ts">
	/**
	 * 📦 数据来源说明
	 *
	 * 组件当前使用内部假数据。实际使用时,请将数据替换为外部 Store:
	 *
	 * 示例:
	 * import { vaultStore } from '$lib/stores/vaultStore';
	 * let data = vaultStore.data; // 替换下方的假数据定义
	 */

	import { createToaster, Dialog, Portal, Toast } from '@skeletonlabs/skeleton-svelte';
	import IconFolderGit from '~icons/lucide/folder-git';
	import IconX from '~icons/lucide/x';

	import { repositoryStore } from '../../../stores/repository.svelte';
	import RepositoryList from './RepositoryList.svelte';
	import OpenProject from './OpenProject.svelte';
	import RepoInfo from './RepoInfo.svelte';
	import { setContext } from 'svelte';
	import { softinfo } from '$lib/utils/softinfo';
	import { localeStore } from '$lib/stores/config/ipc/i18n.svelte';

	// 为对话框创建独立的 toaster 实例
	const dialogToaster = createToaster({
		placement: 'top' // 可以根据需要调整位置
	});
	// 在组件初始化时设置 context
	setContext('dialogToaster', dialogToaster);
	// 关闭对话框时清空全部toaster.
	function opneChanged(param: Record<string, any>) {
		if (!param.open) {
			dialogToaster.dismiss();
		}
	}

	const currentLanguage = $derived(localeStore.lang);

	const repositories = $derived(repositoryStore.repositories);
	const isEmpty = $derived(repositories.length === 0);

	// 假数据定义(使用 $state rune)
	let data = $state({
		vaults: [
			{ id: '1', name: '个人笔记', path: '/Users/me/notes', version: '1.5.3' },
			{ id: '2', name: '工作文档', path: '/Users/me/work', version: '1.5.3' },
			{ id: '3', name: '学习资料', path: '/Users/me/study', version: '1.5.2' }
		],
		selectedVaultId: '1',
		appVersion: '1.5.3',
		currentLanguage: 'zh-CN'
	});

	// 计算当前选中的仓库
	let selectedVault = $derived(
		data.vaults.find((v) => v.id === data.selectedVaultId) || data.vaults[0]
	);

	function changeLanguage(lang: string) {
		data.currentLanguage = lang;
	}
</script>

<Dialog closeOnEscape={false} closeOnInteractOutside={false} onOpenChange={opneChanged}>
	<Dialog.Trigger class="flex w-full items-center gap-3 px-3 py-2">
		<IconFolderGit class="size-4 flex-shrink-0 text-surface-500" />
		<span class="text-surface-900-50 text-sm whitespace-nowrap">
			{isEmpty ? 'Add Project...' : 'Manage Projects...'}
		</span>
	</Dialog.Trigger>

	<Portal>
		<Dialog.Backdrop class="fixed inset-0 z-50 bg-surface-50-950/50" />
		<Dialog.Positioner class="fixed inset-0 z-50 flex items-center justify-center p-4">
			<Dialog.Content
				class="relative flex h-[600px] w-full max-w-4xl flex-col overflow-hidden card bg-surface-100-900 shadow-xl"
			>
				<!-- 关闭按钮 - 移到这里,在内容之前 -->
				<Dialog.CloseTrigger
					class="absolute top-4 right-4 z-10 btn-icon preset-tonal"
					aria-label="关闭对话框"
				>
					<IconX class="size-5" />
				</Dialog.CloseTrigger>

				<!-- 对话框主体 - 两列布局 -->
				<div class="flex min-h-0 flex-1">
					<!-- 左列 - 仓库列表 -->
					<div class="flex w-1/3 flex-col border-r border-surface-200/30">
						<!-- 列表占位区域 -->
						<div class="flex-1 space-y-1 overflow-y-auto p-2 pt-6">
							<RepositoryList></RepositoryList>
						</div>
					</div>
					<!-- 右列 - 详情和操作 -->
					<div class="flex flex-1 flex-col">
						<!-- 版本信息 -->
						<div class="space-y-4 border-b border-surface-200/30 p-6">
							<Dialog.Title class="pr-12 text-2xl font-bold">
								vlogi.cc
								<span class="align-super text-xs opacity-50">
									v{softinfo.version}
								</span>
							</Dialog.Title>

							<div class="space-y-2 text-sm">
								<RepoInfo></RepoInfo>
							</div>
						</div>

						<!-- 操作按钮区域 - 占据剩余空间 -->
						<div class="flex flex-1 items-center justify-center p-6">
							<OpenProject></OpenProject>
						</div>

						<!-- 语言选择组件 -->
						<div class="border-t border-surface-200/30 p-6">
							<div class="flex items-center justify-between">
								<span class="pr-1 text-sm whitespace-nowrap opacity-60">语言</span>
								<select
									class="select min-w-32 preset-tonal"
									value={currentLanguage}
									onchange={(e) => changeLanguage(e.currentTarget.value)}
								>
									<option value="zh-CN">简体中文</option>
									<option value="zh-TW">繁體中文</option>
									<option value="en-US">English</option>
									<option value="ja-JP">日本語</option>
								</select>
							</div>
						</div>
					</div>
				</div>
				<!-- Toast Group 放在对话框内容中 -->
				<!-- Toast Group: 使用 absolute 定位，并添加 pointer-events 控制 -->
				<Toast.Group
					toaster={dialogToaster}
					class="pointer-events-none !absolute !inset-0 z-50 my-3 !p-4"
				>
					{#snippet children(toast)}
						<Toast {toast} class="pointer-events-auto py-4">
							<Toast.Message>
								<Toast.Title>{toast.title}</Toast.Title>
								<Toast.Description>{toast.description}</Toast.Description>
							</Toast.Message>
							<Toast.CloseTrigger />
						</Toast>
					{/snippet}
				</Toast.Group>
			</Dialog.Content>
		</Dialog.Positioner>
	</Portal>
</Dialog>

<style>
	/* 确保 select 元素在暗色模式下也有良好的样式 */
	select {
		cursor: pointer;
	}
</style>
