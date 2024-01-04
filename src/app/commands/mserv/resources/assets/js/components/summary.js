import {eventBus} from '../services/EventBus.js';
import {history} from '../services/history.js';
import {app} from '../services/app.js';

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

    keyuptimer = 0;
    movie = null;

    constructor() {
        super();
        this.root = this.attachShadow({mode: "closed"});
        this.render();

        eventBus.register("current-movie", ({detail}) => {
            this.movie = detail.movie;
            history.pushHistory("current-movie", detail);
            this.render();
        });

        document.addEventListener("keyup", e => {
            if (e.code === "Escape") {
                this.close();
            }
        })
    }

    close() {
        this.movie = null;
        this.render();
    }

    renderPlay() {
        if (this.movie?.file_type === "image") {
            return `<button class="play" role="button">🖼</button>`;
        } else if (this.movie?.file_type === "movie") {
            return `<button class="play" role="button">▶</button>`;
        } else if (this.movie?.file_type === "pdf") {
            return `<button class="play" role="button">&#128462;</button>`;
        } else {
            return '';
        }
    }

    render() {
        if (!this.movie) {
            this.root.innerHTML = '';
            return;
        };
        this.root.innerHTML = `${this.css}
<article>
    <h2 class="title">
        ${this.renderPlay()}
        &nbsp;&nbsp;
        ${this.movie.title} ${this.movie.year ? `(${this.movie.year})` : ''}
    </h2>
    <div style="text-align:center;">
        <img 
            id="poster" 
            data-attempt="0" 
            data-filepath="${this.movie.file_path.escape_path_attribute()}"
            src="${this.movie.poster_url.escape_path_attribute()}" 
            alt="${this.movie.title.escape_quote()}">
    </div>
    <section class="summary">
        <p>${this.movie.summary}</p>
        <ul class="info"><li class="item cast pointer">${this.movie.casts.join("</li><li class=\"item cast pointer\">")}</li></ul>
        <ul class="info"><li class="item genre pointer">${this.movie.genres.join("</li><li class=\"item genre pointer\">")}</li></ul>
        <footer>
            <span class="info pointer movie-path">${this.movie.file_path}</span>
            <br>
            <time>${this.movie.duration?.secondsToHMS() ?? ''}</time>
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
            app.openItem(this.movie);
        });
        this.root.querySelectorAll("li.genre").forEach(li => li.addEventListener("click", e => {
            eventBus.fire("navigate-search", {
                term: `:genre ${e.target.innerHTML.trim()}`,
            });
            this.close();
        }));
        this.root.querySelectorAll("li.cast").forEach(li => li.addEventListener("click", e => {
            eventBus.fire("navigate-search", {
                term: `:cast ${e.target.innerHTML.trim()}`,
            });
            this.close();
        }));
        this.root.querySelector(".movie-path")?.addEventListener("click", () => {
            const text = this.movie.file_path.split(/\/|\\/).pop();
            try {
                navigator.clipboard.writeText(text);
            } catch (_) {
                const selBox = window.document.createElement('textarea');
                selBox.style.position = 'fixed';
                selBox.style.left = '0';
                selBox.style.top = '0';
                selBox.style.opacity = '0';
                selBox.value = text;
                document.body.appendChild(selBox);
                selBox.focus();
                selBox.select();
                document.execCommand('copy');
                document.body.removeChild(selBox);
            }
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
                this.movie.poster_url = thumb;
                e.target.src = thumb;
            }
        });
        
    }
}

window.customElements.define('app-summary', SummaryComponent);