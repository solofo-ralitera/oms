import {eventBus} from '../helpers/EventBus.js';

export class SearchComponent extends HTMLElement {
    keyuptimer = 0;

    constructor() {
        super();
        this.root = this.attachShadow({mode: "closed"});
        this.render();
    }
    css() {
        return `<style type="text/css">
input[type=search] {
    width: 100%;
    border-radius: 5px;
    padding: 1em;
    font-size: 1.2em;
}
</style>`;
    }

    render() {
        this.root.innerHTML = `
            <search>
                ${this.css()}
                <input type="search" id="search" placeholder="Search...">
            </search>
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