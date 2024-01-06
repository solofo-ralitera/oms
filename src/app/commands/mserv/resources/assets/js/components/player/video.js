import {eventBus} from '../../services/EventBus.js';
import {app} from '../../services/app.js';

export class PlayerVideoComponent extends HTMLElement {
    css = `<style type="text/css">
    .video-container {
        position: relative;
        background-color: black;
        z-index: 3;
        max-width: 100vw;
        max-height: 100vh;
    }
    .tool {
        display: grid;
        grid-template-columns: 1fr 35px 35px 1em;
        color: white;
        height: 50px;
        line-height: 50px;
        text-align: center;
        transition: opacity 0.3s;
        white-space: nowrap;
    }
    .tool .info {
        text-align: left;
        font-size: 0.8em;
        color: grey;
        padding-left: 1em;
    }
    .tool > button {
        all: unset;
        width: 35px;
        height: 50px;
        line-height: 50px;
        cursor: pointer;
    }
    .tool > button:hover {
        font-weight: bold;
    }
    video {
        width: 100%;
        height: calc(100% - 50px - 10px);
    }
    .footer {
        height: 10px;
    }
    </style>`;

    mediaItem = null;

    constructor() {
        super();
        this.root = this.attachShadow({mode: "closed"});
    }

    set media(mediaItem) {
        this.mediaItem = mediaItem;
        this.render();
    }

    render() {
        if (!this.mediaItem) {
            this.root.innerHTML = '';
            return;
        };
        this.root.innerHTML = `${this.css}
            <div class="video-container" data-size="mini">
                <div class="tool">
                    <span class="info">${this.mediaItem.title}</span>
                    <button class="full" arial-label="Extend player">&#9634;</button>
                    <button class="close" arial-label="Close video player">x</button>
                    <span></span>
                </div>
                <video controls autoplay
                    poster="${this.mediaItem.thumb_url.escape_path_attribute()}">
                    <source src="./stream${this.mediaItem.file_path.escape_path_attribute()}" type="video/mp4" />
                    <p>
                        Your browser doesn't support this video. Here is the path of the file:
                        ${this.mediaItem.file_path}
                    </p>
                </video>
                <footer class="footer">&nbsp;</footer>
            </div>`;

        const video = this.root.querySelector("video");
        app.initPLayerVolume(video);

        this.root.querySelector(".close")?.addEventListener("click", e => {
            eventBus.fire("play-media", null);
        });

        this.root.querySelector(".full")?.addEventListener("click", e => {
            const player = this.root.querySelector(".video-container");
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
    }
}

window.customElements.define('app-player-video', PlayerVideoComponent);