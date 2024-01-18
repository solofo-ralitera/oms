import {eventBus} from '../services/EventBus.js';
import {history} from '../services/history.js';
import {app} from '../services/app.js';

const TRANSCODE_OUTPUT = "TRANSCODE_OUTPUT";

export class SummaryComponent extends HTMLElement {
    css = `<style type="text/css">
:host {
    position: relative;
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
}
.title {
    padding: 0 1em;
}
.summary {
    position: fixed;
    bottom: 0;
    line-height: 1.5em;
    padding: 1em;
    background-color: black;
    opacity: 0.7;
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
time {
    font-size: 0.8em;
}
</style>`;

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

    render() {
        if (!this.media) {
            this.root.innerHTML = '';
            return;
        };
        this.root.innerHTML = `${this.css}
<article>
    <h2 class="title">
        ${this.renderPlay()}
        &nbsp;&nbsp;
        ${this.media.title} ${this.media.year ? `(${this.media.year})` : ''}
    </h2>
    <div style="text-align:center;">
        <img 
            id="poster" 
            data-attempt="0" 
            data-filepath="${this.media.file_path.escape_path_attribute()}"
            src="${this.media.poster_url.escape_path_attribute()}"
            alt="${this.media.title.escape_quote()}">
    </div>
    <section class="summary">
        <p>${this.media.summary}</p>
        <ul class="info">
            <span class="all-cast pointer">Casts</span>:
            <li class="item cast pointer">${this.media.casts.join("</li><li class=\"item cast pointer\">")}</li>
        </ul>
        <ul class="info"><li class="item genre pointer">${this.media.genres.join("</li><li class=\"item genre pointer\">")}</li></ul>
        <footer>
            <span class="info pointer media-path">${this.media.file_path}</span>
            ${this.renderTranscode()}
            <br>
            <time>${this.media.duration?.secondsToHMS() ?? ''}</time>
        </footer>
    </section>
</article>`;

        this.root.querySelector(".title").addEventListener("click", () => {
            this.close();
        });
        this.root.querySelector("img").addEventListener("click", () => {
            this.close();
        });
        this.root.querySelector(".play")?.addEventListener("click", () => {
            app.openMedia(this.media);
        });
        this.root.querySelectorAll("li.genre").forEach(li => li.addEventListener("click", e => {
            eventBus.fire("navigate-search", {
                term: `genres="${e.target.innerHTML.trim()}"`,
            });
            this.close();
        }));
        this.root.querySelector(".all-cast")?.addEventListener("click", () => {
            eventBus.fire("navigate-search", {
                term: `:cast`,
            });
        });
        this.root.querySelectorAll("li.cast").forEach(li => li.addEventListener("click", e => {
            eventBus.fire("navigate-search", {
                term: `casts="${e.target.innerHTML.trim()}"`,
            });
            this.close();
        }));
        this.root.querySelector(".media-path")?.addEventListener("click", () => {
            const text = this.media.file_path.split(/\/|\\/).pop();
            if (text) text.toClipBoard();
        });
        this.root.querySelector(".transcode-path")?.addEventListener("click", () => {
            app.transcodeDir(this.media.file_path);
        });
        
        this.root.querySelector("#poster").addEventListener("error", e => {
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