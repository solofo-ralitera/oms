import { MediaComponent } from "./media.js";
import { ConfigComponent } from "./config.js";
import {eventBus} from '../services/EventBus.js';
import {elasticMedia} from '../services/elastic.js';
import { Genres } from "./config/genres.js";
import { Casts } from "./config/casts.js";

const CSS = `<style type="text/css">
#container {
    display: flex;
    justify-content: center;
    flex-flow: wrap row;
    align-items: flex-start;
    gap: 4px;
}
</style>`;

export class MediasComponent extends HTMLElement {
    static observedAttributes = ["search"];
    isRendering = false;
    currentFrom = 0;
    pageSize = 80;
    numMedias = 0;

    constructor() {
        super();
        this.root = this.attachShadow({mode: "closed"});
        this.searchTerm = this.getAttribute('search') ?? "";

        // Display next on scroll
        this.observer = new IntersectionObserver((entries, observer) => entries.forEach((entry) => {
            if (entry.isIntersecting) this.searchAll();
        }), {
            root: window.document,
            rootMargin: "0px",
            threshold: 0.1,
        });

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
        if (oldValue !== newValue || !newValue) {
            if (name === "search") {
                this.searchTerm = newValue
                this.render();
            }
        }
    }

    async searchAll() {
        const medias = await elasticMedia.search(this.searchTerm, this.currentFrom, this.pageSize);
        medias.forEach(media => {
            this.numMedias++;
            const appMedia = new MediaComponent();
            appMedia.media = media;
            this.root.querySelector("#container")?.append(appMedia);
        });
        if (medias.length) {
            this.currentFrom += this.pageSize;
        }
        // If no result: display setting
        if (["", "*", ":latest", ":last"].includes(this.searchTerm) && this.numMedias === 0) {
            eventBus.fire("navigate-search", {
                initiator: "medias.searchAll.setting",
                term: `:setting`,
            });
        }
    }

    async render() {
        if(this.isRendering) {
            return;
        }
        try {
            this.currentFrom = 0;
            this.numMedias = 0;
            this.isRendering = true;
            this.root.innerHTML = `${CSS}
            <div id="container"></div>
            <div id="scroll">&nbsp;</div>
            `;
            this.observer.observe(this.root.querySelector("#scroll"));
        } finally {
            this.isRendering = false;
        }
    }

    renderConfig() {
        this.root.innerHTML = this.css;
        this.root.append(new ConfigComponent());
    }

    renderGenre() {
        this.root.innerHTML = this.css;
        this.root.append(new Genres());
    }

    renderCast() {
        this.root.innerHTML = this.css;
        this.root.append(new Casts());
    }
}

window.customElements.define('app-medias', MediasComponent);