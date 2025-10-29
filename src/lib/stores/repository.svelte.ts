// src/lib/stores/repository.svelte.ts

export interface Repository {
    id: string;
    name: string;
    path: string;
    ver: string;
    ctime: number;
}

class RepositoryStore {
    repositories = $state<Repository[]>([
        { id: '1', name: 'Personal Notes', path: '/home/user/notes', ver: "0.1.0", ctime: 123123213 },
        { id: '2', name: 'Work Documents', path: '/home/user/work', ver: "0.1.0", ctime: 123123213 },
        { id: '3', name: 'Research Papers and Academic Writing', path: '/home/user/research', ver: "0.1.0", ctime: 123123213 }
    ]);

    currentId = $state('1');

    selectedId = $state('');

    // Derived
    get currentRepository() {
        return this.repositories.find(r => r.id === this.currentId);
    }

    // Actions
    setCurrentRepository(id: string) {
        this.currentId = id;
    }

    setSelectedRepo(id: string) {
        this.selectedId = id;
    }

    setRepositories(repositories: Repository[]) {
        this.repositories = repositories;

        // 检查 currentId 是否在新数组中有效
        if (this.currentId && !repositories.some(r => r.id === this.currentId)) {
            this.currentId = '';
        }

        // 检查 selectedId 是否在新数组中有效
        if (this.selectedId && !repositories.some(r => r.id === this.selectedId)) {
            this.selectedId = '';
        }
    }

    addRepository(repository: Repository) {
        this.repositories = [...this.repositories, repository];
    }

    async removeRepository(id: string) {
        this.repositories = this.repositories.filter(r => r.id !== id);

        // 如果删除的是当前仓库,切换到第一个
        if (this.currentId === id && this.repositories.length > 0) {
            this.currentId = this.repositories[0].id;
        }
    }

    async updateRepository(id: string, updates: Partial<Omit<Repository, 'id'>>) {
        this.repositories = this.repositories.map(repo =>
            repo.id === id ? { ...repo, ...updates } : repo
        );
    }

    openManagement() {
        console.log('Opening repository management...');
        // 实现管理界面打开逻辑
    }
}

export const repositoryStore = new RepositoryStore();