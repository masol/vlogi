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
import { m } from '$lib/paraglide/messages.js';

/* ---------- 类型与常量 ---------- */

type MessageKey = keyof typeof m;
type MessageParams<K extends MessageKey> = Parameters<typeof m[K]>;

/**
 * 创建响应式翻译
 * @param key - 翻译键
 * @param params - 翻译参数（可选）
 */
/**
 * 创建响应式翻译
 */
export function t<K extends MessageKey>(
    key: K | string, // 添加string,因为添加元素会让lint报错，每次需要重建缓冲，否则报错．不密集翻译时，移除 | string
    ...params: MessageParams<K>
): string {
    // 模板本身就是响应式上下文，Svelte 会自动追踪 t() 函数内部对 localeStore.lang 的访问。
    const _ = localeStore.lang;
    void (_);
    return Reflect.apply(m[key as K], null, params) as string;
}

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

/* ---------- 导出单例 ---------- */
export const localeStore = new LocalStore(); //createLocaleStore();