import {elasticMedia} from '../../services/elastic.js';
import {eventBus} from '../../services/EventBus.js';

export class Genres extends HTMLElement {
    css = `<style type="text/css">
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
    
    constructor() {
        super();
        this.root = this.attachShadow({mode: "closed"});
        this.render();
    }

    async render() {
        const genres = await elasticMedia.getGenres();
        this.root.innerHTML = `${this.css}
            <article>
                <h3>Genres</h3>
                <ul>
                ${genres.map(genre => 
                    `<li class="genre" data-genre="${genre?.escape_quote()}">${genre}</li>`
                ).join("")}
                </ul>
            </article>`;
        this.root.querySelectorAll("li.genre").forEach(el => el.addEventListener("click", li => {
            li.preventDefault();
            const genre = li.target.getAttribute('data-genre');
            if (genre) {
                eventBus.fire("navigate-search", {
                    term: `:genre ${genre}`,
                });
            }
        }))
    }
}

window.customElements.define('app-config-genres', Genres);
