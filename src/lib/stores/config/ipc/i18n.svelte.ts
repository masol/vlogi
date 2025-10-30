// src/lib/stores/locale.svelte.ts

import {
    setLocale as internalSetLocale,
    baseLocale,
    locales,
    type Locale,
} from '$lib/paraglide/runtime.js';
import dayjs from 'dayjs';
import 'dayjs/locale/es'
import 'dayjs/locale/zh-cn'
import 'dayjs/locale/zh-tw'
import 'dayjs/locale/en'
import { eventBus } from '$lib/utils/evt';
import { appDB } from '$lib/utils/appdb';
import { softinfo } from '$lib/utils/softinfo';

/* ---------- 类型与常量 ---------- */

function isValidLocale(str: string): str is Locale {
    return (locales as unknown as string[]).includes(str);
}

/**
 * 把任何 BCP 47 tag 折叠成最接近的、项目里实际拥有的语言。
 * 例: en-US -> en, zh-HK -> zh-CN, de-AT -> de
 */
export function nearestLocale(
    wanted: string
): Locale | undefined {
    try {
        const loc = new Intl.Locale(wanted);
        // 1. 先试完整匹配
        if (isValidLocale(loc.baseName))
            return loc.baseName;
        // 2. 语言代码匹配（忽略地区）
        const lang = loc.language;
        return locales.find(a => new Intl.Locale(a).language === lang);
    } catch {
        return undefined;
    }
}


/* ---------- 工具 ---------- */
// function getPreferredLang(): Locale | undefined {
//     const nav = window.navigator;

//     const lang = nearestLocale(nav.language);
//     // console.log("nearestLocale lang=", lang)
//     if (lang) {
//         return lang;
//     }

//     if (isArray(nav.languages)) {
//         return nav.languages.find(isValidLocale) as Locale | undefined;
//     }
//     return undefined;
// }

class LocalStore {
    lang = $state<string>(baseLocale);
    #unsub: (() => void) | null = null;

    async setLocale(next: string, save = true) {
        if (next === this.lang) return;          // 没变直接跳过

        const realLang = nearestLocale(next);
        if (!realLang) {
            throw new Error(`Locale "${next}" is not available`);
        }

        await internalSetLocale(realLang, { reload: false });
        dayjs.locale(realLang);
        this.lang = realLang;
        eventBus.emit("langchange", realLang);

        // @todo: 这里存入config.
        if (save) {
            await appDB.upsertByKey("lang", JSON.stringify({ lang: realLang }));
        }
    };

    // 从数据库中加载lang配置，如果数据库未配置，则返回false.
    private async loadFromDB(): Promise<boolean> {
        const cfgs = await appDB.getConfigsByKey("lang");
        if (cfgs && cfgs.length > 0) {
            await this.setLocale(cfgs[0].value.lang as string, false);
            return true;
        }
        return false;
    }

    async init() {
        const loaded = await this.loadFromDB();
        if (!loaded && softinfo.locale) {
            await this.setLocale(softinfo.locale, false);
        }
        this.#unsub = await eventBus.listen("cfgchanged:lang", this.loadFromDB.bind(this))
    }

    close() {
        if (this.#unsub) {
            this.#unsub();
            this.#unsub = null;
        }
    }
}


// /* ---------- store 工厂 ---------- */
// function createLocaleStore() {
//     /* 内部真正保存当前语言的 writable */
//     const { subscribe, set: _set } = writable<Locale>(baseLocale);

//     /* 外部只读 getter，等价于原来的 langTag() */
//     const langTag = () => get({ subscribe }) as Locale;

//     /* 核心：切换语言 + 可选持久化 */
//     const setLocale = async (next: string, save = false) => {
//         if (next === langTag()) return;          // 没变直接跳过

//         const realLang = nearestLocale(next);
//         if (!realLang) {
//             throw new Error(`Locale "${next}" is not available`);
//         }

//         await internalSetLocale(realLang, { reload: false });
//         _set(realLang);                 // 真正更新 store
//         dayjs.locale(realLang);
//         eventBus.emit("langchange", realLang);

//         // @todo: 这里存入config.
//         if (save && typeof localStorage !== 'undefined') {
//             localStorage.setItem(STORAGE_KEY, next);
//         }
//     };

//     /* 自动检测：浏览器首选 -> store，可清除旧记录 */
//     const autoDetect = async (removeStorage = true) => {
//         if (typeof window === 'undefined') return;

//         if (removeStorage) localStorage.removeItem(STORAGE_KEY);

//         const pref = getPreferredLang();
//         if (pref) await setLocale(pref, false);
//     };

//     /* 初始化：storage -> navigator -> 默认  */
//     const init = async () => {
//         if (typeof window === 'undefined') return;

//         // console.log("getPreferredLang=", getPreferredLang())

//         let next: string | null =
//             localStorage.getItem(STORAGE_KEY) ?? getPreferredLang() ?? baseLocale;

//         if (!isValidLocale(next)) next = baseLocale;
//         if (next !== langTag()) await setLocale(next, false);
//     };

//     /* 对外 API */
//     return {
//         subscribe,
//         langTag,
//         setLocale,
//         autoDetect,
//         init,
//     };
// }

/* ---------- 导出单例 ---------- */
export const localeStore = new LocalStore(); //createLocaleStore();