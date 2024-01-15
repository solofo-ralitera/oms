import {elasticMedia} from '../../services/elastic.js';
import {eventBus} from '../../services/EventBus.js';

export class Casts extends HTMLElement {
    alphaCast = 'abcdefghijklmnopqrstuvwxyz_';
    css = `<style type="text/css">
ul {
    line-height: 1.5em;
    color: yellowgreen;
}
li.cast {
    cursor: pointer;
    display:inline;
}
li.cast:hover {
    text-decoration: underline;
}
li.cast~li.cast::before {
    content: " - ";
}
    </style>`;
    
    constructor() {
        super();
        this.root = this.attachShadow({mode: "closed"});
        this.render();
    }

    async renderCasts() {
        let str = '';
        for (let i=0; i < this.alphaCast.length; i++) {
            str += await this.renderCastLetter(this.alphaCast[i]);
        }
        return str;
    }

    async renderCastLetter(letter) {
        const casts = await elasticMedia.getCasts();
        return `<article>
            <h3>
                ${letter.toUpperCase()}
                <hr>
            </h3>
            <ul>
                ${casts.filter(c => {
                    if (letter === '_') {
                        return !this.alphaCast.includes(c.normalize('NFC').toLowerCase().charAt(0));
                        }
                    return c.toLowerCase().normalize('NFC').charAt(0) === letter
                })
                .map(cast => `<li class="cast" data-cast="${cast?.escape_quote()}">${cast}</li>`)
                .join("")}
            </ul>
        </article>`;
    }

    async render() {
        this.root.innerHTML = `${this.css}
            <article>
                <header>
                    <u>Casts</u>
                </header>
                ${await this.renderCasts()}
            </article>`;
        this.root.querySelectorAll("li.cast").forEach(el => el.addEventListener("click", li => {
            li.preventDefault();
            const cast = li.target.getAttribute('data-cast');
            if (cast) {
                eventBus.fire("navigate-search", {
                    term: `:cast ${cast}`,
                });
            }
        }))
    }
}

window.customElements.define('app-config-casts', Casts);
