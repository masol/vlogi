// src/lib/stores/locale.ts
import { writable, get } from 'svelte/store';
import {
    setLocale as internalSetLocale,
    baseLocale,
    locales,
    type Locale,
} from '$lib/paraglide/runtime.js';
import { isArray } from 'remeda';

/* ---------- 类型与常量 ---------- */
const STORAGE_KEY = 'locale';

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
function getPreferredLang(): Locale | undefined {
    const nav = window.navigator;

    const lang = nearestLocale(nav.language);
    // console.log("nearestLocale lang=", lang)
    if (lang) {
        return lang;
    }

    if (isArray(nav.languages)) {
        return nav.languages.find(isValidLocale) as Locale | undefined;
    }
    return undefined;
}



/* ---------- store 工厂 ---------- */
function createLocaleStore() {
    /* 内部真正保存当前语言的 writable */
    const { subscribe, set: _set } = writable<Locale>(baseLocale);

    /* 外部只读 getter，等价于原来的 langTag() */
    const langTag = () => get({ subscribe }) as Locale;

    /* 核心：切换语言 + 可选持久化 */
    const setLocale = async (next: string, save = false) => {
        if (next === langTag()) return;          // 没变直接跳过

        if (!isValidLocale(next)) {
            throw new Error(`Locale "${next}" is not available`);
        }

        await internalSetLocale(next, { reload: false });
        _set(next);                              // 真正更新 store

        if (save && typeof localStorage !== 'undefined') {
            localStorage.setItem(STORAGE_KEY, next);
        }
    };

    /* 自动检测：浏览器首选 -> store，可清除旧记录 */
    const autoDetect = async (removeStorage = true) => {
        if (typeof window === 'undefined') return;

        if (removeStorage) localStorage.removeItem(STORAGE_KEY);

        const pref = getPreferredLang();
        if (pref) await setLocale(pref, false);
    };

    /* 初始化：storage -> navigator -> 默认  */
    const init = async () => {
        if (typeof window === 'undefined') return;

        // console.log("getPreferredLang=", getPreferredLang())

        let next: string | null =
            localStorage.getItem(STORAGE_KEY) ?? getPreferredLang() ?? baseLocale;

        if (!isValidLocale(next)) next = baseLocale;
        if (next !== langTag()) await setLocale(next, false);
    };

    /* 对外 API */
    return {
        subscribe,
        langTag,
        setLocale,
        autoDetect,
        init,
    };
}

/* ---------- 导出单例 ---------- */
export const localeStore = createLocaleStore();