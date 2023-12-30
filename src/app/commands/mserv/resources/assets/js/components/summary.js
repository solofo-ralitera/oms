import {eventBus} from '../services/EventBus.js';

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

        eventBus.register("current-movie", e => {
            this.movie = e.detail;
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

    render() {
        if (!this.movie) {
            this.root.innerHTML = '';
            return;
        };
        this.root.innerHTML = `${this.css}
<article>
    <h2 class="title">
        <button class="play" role="button">▶</button>
        &nbsp;&nbsp;
        ${this.movie.title} (${this.movie.year})
    </h2>
    <div style="text-align:center;">
        <img src="${this.movie.poster_url}" alt="${this.movie.title.escape_quote()}">
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
            eventBus.fire("play-movie", JSON.parse(JSON.stringify(this.movie)));
        });
        this.root.querySelectorAll("li.genre").forEach(li => li.addEventListener("click", e => {
            eventBus.fire("navigate-search", `:genre ${e.target.innerHTML.trim()}`);
        }));
        this.root.querySelectorAll("li.cast").forEach(li => li.addEventListener("click", e => {
            eventBus.fire("navigate-search", `:cast ${e.target.innerHTML.trim()}`);
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
        
    }
}

window.customElements.define('app-summary', SummaryComponent);