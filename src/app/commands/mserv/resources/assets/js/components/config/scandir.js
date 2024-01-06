import {app} from '../../services/app.js';

const TRANSCODE_OUTPUT = "TRANSCODE_OUTPUT";
const TRANSCODE_THREAD = "TRANSCODE_THREAD";

export class ScanDir extends HTMLElement {
    css = `<style type="text/css">
.container {
    display: grid;
    grid-template-rows: 1fr;
    grid-gap: 1em;
}
button {
    padding: 1em;
    width: 100%;
}
    </style>`;
    
    constructor() {
        super();
        this.root = this.attachShadow({mode: "closed"});
        this.render();
    }

    render() {
        this.root.innerHTML = `${this.css}
<section class="container">
    <button id="scan-dir">Scan directory</button>
    <button id="transcode-dir">
        Transcode directory
        <br>
        (${TRANSCODE_OUTPUT} x ${TRANSCODE_THREAD})
    </button>
</section>        
        `;

        this.root.querySelector("#scan-dir").addEventListener("click", () => {
            app.scanDir();
        });
        this.root.querySelector("#transcode-dir").addEventListener("click", () => {
            app.transcodeDir();
        });
    }
}

window.customElements.define('app-config-scan-dir', ScanDir);