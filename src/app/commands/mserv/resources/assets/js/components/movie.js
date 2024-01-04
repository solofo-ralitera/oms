import {eventBus} from '../services/EventBus.js';
import {app} from '../services/app.js';

export class MovieComponent extends HTMLElement {
    css = `<style type="text/css">
ul {
    padding: 0;
    margin: 0;
}
ul li {
    display:inline;
}
.item~.item::before {
    content: ", ";
}
h2,h3,h4 {
    margin: 0;
    font-weight: normal;
}
.card {
    color: white;
    background-color: black;
    box-shadow: rgba(9, 30, 66, 0.25) 0px 4px 8px -2px, rgba(9, 30, 66, 0.08) 0px 0px 0px 1px;    height: 456px;
    display: grid;
    grid-template-rows: 2em 1fr;
    position: relative;
    margin-top: 5px;
}
.card #card-title {
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
    overflow: hidden;
}
.card .card-body {
    position: relative;
    display: flex;
    justify-content: center;
    align-items: center; 
    overflow: hidden;
    width: 295px;
    height: 451px;
    border-bottom-left-radius: 5px;
    border-bottom-right-radius: 5px;
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
    filter: blur(8px) grayscale(85%);
    -webkit-filter: blur(8px) grayscale(85%);
    z-index: 0;
}
.card .card-body .card-body-content {
    all: unset;
    z-index: 1;
    width: 100%;
    -webkit-user-select: text;
    -moz-user-select: text;
    -ms-user-select: text;
    user-select: text;
}
.card .card-body .card-body-content:focus {
    outline: revert;
}
.card .card-body .card-body-summary {
    font-size: 0.9em;
    padding: 1em;
    line-height: 1.5em;
    color: white;
    background-color: rgb(49, 49, 49);
    mix-blend-mode: difference;
}
#thumb {
    max-width: 295px;
}
.info {
    font-size: 0.8em;
}
ul.genres {
    margin-top: 0.5em;
}
li.genre, li.cast {
    cursor: pointer;
}
li.genre:hover, li.cast:hover {
    text-decoration: underline;
}
.play {
    text-align: center;
    cursor: pointer;
    vertical-align: middle;
    margin: 0 0.5em 0 0;
}
</style>`;
    _movie = null;

