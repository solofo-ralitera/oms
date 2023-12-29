import {app} from '../../services/app.js';

export class ScanDir extends HTMLElement {
    css = `<style type="text/css">
#scan-dir {
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
<section>
    <button id="scan-dir">Scan directory</button>
</section>        
        `;

        this.root.querySelector("#scan-dir").addEventListener("click", () => {
            app.scanDir();
        });
    }
}

window.customElements.define('app-config-scan-dir', ScanDir);