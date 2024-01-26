import {eventBus} from '../services/EventBus.js';
import {history} from '../services/history.js';
import {app} from '../services/app.js';

export class SearchComponent extends HTMLElement {
    css = `<style type="text/css">
search {
    position: relative;
    display: grid;
    grid-template-columns: 1fr;   
    opacity: 0.91;
    border-bottom: 1px solid #5f6368;
    border-top: 0px;
}
input[type=search] {
    width: 100%;
    border: 1px solid #5f6368;
    box-shadow: none;
    padding: 15px 1em;
    background-color: #000;
    outline: none;
}
input[type=search]:focus {
    background-color: #303134;
    border: 1px solid #303134;
    opacity: 1;
}
</style>`;
    keyuptimer = 0;

    constructor() {
        super();
        this.root = this.attachShadow({mode: "closed"});
        this.render();

        history.pushHistory("navigate-search", {
            initiator: "search.constructor.pushHistory",
            term: "",
        });

        eventBus.register("navigate-search", ({detail}) => {
            this.setValue(detail);
        });
    }

    setValue(data) {
        this.root.querySelector("#search").value = data.term;
        this.search(data);
    }

    search(data = {}) {
        const term = this.root.querySelector("#search").value;
        if (typeof data.term === "undefined") data.term = term;
        window.clearTimeout(this.keyuptimer);
        this.keyuptimer = window.setTimeout(() => {
            history.pushHistory("navigate-search", {
                initiator: "search.search.keyuptimer.history",
                ...data,
            });
            app.saveSearchTerm(term);

            eventBus.fire("current-media", {
                media: null,
                fromHistory: true,
            });

            if (term.startsWith(":setting") || term.startsWith(":parameter")) {
                eventBus.fire("display-config", null);
                return;
            }

            if (term === ":genre" || term === ":genres" ) {
                eventBus.fire("display-genre", null);
                return;
            }

            if (term === ":cast" || term === ":casts" ) {
                eventBus.fire("display-cast", null);
                return;
            }
            eventBus.fire("media-search", term);
        }, 350);
    }

    render() {
        this.root.innerHTML = `${this.css}
            <search>
                <input placeholder="Search" type="search" id="search" aria-label="Search" autocomplete="off">
            </search>`;
        this.root.querySelector("#search").addEventListener("input", e => {
            this.search();
        });
        this.root.querySelector("#search").addEventListener("keypress", e => {
            if (e.code === "Enter") {
                this.search();
            }
        });
    }
}

window.customElements.define('app-search', SearchComponent);
