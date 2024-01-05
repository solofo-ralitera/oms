import {eventBus} from '../../services/EventBus.js';
import {app} from '../../services/app.js';

export class PlayerAudioComponent extends HTMLElement {
    css = `<style type="text/css">
    .audio-container {
        position: relative;
        background-color: black;
        z-index: 3;
        max-width: 100vw;
        max-height: 100vh;
        margin-left: 2em;
    }
    .tool {
        display: grid;
        grid-template-columns: 1fr 35px;
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
    .footer {
        height: 2em;
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
            <div class="audio-container" data-size="mini">
                <div class="tool">
                    <span class="info">${this.mediaItem.title}</span>
                    <button class="close" arial-label="Close audio player">x</button>
                </div>
                <audio controls autoplay
                    <source src="./media${this.mediaItem.file_path.escape_path_attribute()}" type="audio/${this.mediaItem.file_path.extension()}" />
                    <p>
                        Your browser doesn't support this audio. Here is the path of the file:
                        ${this.mediaItem.file_path}
                    </p>
                </audio>
                <footer class="footer">&nbsp;</footer>
            </div>`;

        const audio = this.root.querySelector("audio");
        audio.volume = app.playerVolume();
        audio.addEventListener("volumechange", event => {
            app.playerVolume(event.target);
        });
        
        this.root.querySelector(".close")?.addEventListener("click", e => {
            eventBus.fire("play-media", null);
        });
    }
}

window.customElements.define('app-player-audio', PlayerAudioComponent);