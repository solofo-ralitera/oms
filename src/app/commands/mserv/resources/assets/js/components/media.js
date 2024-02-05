import {eventBus} from '../services/EventBus.js';
import {app} from '../services/app.js';

const CSS = `<style type="text/css">
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
.pointer {
    cursor: pointer;
}
.pointer:hover {
    text-decoration: underline;
}
.play {
    text-align: center;
    cursor: pointer;
    vertical-align: middle;
    margin: 0 0.5em 0 0;
}
.play.pdf {
    color: #fff;
    background-color: #ef6b6b;
    border: medium none;
    border-spacing: 0;
}
</style>`;

export class MediaComponent extends HTMLElement {
    _media = null;

    constructor() {
        super();
        this.root = this.attachShadow({mode: "closed"});
        
        this.observer = new IntersectionObserver((entries, observer) => {
            entries.forEach((entry) => {
                if (entry.isIntersecting && entry.intersectionRatio >= 0.1) {
                    if (this.root.querySelector(".card .card-body-bg")) {
                        this.root.querySelector(".card .card-body-bg").style.backgroundImage = `linear-gradient(to bottom, rgba(0, 0, 0, 0.73), rgb(192,192,192, 0.1)),url("${this._media.thumb_url.escape_path_attribute()}")`;
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

    set media(media) {
        this._media = media;
        if (this._media) {
            // < 5: cas des N/A
            if (!this._media.thumb_url || this._media.thumb_url.length < 5) {
                this._media.thumb_url = `/thumb${this._media.file_path}`;
            }
            if (!this._media.poster_url || this._media.poster_url.length < 5) {
                this._media.poster_url = `/poster${this._media.file_path}`;
            }
            if (!this._media.casts) {
                this._media.casts = [];
            }
            if (!this._media.genres) {
                this._media.genres = [];
            }
        }
        this.render();
    }
    
    renderImage(lazy = true) {
        if (this._media?.thumb_url) {
            if (lazy === true) {
                return `<img 
                    src="data:image/gif;base64,R0lGODlhAQABAIAAAP///wAAACH5BAEAAAAALAAAAAABAAEAAAICRAEAOw==" 
                    data-src="${this._media.thumb_url.escape_path_attribute()}" 
                    data-filepath="${this._media.file_path.escape_path_attribute()}" 
                    data-attempt="0"
                    id="thumb"
                    loading="lazy"
                    alt="Poster of ${this._media.title.escape_quote()}">`;
            }
            return `<img 
                src="${this._media.thumb_url.escape_path_attribute()}" 
                data-src="${this._media.thumb_url.escape_path_attribute()}" 
                data-filepath="${this._media.file_path.escape_path_attribute()}" 
                data-attempt="0"
                id="thumb" 
                loading="lazy"
                alt="Poster of ${this._media.title.escape_quote()}">`;
        }
        return this.renderSummary();
    }

    playEvent() {
        window.setTimeout(() => {
            const plays = this.root.querySelectorAll(".play");
            plays.forEach(play => play.addEventListener("click", (e) => {
                e.preventDefault();
                e.stopPropagation();
                app.openMedia(this._media);
            }));
        }, 250);
    }

    renderSummary() {
        if (!this._media) {
            return "";
        }
        this.playEvent();

        window.setTimeout(() => {
            this.root.querySelectorAll("li.genre").forEach(li => {
                li.addEventListener("click", e => {
                    eventBus.fire("navigate-search", {
                        initiator: "media.renderSummary.genres",
                        term: `genres="${e.target.innerHTML.trim()}"`,
                    });
                });
            });
            this.root.querySelectorAll("li.cast").forEach(li => {
                li.addEventListener("click", e => {
                    eventBus.fire("navigate-search", {
                        initiator: "media.renderSummary.casts",
                        term: `casts="${e.target.innerHTML.trim()}"`,
                    });
                });
            });
        }, 500);
        let summary = this._media.summary.sanitize().substring(0, 373);
        if (this._media.summary.length > 373) {
            summary += "...";
        }

        return `<article class="card-body-summary">
            <p>${summary}</p>
            <hr>
            <ul class="info"><li class="item cast pointer">${this._media.casts.join("</li><li class=\"item cast\">").sanitize()}</li></ul>
            <ul class="info genres">
                <li class="item"><time>${(this._media.duration?.secondsToHMS() ?? '').sanitize()}</time></li>
                <li class="item genre pointer">${this._media.genres.join("</li><li class=\"item genre\">").sanitize()}</li>
            </ul>
        </article>`;
    }

    renderPlay() {
        if (!this._media) return '';

        if (this._media.file_type === "image") {
            return `<button class="play" tabindex="1" aria-label="Display ${this._media.title.escape_quote()}">🖼</button>`;
        } else if (this._media.file_type === "pdf") {
            return `<button class="play pdf" tabindex="1" aria-label="Display ${this._media.title.escape_quote()}">pdf</button>`;
        } else if (["video", "audio"].includes(this._media.file_type)) {
            return `<button class="play" tabindex="1" aria-label="Play ${this._media.title.escape_quote()}">▶</button>`;
        } else {
            return '';
        }
    }

    fireCurrent() {
        eventBus.fire("current-media", { media: this._media });
    }

    displayContent() {
        if (!this._media.summary || this._media.summary.length < 5) {
            this.fireCurrent();
        } else {
            const content = this.root.querySelector(".card-body-content").innerHTML;
            this.root.querySelector(".card-body-content").innerHTML = content.includes("<img") ? this.renderSummary() : this.renderImage(false);
        }
    }

    async render() {
        if (!this._media) {
            this.root.innerHTML = '';
            return;
        }

        this.root.innerHTML = `${CSS}
            <article class="card" id="card">
                <header id="card-title">
                    <span>
                        ${this.renderPlay()}
                        ${this._media.title.sanitize()}
                    </span>
                    <span class="info" aria-label="Year ${this._media.year?.escape_quote()}">
                        ${this._media.year ? `(<span class="pointer year">${this._media.year.sanitize()}</span>)` : ''}
                    </span>
                </header>
                <div class="card-body">
                    <div class="card-body-bg"></div>
                    <button class="card-body-content" tabindex="2" role="button">
                        ${this.renderImage(true)}
                    </button>
                </div>
            </article>`;

        this.root.querySelector("#card-title")?.addEventListener("click", () => {
            this.fireCurrent();
        });
        this.root.querySelector(".card-body-bg")?.addEventListener("click", () => {
            this.fireCurrent();
        });
        this.root.querySelector(".year")?.addEventListener("click", e => {
            e.preventDefault();
            e.stopPropagation();
            eventBus.fire("navigate-search", {
                initiator: "media.render.year",
                term: `year="${e.target.innerHTML.trim()}"`,
            });
        });

        this.root.querySelector("#thumb")?.addEventListener("error", e => {
            // If image cannot be loaded, generate thumb localy
            let attempt = parseInt(e.target.getAttribute("data-attempt"));
            if (isNaN(attempt)) attempt = 0;
            if (attempt > 1) {
                if (!this._media.summary) {
                    if (this._media.file_path.isPdfFile()) {
                        e.target.src = "/assets/img/pdf.png";
                    } else {
                        this.root.querySelector(".card-body-content").innerHTML = this.renderSummary();
                    }
                } else {
                    this.root.querySelector(".card-body-content").innerHTML = this.renderSummary();
                }
            } else {
                attempt++;
                e.target.setAttribute("data-attempt", attempt);
                const thumb = `/thumb${e.target.getAttribute("data-filepath")}`;
                this._media.thumb_url = thumb;
                e.target.src = thumb;
            }
        });

        this.root.querySelector(".card-body-content")?.addEventListener("click", () => {
            this.displayContent();
        });

        this.playEvent();
        this.observer.observe(this.root.querySelector("#card"));
    }
}

window.customElements.define('app-media', MediaComponent);