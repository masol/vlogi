import { softinfo } from "$lib/utils/softinfo";
import { repositoryStore } from "./config/ipc/repository.svelte";

export class ProjectStore {
    currentId = $state('');

    // Derived
    get currentRepository() {
        return repositoryStore.repositories.find(r => r.id === this.currentId);
    }

    // Actions
    setCurrentRepository(id: string) {
        this.currentId = id;
    }

    async init() {
        // @todo: 未响应repo.removed/repo.reset．响应了也只是一致性检查，不通过也只能给出内部错误提示(一致性检查如何处理)

        let sucOpend = false;
        // 首先检查是否有命令行请求打开/新建．
        if (softinfo.prjarg) {
            sucOpend = await this.loadProject(softinfo.prjarg);
        }

        if (!sucOpend && repositoryStore.openedRepos() === 0) {
            // 首次打开，读取配置opened project.
        }


    }


    // 打开/新建项目．
    async loadProject(path: string): Promise<boolean> {
        void (path)
        return true;
    }
}


export const projectStore = new ProjectStore();