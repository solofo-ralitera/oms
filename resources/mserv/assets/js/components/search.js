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
    border: 1px solid #5f6368;
    box-shadow: none;
    border-radius: 0 0 11px 11px;
    padding: 15px 27px;
    border: 1px solid #5f6368;
    border-top: 0px;
    background-color: #000;
    outline: none;
    opacity: 0.91;
}
input[type=search]:focus {
    background-color: #303134;
    border: 1px solid #303134;
    opacity: 1;
}
</style>`;
    }

    render() {
        this.root.innerHTML = `${this.css()}
            <search>
                <input type="search" id="search" aria-label="Search" autofocus>
            </search>`;
        this.root.querySelector("#search").addEventListener("input", e => {
            window.clearTimeout(this.keyuptimer);
            const value = e.target.value;
            this.keyuptimer = window.setTimeout(() => {
                eventBus.fire("movie-search", value);
            }, 350);
        });
    }
}

window.customElements.define('app-search', SearchComponent);