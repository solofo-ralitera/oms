import {eventBus} from './EventBus.js';

export const history = new class {
    constructor() {
        window.addEventListener("popstate", ({state}) => {
            if(!state) return;
            eventBus.fire(state.event, state.data);
        });
    }

    pushHistory(event, data) {
        if (data.fromHistory) return;
        const state = { event, data: Object.assign({
            fromHistory: true,
        }, JSON.parse(JSON.stringify(data))) };
        const url = "/";        
        window.history.pushState(state, "", url);
    }
};
