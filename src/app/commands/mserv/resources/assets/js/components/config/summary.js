import {app} from '../../services/app.js';
import {elasticMovie} from '../../services/elastic.js';

const BASE_URL = "BASE_URL";

export class Summary extends HTMLElement {
    css = `<style type="text/css">
#summary-detail-content {
    font-size: .8em;
}
.summary-detail-link {
    cursor: pointer;
}
.summary-detail-link:hover {
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
            str += `<li>${extension}: ${count}</li>`;
        });
        return str;
    }

    summaryDetail() {
        Promise.all([
            elasticMovie.getAll(),
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
            elasticMovie.totalCount(),
            app.summary(),
        ])
            .then(([elasticCount, dirSummary]) => this.root.innerHTML = `${this.css}
<article>
<header>
    <u>Directory summary</u>: ${BASE_URL}
</header>
<p>
    Number of files / indexed files: <span class="summary-detail-link">${dirSummary.files_count} / ${elasticCount}</span>
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
            })
            .catch(() => {
                this.root.innerHTML = '...error...';
            });
    }
}

window.customElements.define('app-config-summary', Summary);
