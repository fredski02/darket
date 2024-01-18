import { getCurrentInstance } from 'vue'

export default function useEventBus() {
    const internalInstance = getCurrentInstance();
    const emitter = internalInstance.appContext.config.globalProperties.emitter;

    return emitter;
}
