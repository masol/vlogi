// src/lib/stores/repository.svelte.ts
import { appDB } from '$lib/utils/appdb';
import { eventBus } from '$lib/utils/evt';
import { softinfo } from '$lib/utils/softinfo';


export interface Repository {
    id: string;
    name: string;
    path: string;
    ver: string;
    ctime: number;
    owner: number; // 此项目当前被谁打开的? 空表示未打开．
}

type RepoValue = {
    name: string;
    path: string;
    ver: string;
    owner: number;
}

const KEYNAME = 'repository'

function Repo2Value(repo: Repository): RepoValue {
    return {
        name: repo.id,
        path: repo.path,
        ver: repo.ver || softinfo.version,
        owner: repo.owner || 0,
    }
}

class RepositoryStore {
    private unsub: (() => void) | null = null;

    repositories = $state<Repository[]>([
    ]);

    selectedId = $state('');

    find(id: string) {
        return this.repositories.find(r => r.id === id);
    }

    openedRepos(): number {
        return this.repositories.filter(repo => repo.owner !== 0).length;
    }

    setSelectedRepo(id: string) {
        this.selectedId = id;
    }

    setRepositories(repositories: Repository[]) {
        this.repositories = repositories;

        eventBus.emit<"repo.reset">("repo.reset", { length: repositories.length })

        // 检查 currentId 是否在新数组中有效，
        // @todo: 支持这个一致性检查，在projectStore中－－单向依赖．
        // if (this.currentId && !repositories.some(r => r.id === this.currentId)) {
        //     this.currentId = '';
        // }

        // 检查 selectedId 是否在新数组中有效
        if (this.selectedId && !repositories.some(r => r.id === this.selectedId)) {
            this.selectedId = '';
        }
    }

    private async updateDb(id: string, value: RepoValue) {
        await appDB.upsertById(id, KEYNAME, JSON.stringify(value), true)
    }

    async addRepository(repository: Repository) {
        this.repositories = [...this.repositories, repository];
        await this.updateDb(repository.id, Repo2Value(repository));
    }

    async removeRepository(id: string) {
        this.repositories = this.repositories.filter(r => r.id !== id);

        eventBus.emit<"repo.removed">("repo.removed", { id })
        // 如果删除的是当前仓库,设置为空．
        // if (this.currentId === id) {
        //     this.currentId = "";
        // }
        await appDB.remove(id, KEYNAME, true)
    }

    async updateRepository(id: string, updates: Partial<Omit<Repository, 'id'>>) {
        this.repositories = this.repositories.map(repo =>
            repo.id === id ? { ...repo, ...updates } : repo
        );
        await this.updateDb(id, Repo2Value(updates as Repository));
    }


    // 从数据库中加载lang配置，如果数据库未配置，则返回false.
    private async loadFromDB(): Promise<boolean> {
        const cfgs = await appDB.getConfigsByKey(KEYNAME);
        if (cfgs) {
            this.setRepositories(cfgs.map(cfg => {
                return {
                    id: cfg.id,
                    ctime: cfg.created_at,
                    name: (cfg.value as RepoValue).name,
                    ver: (cfg.value as RepoValue).ver || softinfo.version,
                    path: (cfg.value as RepoValue).path,
                    owner: (cfg.value as RepoValue).owner || 0,
                }
            }))
            return true;
        }
        return false;
    }

    async init() {
        await this.loadFromDB();
        this.unsub = await eventBus.listen("cfgchanged:repository", this.loadFromDB.bind(this))
    }

    close() {
        if (this.unsub) {
            this.unsub();
            this.unsub = null;
        }
    }
}

export const repositoryStore = new RepositoryStore();