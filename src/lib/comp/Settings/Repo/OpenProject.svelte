<script lang="ts">
	import IconPlus from '~icons/lucide/plus';
	import IconFolderOpen from '~icons/lucide/folder-open';
	// import { m } from '$lib/paraglide/messages.js';
	import { localeStore, t } from '$lib/stores/config/ipc/i18n.svelte';

	function createNewProject() {
		console.log('创建新项目');
	}

	function openLocalProject() {
		console.log('打开本地项目');
	}
</script>

<div class="flex items-center justify-center">
	<div class="w-full max-w-2xl space-y-4 px-4">
		<!-- 新建项目 -->
		<div class="flex items-center justify-between gap-4">
			<div class="flex flex-1 flex-col items-start gap-0.5 text-left">
				<span class="text-base leading-tight font-medium">
					{t('zippy_whole_elephant_support')}
				</span>
				<span class="text-sm leading-tight opacity-75">
					{t('helpful_mean_jurgen_roam')}
				</span>
			</div>

			<button type="button" class="btn preset-filled-primary-500" onclick={createNewProject}>
				<IconPlus class="size-5" />
				<span>新建</span>
			</button>
		</div>

		<hr class="hr" />
		<!-- 打开本地项目 -->
		<div class="flex items-center justify-between gap-4">
			<div class="flex flex-1 flex-col items-start gap-0.5 text-left">
				<span class="text-base leading-tight font-medium"> 打开本地项目 </span>
				<span class="text-sm leading-tight opacity-75">
					将一个文件夹作为项目仓库在 vlogi.cc 中打开
				</span>
			</div>

			<button type="button" class="btn preset-tonal-surface" onclick={openLocalProject}>
				<IconFolderOpen class="size-5" />
				<span>打开</span>
			</button>
		</div>
	</div>
</div>
