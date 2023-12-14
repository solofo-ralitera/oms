import {eventBus} from '../helpers/EventBus.js';

export class PLayerComponent extends HTMLElement {
    keyuptimer = 0;
    movie = null;

    constructor() {
        super();
        this.root = this.attachShadow({mode: "closed"});
        this.render();

        eventBus.register("play-movie", e => {
            this.movie = e.detail;
            this.render();
        });
    }
    css() {
        return `
        <style type="text/css">
:root {
    position: relative;
}        
.close {
    position: absolute;
    top: 0;
    right: 1em;
    z-index: 2;
    color: white;
    border-radius: 50%;
    height: 50px;
    line-height: 50px;
    text-align: center;
    cursor: pointer;
    opacity: 0;   
    transition: opacity 0.3s;
}
.close:hover {
    opacity: 1;
}
.info {
    font-size: 0.7em;
}
        </style>
        `;
    }

    render() {
        if (this.movie === null) {
            this.root.innerHTML = '';
            return;
        };
        this.root.innerHTML = `${this.css()}        
<div class="close"><span class="info">${this.movie.title}</span>&nbsp;X</div>
<video  controls 
        id="video-player"
        class="video-js" 
        data-setup='{"controls": true, "autoplay": "play", "preload": "auto"}'
        poster="${this.movie.thumb_url}">
    <source src="./movie${this.movie.file_path}" type="video/mp4" />
    <p>
        Your browser doesn't support this video. Here is the path of the file:
        ${this.movie.file_path}
    </p>
</video>`;
        this.root.querySelector("#search")?.addEventListener("input", e => {
            window.clearTimeout(this.keyuptimer);
            const value = e.target.value;
            this.keyuptimer = window.setTimeout(() => {
                eventBus.fire("movie-search", value);
            }, 350);
        });
        this.root.querySelector(".close")?.addEventListener("click", e => {
            eventBus.fire("play-movie", null);
        });

        // const player = videojs(this.root.querySelector("#video-player"));
        // console.log(player);
    }
}

window.customElements.define('app-player', PLayerComponent);