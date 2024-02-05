import {eventBus} from '../services/EventBus.js';
import {PlayerVideoComponent} from './player/video.js';
import {PlayerAudioComponent} from './player/audio.js';

export class PLayerComponent extends HTMLElement {
    media = null;
    player = null;

    constructor() {
        super();
        this.root = this.attachShadow({mode: "closed"});
        this.render();

        eventBus.register("play-media", e => {
            this.media = e.detail;
            if (this.media === null) {
                this.removePlayer();
            }
            this.render();
        });
        eventBus.register("play-media-current-time", e => {
            if (!this.player) {
                this.media = e.detail.media;
                this.render();                
            };
            this.player.currentTime = e.detail.time;
        });
    }

    css() {
        return `<style type="text/css">
:root {
    position: relative;
}        
</style>`;
    }

    removePlayer() {
        try {
            this.player.remove();
            this.player = null;
        } catch (err) {
            this.player = null;
        }
    }

    renderPlayer() {
        if(!this.media) {
            this.removePlayer();
            return '';
        }
        if (this.media.file_type === "image") {
            
        } else if (this.media.file_type === "video") {
            this.player = new PlayerVideoComponent();
            this.player.media = this.media;
            this.root.append(this.player);
        } else if (this.media.file_type === "audio") {
            this.player = new PlayerAudioComponent();
            this.player.media = this.media;
            this.root.append(this.player);
        } else {
            this.removePlayer();
            return '';
        }  
    }

    render() {
        if (!this.media) {
            this.removePlayer();
            this.root.innerHTML = '';
            return;
        };
        this.root.innerHTML = `${this.css()}`;
        this.renderPlayer();

    }
}

window.customElements.define('app-player', PLayerComponent);