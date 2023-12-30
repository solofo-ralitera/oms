import { MovieComponent } from "./movie.js";
import { ConfigComponent } from "./config.js";
import {eventBus} from '../services/EventBus.js';
import {elasticMovie} from '../services/elastic.js';

export class MoviesComponent extends HTMLElement {
    static observedAttributes = ["search"];
    isRendering = false;
    currentFrom = 0;
    pageSize = 100;

    constructor() {
        super();
        this.root = this.attachShadow({mode: "closed"});
        this.searchTerm = this.getAttribute('search') ?? "";

        eventBus.register("movie-search", ({detail}) => {
            this.setAttribute("search", detail);
        });

        eventBus.register("display-config", () => {
            this.renderConfig();
        });
    }

    attributeChangedCallback(name, oldValue, newValue) {
        if (oldValue !== newValue) {
            if (name === "search") {
                this.searchTerm = newValue
                this.render();
            }
        }
    }

    css() {
return `<style type="text/css">
:host {
    display: flex;
    justify-content: center;
    flex-flow: wrap row;
    align-items: flex-start;
    height: 100vh;
    gap: 4px;
}
</style>`;
    }

    async searchAll() {
        const movies = await elasticMovie.search(this.searchTerm, this.currentFrom, this.pageSize);
        movies.forEach(movie => {
            const appMovie = new MovieComponent();
            appMovie.movie = movie;
            this.root.append(appMovie);
        });
        if (movies.length) {
            this.currentFrom += this.pageSize;
            await this.searchAll();
        }
    }

    async render() {
        if(this.isRendering) {
            return;
        }
        try {
            this.currentFrom = 0;
            this.isRendering = true;
            this.root.innerHTML = this.css();
            this.searchAll();
        } finally {
            this.isRendering = false;
        }
    }

    renderConfig() {
        this.root.innerHTML = this.css();
        this.root.append(new ConfigComponent());
    }
}

window.customElements.define('app-movies', MoviesComponent);