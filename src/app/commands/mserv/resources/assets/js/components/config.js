import { ScanDir } from "./config/scandir.js";
import { Summary } from "./config/summary.js";

export class ConfigComponent extends HTMLElement {
    css = `<style type="text/css">
:host {
    display: flex;
    justify-content: center;
    align-items: center;
    gap: 2em;
}
    </style>`;

    constructor() {
        super();
        this.root = this.attachShadow({mode: "closed"});
        this.render();
    }

    render() {
        this.root.innerHTML = `${this.css}
<app-config-summary></app-config-summary>
<app-config-scan-dir></app-config-scan-dir>
        `;
    }
}

window.customElements.define('app-config', ConfigComponent);