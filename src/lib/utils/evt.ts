import { listen, emit } from '@tauri-apps/api/event';
import mitt from 'mitt';
import { type EventType } from 'mitt';

type BusEvt = Record<EventType, unknown>;
const TauriEvtPrefix = "tauri://";

class EventBus {
	private static instance: EventBus;
	private emitter = mitt<BusEvt>();

	private constructor() { }

	static get inst(): EventBus {
		if (!EventBus.instance) {
			EventBus.instance = new EventBus();
		}
		return EventBus.instance;
	}

	/**
	 * 订阅事件
	 * @param type 事件名(以tauri::开头的由Tauri发出的事件，并会)
	 * @param handler 事件处理器
	 * @returns 调用返回的函数即可取消订阅
	 */
	async listen<K extends keyof BusEvt>(type: string, handler: (event: BusEvt[K]) => void): Promise<() => void> {
		if (type.startsWith(TauriEvtPrefix)) {
			return listen(type, handler);
		}

		this.emitter.on(type as K, handler);
		// 返回一个 unlisten 函数
		return () => {
			this.emitter.off(type as K, handler);
		};
	}

	emit<K extends keyof BusEvt>(type: string, event: BusEvt[K]) {
		if (type.startsWith(TauriEvtPrefix)) {
			emit(type, event)
		}
		this.emitter.emit(type, event);
	}
}

export const eventBus = EventBus.inst;