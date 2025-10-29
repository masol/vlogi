import { invoke } from "@tauri-apps/api/core";

type SysInfo = {
    version : string;
}

export class InfoStore {
    version = $state<string>("");


    // 只在初始化时调用一次．(onMount处)
    async init() {
        const res = await invoke("get_version") as SysInfo;
        this.version = res.version;
    }
}

export const infoStore = new InfoStore();
