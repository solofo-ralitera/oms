import {app} from '../services/app.js';

export class ConfigComponent extends HTMLElement {
    css = `<style type="text/css">

    </style>`;

    constructor() {
        super();
        this.root = this.attachShadow({mode: "closed"});
        this.render();
    }

    render() {
        this.root.innerHTML = `${this.css}
<section>
    <button id="scan-dir">Scan directory</button>
</section>        
        `;

        this.root.querySelector("#scan-dir").addEventListener("click", () => {
            app.scanDir();
        });
    }
}

window.customElements.define('app-config', ConfigComponent);