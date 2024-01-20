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
#transcode-dir {
    text-align: left;
}
    </style>`;
    
    constructor() {
        super();
        this.root = this.attachShadow({mode: "closed"});
        this.render();
    }

    transcodeOutput() {
        return TRANSCODE_OUTPUT
            .split(",")
            .map(output => output.replace(">", " -> "))
            .join("<br>");
    }

    render() {
        this.root.innerHTML = `${this.css}
<article>
    <h3>Tools</h3>
    <section class="container">
        <button id="scan-dir">
            Scan directory
        </button>
        <button id="update-metadata">
            Update all movie file metadata
        </button>
        <button id="transcode-dir">
            Transcode directory (x${TRANSCODE_THREAD.sanitize()})
            <br>
            <span style="font-size:0.9em;color:lightgray;">${app.transcodeOutput()}</span>            
        </button>
    </section>        
</article>`;

        this.root.querySelector("#scan-dir").addEventListener("click", () => {
            app.scanDir();
        });
        this.root.querySelector("#update-metadata").addEventListener("click", () => {
            app.updateMetadata();
        });
        this.root.querySelector("#transcode-dir").addEventListener("click", () => {
            app.transcodeDir();
        });
    }
}

window.customElements.define('app-config-scan-dir', ScanDir);