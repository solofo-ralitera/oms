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
        return `<style type="text/css">
:root {
    position: relative;
}        
.tool {
    position: absolute;
    top: 0;
    right: 1em;
    z-index: 4;
    color: white;
    border-radius: 50%;
    height: 50px;
    line-height: 50px;
    text-align: center;
    transition: opacity 0.3s;
    white-space: nowrap;
}
.tool > * {
    all: unset;
    display: inline-block;
    margin: 0 0.3em;
    font-size: 0.9em;
}
.tool .info {
    opacity: 0;
}
.tool:hover:info {
    opacity: 1;
}
video {
    background-color: black;
    z-index: 3;
    max-width: 100vw;
    max-height: 100vh;
}
</style>`;
    }

    render() {
        if (this.movie === null) {
            this.root.innerHTML = '';
            return;
        };
        this.root.innerHTML = `${this.css()}        
<div class="tool">
    <span class="info">${this.movie.title}</span>
    <button class="full">&#9633;</button>
    <button class="close">X</button>
</div>
<video  controls 
        id="video-player"
        data-size="mini"
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

        this.root.querySelector(".full")?.addEventListener("click", e => {
            const player = this.root.querySelector("#video-player");
            if (player) {
                switch (player.getAttribute("data-size")) {
                    case "mini":
                        player.style.width = "100vw";
                        player.style.height = "inherit";
                        player.setAttribute("data-size", "full-width");
                        break;
                    case "full-width":
                            player.style.width = "100vw";
                            player.style.height = "100vh";
                            player.setAttribute("data-size", "full");
                            break;
                    case "full":
                            player.style.width = "inherit";
                            player.style.height = "inherit";
                            player.setAttribute("data-size", "mini");
                            break;
                    }
            }
        });

        // const player = videojs(this.root.querySelector("#video-player"));
        // console.log(player);
    }
}

window.customElements.define('app-player', PLayerComponent);