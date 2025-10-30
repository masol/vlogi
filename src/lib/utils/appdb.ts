import Database from '@tauri-apps/plugin-sql';
import { eventBus } from './evt';
import JSON5 from 'json5'
import { pipe, map, unique } from 'remeda';
import { invoke } from "@tauri-apps/api/core";

type Change = {
    id: string;
    key: string;
    ctime: number;
}

export type ConfigItem = {
    id: string,
    value: Record<string, unknown>,
    created_at: number,
    updated_at: number
}

type ConfigInner = {
    id: string,
    key: string,
    value: string,
    created_at: number,
    updated_at: number
}

function ConfigMapper(inner: ConfigInner): ConfigItem {
    return {
        id: inner.id,
        value: JSON5.parse(inner.value),
        created_at: inner.created_at,
        updated_at: inner.updated_at
    }
}

class AppDB {
    #db!: Database;
    #latest: number = 0;
    #unsub: (() => void) | null = null;

    async init() {
        // @todo: appWindow.onCloseRequested(async (event) => 中清理．(以后改进)
        this.#unsub = await eventBus.listen(eventBus.tauriEvt("cfgchanged"), () => {
            this.chkChange();
        });

        this.#db = await Database.load("sqlite:app.db");

        // 获取最近时间的一条记录并存储 ctime
        const result = await this.#db.select<Array<{ ctime: number }>>(
            "SELECT ctime FROM change ORDER BY ctime DESC LIMIT 1"
        );

        this.#latest = result.length > 0 ? result[0].ctime : 0;

        if (result.length > 0) {
            //删除1小时以上的记录．
            const oneHourAgo = Math.floor(Date.now() / 1000) - (60 * 60);
            await this.#db.execute(
                "DELETE FROM change WHERE ctime < ?",
                [oneHourAgo]
            );
        }
    }

    // 1. 根据 key 进行 upsert（不存在则新建，存在且唯一则更新，存在多个则不操作）
    // @todo: 需要使用锁，否则并发访问数据库，此种分离处理可能会出错！
    async upsertByKey(key: string, value: string, bNotify: boolean = true): Promise<{ success: boolean, id: string | null }> {
        // 查询该 key 的所有记录
        const existing = await this.#db.select<Array<{ id: string, created_at: number }>>(
            'SELECT id, created_at FROM config WHERE key = $1',
            [key]
        );

        // 如果不存在记录，新建
        if (!existing || existing.length === 0) {
            const id = crypto.randomUUID();
            await this.#db.execute(
                'INSERT INTO config (id, key, value, created_at, updated_at) VALUES ($1, $2, $3, strftime(\'%s\', \'now\'), strftime(\'%s\', \'now\'))',
                [id, key, value]
            );
            if (bNotify)
                await this.recordConfigChange(key, id);
            return { success: true, id };
        }

        // 如果有多个记录，不操作，返回 false
        if (existing.length > 1) {
            console.warn(`Key "${key}" has multiple records (${existing.length}), upsert skipped.`);
            return { success: false, id: null };
        }

        // 只有一个记录时，执行更新
        const record = existing[0];
        await this.#db.execute(
            'UPDATE config SET value = $1, updated_at = strftime(\'%s\', \'now\') WHERE id = $2',
            [value, record.id]
        );
        if (bNotify)
            await this.recordConfigChange(key, record.id);
        return { success: true, id: record.id };
    }

    async upsertById(id: string, key: string, value: string, bNotify = true): Promise<{ success: boolean, id: string }> {
        // 查询该 ID 是否存在
        const existing = await this.#db.select<Array<{ id: string }>>(
            'SELECT id FROM config WHERE id = $1',
            [id]
        );

        // 如果不存在，新建
        if (!existing || existing.length === 0) {
            await this.#db.execute(
                'INSERT INTO config (id, key, value, created_at, updated_at) VALUES ($1, $2, $3, strftime(\'%s\', \'now\'), strftime(\'%s\', \'now\'))',
                [id, key, value]
            );
            if (bNotify)
                await this.recordConfigChange(key, id);
            return { success: true, id };
        }

        // 如果存在，更新
        await this.#db.execute(
            'UPDATE config SET key = $1, value = $2, updated_at = strftime(\'%s\', \'now\') WHERE id = $3',
            [key, value, id]
        );
        if (bNotify)
            await this.recordConfigChange(key, id);
        return { success: true, id };
    }

    // 2. 插入新记录
    async insert(key: string, value: string, bNotify = true): Promise<{ success: boolean, id: string | null }> {
        try {
            const id = crypto.randomUUID();
            await this.#db.execute(
                'INSERT INTO config (id, key, value, created_at, updated_at) VALUES ($1, $2, $3, strftime(\'%s\', \'now\'), strftime(\'%s\', \'now\'))',
                [id, key, value]
            );
            if (bNotify)
                await this.recordConfigChange(key, id);
            return { success: true, id };
        } catch (error) {
            console.error(`Failed to insert record with key "${key}":`, error);
            return { success: false, id: null };
        }
    }


    // 3. 根据 id 删除记录
    async remove(id: string, key: string, bNotify = true): Promise<{ success: boolean }> {
        try {
            // 先获取 key，以便记录变更
            const record = await this.getConfigById(id);

            await this.#db.execute(
                'DELETE FROM config WHERE id = $1',
                [id]
            );

            if (bNotify && record) {
                await this.recordConfigChange(key, id);
            }

            return { success: true };
        } catch (error) {
            console.error(`Failed to remove record with id "${id}":`, error);
            return { success: false };
        }
    }

    async getConfigsByKey(key: string): Promise<ConfigItem[]> {
        const results = await this.#db.select<ConfigInner[]>(
            'SELECT id, key, value, created_at, updated_at FROM config WHERE key = $1',
            [key]
        );
        return results.map(ConfigMapper);
    }

    async getConfigById(id: string): Promise<ConfigItem | null> {
        const results = await this.#db.select<ConfigInner[]>(
            'SELECT id, key, value, created_at, updated_at FROM config WHERE id = $1',
            [id]
        );
        return results.length > 0 ? ConfigMapper(results[0]) : null;
    }

    // 记录配置变更
    private async recordConfigChange(key: string, cfgid: string | null = null): Promise<void> {
        try {
            // 获取当前时间戳（秒）
            const now = Math.floor(Date.now() / 1000);

            // 插入变更记录
            await this.#db.execute(
                'INSERT INTO change (key, cfgid, ctime) VALUES ($1, $2, $3)',
                [key, cfgid, now]
            );

            // 更新 latest 时间戳
            this.#latest = now;

            // 通知变更．

            await invoke("emit_cfg_changed");
        } catch (error) {
            console.error(`Failed to record config change for key "${key}":`, error);
        }
    }

    private async chkChange() {
        const results = await this.#db.select<Change[]>(
            "SELECT * FROM change WHERE ctime > $1 ORDER BY ctime ASC",
            [this.#latest]
        );

        console.log("chkChange results=", results);

        if (results.length > 0) {
            // 提取唯一的 key 并为每个 key 触发一次事件
            const uniqueKeys: string[] = pipe(
                results,
                map((change: Change) => change.key),
                unique()
            );

            uniqueKeys.forEach(key => eventBus.emit(`cfgchanged:${key}`, {}));

            // 更新 latest 时间戳到最新的变更记录
            this.#latest = results[results.length - 1].ctime;
        }
    }

    close() {
        if (this.#unsub) {
            this.#unsub();
            this.#unsub = null;
        }
    }

}

export const appDB = new AppDB();