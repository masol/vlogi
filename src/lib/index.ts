// Reexport your entry components here
import { localeStore } from "./stores/i18n.js"
import { infoStore } from "./stores/info.svelte.js";
import { appDB } from "./utils/appdb.js";

export async function init() {
    await Promise.all([
        appDB.init(),
        localeStore.init(),
        infoStore.init()
    ]);
}