    constructor() {
        super();
        this.root = this.attachShadow({mode: "closed"});
        
        this.observer = new IntersectionObserver((entries, observer) => {
            entries.forEach((entry) => {
                if (entry.isIntersecting && entry.intersectionRatio >= 0.1) {
                    if (this.root.querySelector(".card .card-body-bg")) {
                        this.root.querySelector(".card .card-body-bg").style.backgroundImage = `linear-gradient(to bottom, rgba(0, 0, 0, 0.73), rgb(192,192,192, 0.1)),url("${this._movie.thumb_url.escape_path_attribute()}")`;
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
        if (this._movie) {
            // < 5: cas des N/A
            if (!this._movie.thumb_url || this._movie.thumb_url.length < 5) {
                this._movie.thumb_url = `/thumb${this._movie.file_path}`;
            }
            if (!this._movie.poster_url || this._movie.poster_url.length < 5) {
                this._movie.poster_url = `/poster${this._movie.file_path}`;
            }
            if (!this._movie.casts) {
                this._movie.casts = [];
            }
            if (!this._movie.genres) {
                this._movie.genres = [];
            }
        }
        this.render();
    }
    
    renderImage(lazy = true) {
        if (this._movie?.thumb_url) {
            if (lazy === true) {
                return `<img 
                    src="data:image/gif;base64,R0lGODlhAQABAIAAAP///wAAACH5BAEAAAAALAAAAAABAAEAAAICRAEAOw==" 
                    data-src="${this._movie.thumb_url.escape_path_attribute()}" 
                    data-filepath="${this._movie.file_path.escape_path_attribute()}" 
                    data-attempt="0"
                    id="thumb"
                    loading="lazy"
                    alt="Poster of ${this._movie.title.escape_quote()}">`;
            }
            return `<img 
                src="${this._movie.thumb_url.escape_path_attribute()}" 
                data-src="${this._movie.thumb_url.escape_path_attribute()}" 
                data-filepath="${this._movie.file_path.escape_path_attribute()}" 
                data-attempt="0"
                id="thumb" 
                loading="lazy"
                alt="Poster of ${this._movie.title.escape_quote()}">`;
        }
        return this.renderSummary();
    }

    playEvent() {
        window.setTimeout(() => {
            const plays = this.root.querySelectorAll(".play");
            plays.forEach(play => play.addEventListener("click", (e) => {
                e.preventDefault();
                e.stopPropagation();
                app.openItem(this._movie);
            }));
        }, 250);
    }

    renderSummary() {
        if (!this._movie) {
            return "";
        }
        this.playEvent();

        window.setTimeout(() => {
            this.root.querySelectorAll("li.genre").forEach(li => {
                li.addEventListener("click", e => {
                    eventBus.fire("navigate-search", {
                        term: `:genre ${e.target.innerHTML.trim()}`,
                    });
                });
            });
            this.root.querySelectorAll("li.cast").forEach(li => {
                li.addEventListener("click", e => {
                    eventBus.fire("navigate-search", {
                        term: `:cast ${e.target.innerHTML.trim()}`,
                    });
                });
            });
        }, 500);

        return `<article class="card-body-summary">
            <p>${this._movie.summary}</p>
            <hr>
            <ul class="info"><li class="item cast">${this._movie.casts.join("</li><li class=\"item cast\">")}</li></ul>
            <ul class="info genres">
                <li class="item"><time>${this._movie.duration?.secondsToHMS() ?? ''}</time></li>
                <li class="item genre">${this._movie.genres.join("</li><li class=\"item genre\">")}</li>
            </ul>
        </article>`;
    }

    renderPlay() {
        if (this._movie?.file_type === "image") {
            return `<button class="play" tabindex="1" aria-label="Display ${this._movie.title.escape_quote()}">🖼</button>`;
        } else if (this._movie?.file_type === "pdf") {
            return `<button class="play" tabindex="1" aria-label="Display ${this._movie.title.escape_quote()}">&#128462;</button>`;
        } else if (this._movie?.file_type === "movie") {
            return `<button class="play" tabindex="1" aria-label="Play ${this._movie.title.escape_quote()}">▶</button>`;
        } else {
            return '';
        }
    }

    fireCurrent() {
        eventBus.fire("current-movie", { movie: this._movie });
    }

    displayContent() {
        if (!this._movie.summary || this._movie.summary.length < 5) {
            this.fireCurrent();
        } else {
            const content = this.root.querySelector(".card-body-content").innerHTML;
            this.root.querySelector(".card-body-content").innerHTML = content.includes("<img") ? this.renderSummary() : this.renderImage(false);
        }
    }

    async render() {
        if (!this._movie) {
            this.root.innerHTML = '';
            return;
        }

        this.root.innerHTML = `${this.css}
            <article class="card" id="card">
                <header id="card-title">
                    <span>
                        ${this.renderPlay()}
                        ${this._movie.title}
                    </span>
                    <span class="info" aria-label="Year ${this._movie.year?.escape_quote()}">${this._movie.year ? `(${this._movie.year})` : ''}</span>
                </header>
                <div class="card-body">
                    <div class="card-body-bg"></div>
                    <button class="card-body-content" tabindex="2" role="button">
                        ${this.renderImage(true)}
                    </button>
                </div>
            </article>`;

        this.root.querySelector("#card-title").addEventListener("click", () => {
            this.fireCurrent();
        });
        this.root.querySelector(".card-body-bg").addEventListener("click", () => {
            this.fireCurrent();
        });

        this.root.querySelector("#thumb").addEventListener("error", e => {
            let attempt = parseInt(e.target.getAttribute("data-attempt"));
            if (isNaN(attempt)) attempt = 0;
            if (attempt > 1) {
                this.root.querySelector(".card-body-content").innerHTML = this.renderSummary();
            } else {
                attempt++;
                e.target.setAttribute("data-attempt", attempt);
                const thumb = `/thumb${e.target.getAttribute("data-filepath")}`;
                this._movie.thumb_url = thumb;
                e.target.src = thumb;
            }
        });

        this.root.querySelector(".card-body-content").addEventListener("click", () => {
            this.displayContent();
        });
        this.playEvent();

        this.observer.observe(this.root.querySelector("#card"));
    }
}

window.customElements.define('app-movie', MovieComponent);