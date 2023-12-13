import {eventBus} from '../helpers/EventBus.js';

export class SearchComponent extends HTMLElement {
    keyuptimer = 0;

    constructor() {
        super();
        this.root = this.attachShadow({mode: "closed"});
        this.render();
    }
    css() {
        return `
        <style type="text/css">
        input[type=search] {
            width: 100%;
            box-shadow: rgba(0, 0, 0, 0.24) 0px 3px 8px;
            border-radius: 5px;
            padding: 1em;
        }
        </style>
        `;
    }

    render() {
        this.root.innerHTML = `
            <header>
                ${this.css()}
                <input type="search" id="search" placeholder="Search...">
            </header>
        `;
        this.root.querySelector("#search")?.addEventListener("input", e => {
            window.clearTimeout(this.keyuptimer);
            const value = e.target.value;
            this.keyuptimer = window.setTimeout(() => {
                eventBus.fire("movie-search", value);
            }, 350);
        });
    }
}

window.customElements.define('app-search', SearchComponent);