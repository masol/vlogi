

export type EffectType = 'spinner' | 'pulse' | 'dots' | 'ring' | 'random';
export class LoadingStore {
    showBackdrop = $state(false);
    effect = $state<EffectType>("random");
    text = $state("");

    hide() {
        this.showBackdrop = false;
    }
    show(text?: string, effect?: EffectType) {
        if (effect) {
            this.effect = effect;
        }
        this.text = text ?? "";
        this.showBackdrop = true;
    }
}

export const loadingStore = new LoadingStore();