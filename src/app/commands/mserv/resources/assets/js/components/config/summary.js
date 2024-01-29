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
            if (extension.isVideoFile()) {
                str += `<li class="extension pointer" data-extension="${extension.escape_quote()}" title="Transcode to ${app.transcodeOutput(extension).escape_quote()}">
                    ${extension.sanitize()}: ${files_extension[extension].sanitize()}
                </li>`;
            } else {
                str += `<li>${extension.sanitize()}: ${files_extension[extension].sanitize()}</li>`;
            }
        });
        return str;
    }

    summaryDetail() {
        Promise.all([
            elasticMedia.getAll(),
            app.getAllFiles(),
        ]).then(([elasticAll, allFiles]) => {
            const elasticFiles = elasticAll.map(f => f.file_path);
            // check full path
            const difference = allFiles.filter(allFile => !elasticFiles.find(elasticFile => allFile.endsWith(elasticFile)));
            return difference; 
        }).then(files => {
            if (!files.length) return;
            this.root.querySelector("#summary-detail-content").innerHTML = `<br><br>
                Files not indexed:
                <ul>${files.map(f => `<li role="button" class="not-indexed pointer" data-filepath="${f.escape_path_attribute()}">${f.file_name().sanitize()}</li>`).join('')}</ul>
            `;
            this.root.querySelectorAll(".not-indexed").forEach(li => li.addEventListener("click", e => {
                app.scanDir(e.target.getAttribute("data-filepath"));
            }));
        }).catch(() => []);
    }

    render() {
        Promise.all([
            elasticMedia.totalCount(),
            app.summary(),
        ])
            .then(([elasticCount, dirSummary]) => this.root.innerHTML = `${this.css}
<article>
    <h3>
        Directory summary
        <button id="refresh-button">&#10227;</button>
    </h3>
    <p>
        Full path: ${BASE_URL}
    </p>
    <p>
        Number of files / indexed files: <span class="summary-detail-link pointer">${dirSummary.files_count.sanitize()} / ${elasticCount.sanitize()}</span>
        <span id="summary-detail-content"></span>
    </p>
    <p>
        Number of files by extension: 
        <ul>${this.renderCountByExtension(dirSummary.files_extension)}</ul>
    </p>
</article>        
            `)
            .then(() => {
                this.root.querySelector("#refresh-button").addEventListener("click", () => {
                    this.render();
                });
                
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
