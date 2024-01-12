import { ScanDir } from "./config/scandir.js";
import { Summary } from "./config/summary.js";
import { Genres } from "./config/genres.js";
import { ServiceLog } from "./config/service-log.js";
import { Prerequistes } from "./config/prerequisites.js";

export class ConfigComponent extends HTMLElement {
    css = `<style type="text/css">
.config-container {
    display: flex;
    gap: 1em;
    width: 100%;
    flex-wrap: wrap;
    margin: 1em 0;
}
.config-container > * {
    box-shadow: inset 0 0 10px green;
    padding: 1em;
}
footer {
    font-size: 0.8em;
}
    </style>`;

    constructor() {
        super();
        this.root = this.attachShadow({mode: "closed"});
        this.render();
    }

    render() {
        this.root.innerHTML = `${this.css}
        <div class="config-container">
            <app-config-prerequistes></app-config-prerequistes>
        </div>
        <div class="config-container">
            <app-config-genres></app-config-genres>
            <app-config-scan-dir></app-config-scan-dir>
            <app-config-summary></app-config-summary>
            <app-config-service-log></app-config-service-log>
        </div>
        `;
    }
}

window.customElements.define('app-config', ConfigComponent);