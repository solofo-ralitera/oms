import {app} from '../../services/app.js';
import {elasticMedia} from '../../services/elastic.js';

const BASE_URL = "BASE_URL";
const TRANSCODE_OUTPUT = "TRANSCODE_OUTPUT";

export class Summary extends HTMLElement {
    css = `<style type="text/css">
#summary-detail-content {
    font-size: .8em;
}
.pointer {
    cursor: pointer
}
.pointer:hover {
    text-decoration: underline;
}
    </style>`;
    
    constructor() {
        super();
        this.root = this.attachShadow({mode: "closed"});
        this.render();
    }

    renderCountByExtension(files_extension) {
        let str = '';
        Object.keys(files_extension).sort().forEach(extension => {
            if (extension.isVideoFile() && extension.toLowerCase() !== TRANSCODE_OUTPUT) {
                str += `<li class="extension pointer" data-extension="${extension.escape_quote()}" title="Transcode ${extension.escape_quote()} files to ${TRANSCODE_OUTPUT}">
                    ${extension}: ${files_extension[extension]}
                </li>`;
            } else {
                str += `<li>${extension}: ${files_extension[extension]}</li>`;
            }
        });
        return str;
    }

    summaryDetail() {
        Promise.all([
            elasticMedia.getAll(),
            app.getAllFiles(),
        ])
            .then(([elasticAll, allFiles]) => {
                const elasticNames = elasticAll.map(e => e.file_path.split(/[\\\/]/).pop());
                const difference = allFiles.filter(af => !elasticNames.find(ef => af.endsWith(ef)));
                return difference; 
            })
            .then(files => {
                this.root.querySelector("#summary-detail-content").innerHTML = `<br><br>
                    Files not indexed:
                    <ul>${files.map(f => `<li>${f}</li>`).join('')}</ul>
                `;
            })
            .catch(() => []);
    }

    render() {
        Promise.all([
            elasticMedia.totalCount(),
            app.summary(),
        ])
            .then(([elasticCount, dirSummary]) => this.root.innerHTML = `${this.css}
<article>
    <h3>Directory summary</h3>
    <p>
        Full path: ${BASE_URL}
    </p>
    <p>
        Number of files / indexed files: <span class="summary-detail-link pointer">${dirSummary.files_count} / ${elasticCount}</span>
        <span id="summary-detail-content"></span>
    </p>
    <p>
        Number of files by extension: 
        <ul>${this.renderCountByExtension(dirSummary.files_extension)}</ul>
    </p>
</article>        
            `)
            .then(() => {
                this.root.querySelector(".summary-detail-link")?.addEventListener("click", () => {
                    this.summaryDetail();
                });
                this.root.querySelectorAll("li.extension").forEach(li => li.addEventListener("click", e => {
                    app.transcodeDir(e.target.getAttribute("data-extension"));
                }));
            })
            .catch(() => {
                this.root.innerHTML = '...error...';
            });
    }
}

window.customElements.define('app-config-summary', Summary);
