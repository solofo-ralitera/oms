import { ScanDir } from "./config/scandir.js";
import { Summary } from "./config/summary.js";
import { Genres } from "./config/genres.js";

export class ConfigComponent extends HTMLElement {
    css = `<style type="text/css">
:host {
    display: flex;
    gap: 1em;
    width: 100%;
    flex-wrap: wrap;
}
:host > * {
    box-shadow: inset 0 0 10px green;
    padding: 1em;
}
    </style>`;

    constructor() {
        super();
        this.root = this.attachShadow({mode: "closed"});
        this.render();
    }

    render() {
        this.root.innerHTML = `${this.css}
        <app-config-genres></app-config-genres>
        <app-config-scan-dir></app-config-scan-dir>
        <app-config-summary></app-config-summary>
        `;
    }
}

window.customElements.define('app-config', ConfigComponent);