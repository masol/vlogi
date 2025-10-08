// Reexport your entry components here
import { localeStore } from "./stores/i18n.js"


export async function init() {
    await localeStore.init();
}