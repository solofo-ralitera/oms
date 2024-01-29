import {elasticMedia} from '../../services/elastic.js';
import {eventBus} from '../../services/EventBus.js';

export class Genres extends HTMLElement {
    alphaGenre = 'abcdefghijklmnopqrstuvwxyz_';

    css = `<style type="text/css">
ul {
    line-height: 1.5em;
    color: yellowgreen;
}
li.genre {
    cursor: pointer;
    display:inline;
}
li.genre:hover {
    text-decoration: underline;
}
li.genre~li.genre::before {
    content: " - ";
}
    </style>`;
    genres = [];
    constructor() {
        super();
        this.root = this.attachShadow({mode: "closed"});
        this.render();
    }

    async renderGenres() {
        let str = "";
        for (let i=0; i < this.alphaGenre.length; i++) {
            str += await this.renderGenreLetter(this.alphaGenre[i]);
        }
        return str;
    }

    async renderGenreLetter(letter) {
        return `<article>
            <h3>
                ${letter.toUpperCase().sanitize()}
                <hr>
            </h3>
            <ul>
                ${this.genres.filter(genre => {
                    if (letter === '_') {
                        return !this.alphaGenre.includes(genre.normalize('NFC').toLowerCase().charAt(0));
                    }
                    return genre.toLowerCase().normalize('NFC').charAt(0) === letter
                })
                .map(genre => `<li class="genre" role="button" data-genre="${genre?.escape_quote()}">${genre.sanitize()}</li>`)
                .join("")}
            </ul>
        </article>`;
    }

    async render() {
        this.genres = await elasticMedia.getGenres();
        this.root.innerHTML = `${this.css}
            <article>
                <h3>Genres</h3>
                ${await this.renderGenres()}
            </article>`;
        this.root.querySelectorAll("li.genre").forEach(el => el.addEventListener("click", li => {
            li.preventDefault();
            const genre = li.target.getAttribute('data-genre');
            if (genre) {
                eventBus.fire("navigate-search", {
                    initiator: "genres.render.genres",
                    term: `genres="${genre}"`,
                });
            }
        }))
    }
}

window.customElements.define('app-config-genres', Genres);
