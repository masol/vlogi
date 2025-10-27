
export class PanelStore {
    #show = $state<boolean>(false);
    #size = $state<number>(10);

    constructor(show: boolean, size: number) {
        this.#show = show;
        this.#size = size;
    }

    get show() {
        return this.#show;
    }

    get size() {
        return this.#size;
    }

    set show(value) {
        this.#show = value;
    }

    set size(value) {
        this.#size = value;
    }
}

export const leftPanel = new PanelStore(true, 20);
export const rightPanel = new PanelStore(true, 20);

export function calcMainSize() {
    let mainSize = 100;
    if (leftPanel.show) {
        mainSize -= leftPanel.size;
    }
    if (rightPanel.show) {
        mainSize -= rightPanel.size;
    }
    return mainSize
}