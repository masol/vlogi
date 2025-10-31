import { eventBus } from "$lib/utils/evt";
import { invoke } from "@tauri-apps/api/core";
import { logger } from "./logger";

type SoftInfo = {
    version: string;
    initialized: boolean;
    prjarg: string; // 命令行传入的项目路径．
    locale: string; // 系统locale.
    pid: number; // rust后端进程的pid.
}

const INIT_TIMEOUT = 60000; // 60秒超时

export class InfoStore {
    version = "";
    locale = ""; // 系统预设locale,只读，并不需要responsive．
    prjarg = "";
    pid = 0;

    // 只在初始化时调用一次．(onMount处)
    async init() {
        const res = await invoke("get_soft_info") as SoftInfo;
        logger.debug("get_soft_info=", res)

        this.version = res.version;
        this.locale = res.locale;
        this.prjarg = res.prjarg;
        this.pid = res.pid;

        // 如果已经初始化完成，直接返回
        if (res.initialized) {
            return true;
        }

        // 等待初始化完成事件，带超时机制
        return this.waitForInitialization();
    }

    private async waitForInitialization(): Promise<boolean> {
        let timeoutId: number | null = null;
        let unsub: (() => void) | null = null;

        // 清理资源
        const cleanup = () => {
            if (timeoutId !== null) {
                clearTimeout(timeoutId);
                timeoutId = null;
            }
            if (unsub) {
                unsub();
                unsub = null;
            }
        };

        try {
            // 设置超时 Promise
            const timeoutPromise = new Promise<never>((_, reject) => {
                timeoutId = setTimeout(() => {
                    reject(new Error("Initialization timeout after 60 seconds"));
                }, INIT_TIMEOUT) as unknown as number;
            });

            // 设置初始化完成 Promise
            const initializationPromise = new Promise<boolean>((resolve) => {
                // 监听初始化事件
                eventBus.listen(eventBus.tauriEvt("inited"), async () => {
                    cleanup();
                    resolve(true);
                }).then(unsubscribe => {
                    unsub = unsubscribe;
                });

                // 轮询检查初始化状态
                const checkStatus = async () => {
                    while (true) {
                        const currentInfo = await invoke("get_soft_info") as SoftInfo;
                        if (currentInfo.initialized) {
                            cleanup();
                            resolve(true);
                            return;
                        }
                        // 等待一小段时间再检查，避免过于频繁
                        await new Promise(r => setTimeout(r, 1000));
                    }
                };

                checkStatus();
            });

            // 等待任意一个 Promise 完成（超时或初始化完成）
            const result = await Promise.race([initializationPromise, timeoutPromise]);
            cleanup();
            return result;
        } catch (error) {
            cleanup();
            throw error;
        }
    }
}

export const softinfo = new InfoStore();