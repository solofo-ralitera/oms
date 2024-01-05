import {eventBus} from '../services/EventBus.js';
import {PlayerVideoComponent} from './player/video.js';
import {PlayerAudioComponent} from './player/audio.js';

export class PLayerComponent extends HTMLElement {
    media = null;

    constructor() {
        super();
        this.root = this.attachShadow({mode: "closed"});
        this.render();

        eventBus.register("play-media", e => {
            this.media = e.detail;
            this.render();
        });
    }

    css() {
        return `<style type="text/css">
:root {
    position: relative;
}        
</style>`;
    }

    renderPlayer() {
        if(!this.media) {
            return '';
        }
        if (this.media.file_type === "image") {
            
        } else if (this.media.file_type === "video") {
            const player = new PlayerVideoComponent();
            player.media = this.media;
            this.root.append(player);
        } else if (this.media.file_type === "audio") {
            const player = new PlayerAudioComponent();
            player.media = this.media;
            this.root.append(player);
        } else {
            return '';
        }  
    }

    render() {
        if (!this.media) {
            this.root.innerHTML = '';
            return;
        };
        this.root.innerHTML = `${this.css()}`;
        this.renderPlayer();

    }
}

window.customElements.define('app-player', PLayerComponent);