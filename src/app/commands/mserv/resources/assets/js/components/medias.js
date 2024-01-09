import { MediaComponent } from "./media.js";
import { ConfigComponent } from "./config.js";
import {eventBus} from '../services/EventBus.js';
import {elasticMedia} from '../services/elastic.js';
import { Genres } from "./config/genres.js";
import { Casts } from "./config/casts.js";

export class MediasComponent extends HTMLElement {
    static observedAttributes = ["search"];
    isRendering = false;
    currentFrom = 0;
    pageSize = 100;

    constructor() {
        super();
        this.root = this.attachShadow({mode: "closed"});
        this.searchTerm = this.getAttribute('search') ?? "";

        eventBus.register("media-search", ({detail}) => {
            this.setAttribute("search", detail);
        });

        eventBus.register("display-config", () => {
            this.renderConfig();
        });
        eventBus.register("display-genre", () => {
            this.renderGenre();
        });
        eventBus.register("display-cast", () => {
            this.renderCast();
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
        const medias = await elasticMedia.search(this.searchTerm, this.currentFrom, this.pageSize);
        medias.forEach(media => {
            const appMedia = new MediaComponent();
            appMedia.media = media;
            this.root.append(appMedia);
        });
        if (medias.length) {
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

    renderGenre() {
        this.root.innerHTML = this.css();
        this.root.append(new Genres());
    }

    renderCast() {
        this.root.innerHTML = this.css();
        this.root.append(new Casts());
    }
}

window.customElements.define('app-medias', MediasComponent);