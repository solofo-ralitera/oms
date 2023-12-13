import {eventBus} from '../helpers/EventBus.js';

export class MovieComponent extends HTMLElement {
    constructor() {
        super();
        this.root = this.attachShadow({mode: "closed"});
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
    box-shadow: rgba(0, 0, 0, 0.2) 0px 12px 28px 0px, rgba(0, 0, 0, 0.1) 0px 2px 4px 0px, rgba(255, 255, 255, 0.05) 0px 0px 0px 1px inset;
    margin: 5px 0;
    width: 300px;
    height: 456px;
    display: grid;
    grid-template-rows: 2em 1fr;
    position: relative;
}
.card header {
    display: flex;
    align-items:center;
    justify-content:center;
}
.card .card-body {
    position: relative;
    display: flex;
    justify-content: center;
    align-items: center; 
    overflow: hidden;
}
.card .card-body-bg {
    position: absolute;
    top: 0;
    bottom: 0;
    left: 0;
    right: 0;
    background-image: url("${this._movie.thumb_url}");
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
</style>
        `;
    }

    renderImage() {
        if (this._movie.thumb_url) {
            return `<img src="${this._movie.thumb_url}" id="thumb">`;
        }
        return this.renderSummary();
    }

    renderSummary() {
        return `<article class="card-body-summary">
            <summary>${this._movie.summary}</summary>
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
            <article class="card">
                <header id="card-title">
                    ${this._movie.title}&nbsp;<span class="info">(${this._movie.date?.split("-")?.shift()})</span>
                </header>
                <div class="card-body">
                    <div class="card-body-bg"></div>
                    <div class="card-body-content">
                        ${this.renderImage()}
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
            this.root.querySelector(".card-body-content").innerHTML = content.includes("<img") ? this.renderSummary() : this.renderImage();
        });
    }
}

window.customElements.define('app-movie', MovieComponent);