import {app} from '../../services/app.js';
import {elasticMedia} from '../../services/elastic.js';

const BASE_URL = "BASE_URL";

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
        Object.entries(files_extension).forEach(([extension, count]) => {
            str += `<li class="extension pointer" data-extension="${extension.escape_quote()}" title="Transcode ${extension.escape_quote()} files">${extension}: ${count}</li>`;
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
                this.root.querySelector("#summary-detail-content").innerHTML = `
                    <br>
                    <br>
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
<header>
    <u>Directory summary</u>: ${BASE_URL}
</header>
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
