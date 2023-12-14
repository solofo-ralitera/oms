import {eventBus} from '../helpers/EventBus.js';

export class MovieComponent extends HTMLElement {
    constructor() {
        super();
        this.root = this.attachShadow({mode: "closed"});
        
        this.observer = new IntersectionObserver((entries, observer) => {
            entries.forEach((entry) => {
                if (entry.isIntersecting && entry.intersectionRatio >= 0.1) {
                    if (this.root.querySelector(".card .card-body-bg")) {
                        this.root.querySelector(".card .card-body-bg").style.backgroundImage = `url("${this._movie.thumb_url}")`;
                    }
                    if (this.root.querySelector("#thumb")) {
                        this.root.querySelector("#thumb").src = this.root.querySelector("#thumb")?.getAttribute('data-src');
                    }
                }
            });
        }, {
            root: window.document,
            rootMargin: "0px",
            threshold: 0.1,
        });

        this.render();
    }

    set movie(movie) {
        this._movie = movie;
        this.render();
    }
    
    css() {
        return `
<style type="text/css">
.card {
    color: white;
    background-color: black;
    box-shadow: rgba(9, 30, 66, 0.25) 0px 4px 8px -2px, rgba(9, 30, 66, 0.08) 0px 0px 0px 1px;    height: 456px;
    display: grid;
    grid-template-rows: 2em 1fr;
    position: relative;
}
.card header {
    display: flex;
    align-items:center;
    justify-content: space-between;
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    text-shadow: 0 0 1px #000;
    background-color: black;
    opacity: 0.75;
    z-index: 2;
    padding: 0.5em;
}
.card .card-body {
    position: relative;
    display: flex;
    justify-content: center;
    align-items: center; 
    overflow: hidden;
    width: 300px;
    height: 456px;
}
.card .card-body-bg {
    position: absolute;
    top: 0;
    bottom: 0;
    left: 0;
    right: 0;
    background-repeat: no-repeat;
    background-size: cover;
    background-position: center;
    filter: blur(8px) grayscale(100%);
    -webkit-filter: blur(8px) grayscale(100%);
    z-index: 0;
}
.card .card-body .card-body-content {
    z-index: 1;
}
.card .card-body .card-body-summary {
    font-size: 0.9em;
    padding: 1em;
    line-height: 1.5em;
    color: white;
    background-color: rgb(49, 49, 49);
    mix-blend-mode: difference;
}
.info {
    font-size: 0.8em;
}
.play {
    text-align: center;
    cursor: pointer;
}
</style>
        `;
    }

    renderImage(lazy = true) {
        if (this._movie.thumb_url) {
            if (lazy === true) {
                return `<img src="data:image/gif;base64,R0lGODlhAQABAIAAAP///wAAACH5BAEAAAAALAAAAAABAAEAAAICRAEAOw==" data-src="${this._movie.thumb_url}" id="thumb">`;
            }
            return `<img src="${this._movie.thumb_url}" data-src="${this._movie.thumb_url}" id="thumb">`;
        }
        return this.renderSummary();
    }

    playEvent() {
        window.setTimeout(() => {
            this.root.querySelector(".play")?.addEventListener("click", (e) => {
                e.preventDefault();
                e.stopPropagation();
                eventBus.fire("play-movie", JSON.parse(JSON.stringify(this._movie)));
            });
        }, 250);
    }

    renderSummary() {
        this.playEvent();

        return `<article class="card-body-summary">
            <summary>${this._movie.summary}</summary>
            <div class="play">▶</div>
            <br>
            <hr>
            <span class="info">${this._movie.casts.join(", ")}</span>
            <br>
            <span class="info">${this._movie.genres.join(", ")}</span>
        </article>`;
    }

    async render() {
        if (!this._movie) {
            this.root.innerHTML = '';
            return;
        }

        this.root.innerHTML = `
            ${this.css()}
            <article class="card" id="card">
                <header id="card-title">
                    <span>
                        <span class="play">▶</span>
                        &nbsp;
                        ${this._movie.title}&nbsp;
                    </span>
                    <span class="info">(${this._movie.date?.split("-")?.shift()})</span>
                </header>
                <div class="card-body">
                    <div class="card-body-bg"></div>
                    <div class="card-body-content">
                        ${this.renderImage(true)}
                    </div>
                </div>
            </article>
        `;

        this.root.querySelector("#card-title").addEventListener("click", (e) => {
            eventBus.fire("current-movie", JSON.parse(JSON.stringify(this._movie)));
        });
        this.root.querySelector(".card-body-bg").addEventListener("click", (e) => {
            eventBus.fire("current-movie", JSON.parse(JSON.stringify(this._movie)));
        });

        this.root.querySelector("#thumb").addEventListener("error", (e) => {
            this.root.querySelector(".card-body-content").innerHTML = this.renderSummary();
        });

        this.root.querySelector(".card-body-content").addEventListener("click", (e) => {
            const content = this.root.querySelector(".card-body-content").innerHTML;
            this.root.querySelector(".card-body-content").innerHTML = content.includes("<img") ? this.renderSummary() : this.renderImage(false);
        });
        this.playEvent();

        this.observer.observe(this.root.querySelector("#card"));

    }
}

window.customElements.define('app-movie', MovieComponent);