import {eventBus} from '../services/EventBus.js';
import {history} from '../services/history.js';
import {app} from '../services/app.js';
import {elasticMedia} from '../services/elastic.js';
import {MetadataComponent} from './metadata.js';

const CSS = `<style type="text/css">
:host {
    position: relative;
}
.container {
    height: 100%;
}
ul {
    padding: 0;
}
ul li {
    display:inline;
}
.item~.item::before {
    content: ", ";
}
img {
    max-width: 55vw;
}
#poster {
    min-width: 295px;
    max-width: 691px;
}
.title {
    padding: 0 1em;
}
.summary {
    position: absolute;
    bottom: 0;
    left: 0;
    right: 0;
    line-height: 1.5em;
    padding: 1em 1em 2em 1em;
    background: rgb(0, 0, 0, 0.7);
    overflow: auto;
    max-height: 100vh;
}
.info {
    font-size: 0.8em;
}
.play {
    cursor: pointer;
    vertical-align: middle;
}
.pointer {
    cursor: pointer;
}
.pointer:hover {
    text-decoration: underline;
}
pre {
    white-space: pre-line;
    font-size: 0.9em;
}
time {
    font-size: 0.8em;
}
</style>`;

export class SummaryComponent extends HTMLElement {
 

    media = null;

    constructor() {
        super();
        this.root = this.attachShadow({mode: "closed"});
        this.render();

        eventBus.register("current-media", ({detail}) => {
            this.media = detail.media;
            history.pushHistory("current-media", detail);
            this.render();
        });

        document.addEventListener("keyup", e => {
            if (e.code === "Escape") {
                this.close();
            }
        })
    }

    close() {
        this.media = null;
        this.render();
    }

    renderPlay() {
        if (!this.media) return '';
        
        if (this.media?.file_type === "image") {
            return `<button class="play" role="button">🖼</button>`;
        } else if (["video", "audio"].includes(this.media.file_type)) {
            return `<button class="play" role="button">▶</button>`;
        } else if (this.media?.file_type === "pdf") {
            return `<button class="play" role="button">&#128462;</button>`;
        } else {
            return '';
        }
    }

    renderTranscode() {
        let extension = this.media.file_path.extension();
        let transcodeOutput = app.transcodeOutput(extension);
        if (this.media.file_path.isVideoFile() && !transcodeOutput.endsWith(extension)) {
            return `&nbsp;(<span class="info pointer transcode-path">transcode to ${transcodeOutput}</span>)`
        }
        return '';
    }

    renderSummary() {
        return `
        <pre>${this.media.summary}</pre>
        <ul class="info">
            <span class="all-cast pointer">Casts</span>:
            <li class="item cast pointer">${this.media.casts.join("</li><li class=\"item cast pointer\">").sanitize()}</li>
        </ul>
        <ul class="info"><li class="item genre pointer">${this.media.genres.join("</li><li class=\"item genre pointer\">").sanitize()}</li></ul>
        <footer>
            <span class="info pointer media-path" title="Sync metadata">${this.media.file_path.sanitize()}</span>
            ${this.renderTranscode()}
            <br>
            <time>${(this.media.duration?.secondsToHMS() ?? '').sanitize()}</time>
        </footer>        
        `;
    }

    renderEditMetadata() {
        const summary = this.root.querySelector("#summary");
        if (!summary) return;

        const elMetadata = new MetadataComponent();
        elMetadata.media = this.media;
        elMetadata.addEventListener("cancel", () => {
            this.render();
        });
        elMetadata.addEventListener("saved", () => {
            this.render();
        });

        summary.innerHTML = '';
        summary.append(elMetadata);
    }

    render() {
        if (!this.media) {
            this.root.innerHTML = '';
            return;
        };
        this.root.innerHTML = `${CSS}
<article class="container">
    <h2 class="title">
        ${this.renderPlay()}
        &nbsp;&nbsp;
        ${this.media.title.sanitize()} ${this.media.year ? `(<span class="pointer year">${this.media.year.sanitize()}</span>)` : ''}
    </h2>
    <div style="text-align:center;">
        <img 
            id="poster" 
            data-attempt="0" 
            data-filepath="${this.media.file_path.escape_path_attribute()}"
            src="${this.media.poster_url.escape_path_attribute()}"
            alt="${this.media.title.escape_quote()}">
    </div>
    <section class="summary" id="summary">
        ${this.renderSummary()}
    </section>
</article>`;

        this.root.querySelector(".year")?.addEventListener("click", e => {
            e.preventDefault();
            e.stopPropagation();
            eventBus.fire("navigate-search", {
                initiator: "summary.render.year",
                term: `year="${e.target.innerHTML.trim()}"`,
            });
        });
        this.root.querySelector("img")?.addEventListener("click", () => {
            this.close();
        });
        this.root.querySelector(".play")?.addEventListener("click", () => {
            app.openMedia(this.media);
        });
        this.root.querySelectorAll("li.genre")?.forEach(li => li.addEventListener("click", e => {
            eventBus.fire("navigate-search", {
                initiator: "summary.render.genre",
                term: `genres="${e.target.innerHTML.trim()}"`,
            });
            this.close();
        }));
        this.root.querySelector(".all-cast")?.addEventListener("click", () => {
            eventBus.fire("navigate-search", {
                initiator: "summary.render.cast",
                term: `:cast`,
            });
        });
        this.root.querySelectorAll("li.cast")?.forEach(li => li.addEventListener("click", e => {
            eventBus.fire("navigate-search", {
                initiator: "summary.render.casts",
                term: `casts="${e.target.innerHTML.trim()}"`,
            });
            this.close();
        }));

        /**
         * Update file metadata
         * If local file: display metadata form to fill manualy
         * If api: automaticaly update file metadata from api result
        */
        this.root.querySelector(".media-path")?.addEventListener("click", () => {
            if (this.media.provider === "local") 
                this.renderEditMetadata();
            else app.updateMetadata(this.media)
                // Drop index and rebuild new one
                .then(() => elasticMedia.deleteItem(this.media.hash))
                .then(() => app.scanDir(this.media.file_path))
                .catch(err => {
                    if (confirm("Unable to update the metadata, remove this index ?")) {
                        elasticMedia.deleteItem(this.media.hash);
                    }
                });
        });

        this.root.querySelector(".transcode-path")?.addEventListener("click", () => {
            app.transcodeDir(this.media.file_path);
        });
        
        this.root.querySelector("#poster")?.addEventListener("error", e => {
            let attempt = parseInt(e.target.getAttribute("data-attempt"));
            if (isNaN(attempt)) attempt = 0;
            if (attempt > 0) {
                e.target.src = "data:image/gif;base64,R0lGODlhAQABAIAAAP///wAAACH5BAEAAAAALAAAAAABAAEAAAICRAEAOw==";
            } else {
                attempt++;
                e.target.setAttribute("data-attempt", attempt);

                const thumb = `/poster${e.target.getAttribute("data-filepath")}`;
                this.media.poster_url = thumb;
                e.target.src = thumb;
            }
        });
        
    }
}

window.customElements.define('app-summary', SummaryComponent);