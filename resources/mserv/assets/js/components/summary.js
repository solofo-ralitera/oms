import {eventBus} from '../helpers/EventBus.js';

export class SummaryComponent extends HTMLElement {
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
    css() {
        return `
<style type="text/css">
:host {
    position: relative;
}
img {
    max-width: 55vw;
}
article {
    position: fixed;
    bottom: 0;
    line-height: 1.5em;
    padding: 0.5em;
    background-color: black;
    opacity: 0.7;
}
.info {
    font-size: 0.8em;
}
.play {
    cursor: pointer;
}
</style>
        `;
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
        this.root.innerHTML = `
        ${this.css()}
<h1 style="text-align:center;padding:0 1em;">
    ${this.movie.title} (${this.movie.date.split("-").shift()})
    &nbsp;&nbsp;
    <span class="play">▶</div>
</h1>
<div style="text-align:center;">
    <img src="${this.movie.poster_url}">
</div>
<article>
    ${this.movie.summary}
    <br>
    <br>
    <span class="info">${this.movie.casts.join(", ")}</span>
    <br>
    <span class="info">${this.movie.genres.join(", ")}</span>
    <br>
    <span class="info">${this.movie.file_path}</span>
</article>
        `;

        this.root.querySelector("h1").addEventListener("click", () => {
            this.close();
        });
        this.root.querySelector("img").addEventListener("click", () => {
            this.close();
        });
        this.root.querySelector(".play")?.addEventListener("click", (e) => {
            eventBus.fire("play-movie", JSON.parse(JSON.stringify(this.movie)));
        });
    }
}

window.customElements.define('app-summary', SummaryComponent);