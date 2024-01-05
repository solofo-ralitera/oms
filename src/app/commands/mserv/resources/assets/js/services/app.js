import {eventBus} from './EventBus.js';

// init start volume
let PLAYER_VOLUME = parseFloat(window.localStorage.getItem("PLAYER_VOLUME") ?? "1");
PLAYER_VOLUME = isNaN(PLAYER_VOLUME) ? 1 : PLAYER_VOLUME;

export const app = new class {
    playerVolume(mediaElement) {
        if (typeof mediaElement === "undefined") {
            return Math.max(0, Math.min(1, PLAYER_VOLUME));
        } else {
            PLAYER_VOLUME = mediaElement.muted ? 0 : Math.max(0, Math.min(1, mediaElement.volume));
            window.localStorage.setItem("PLAYER_VOLUME", PLAYER_VOLUME);            
        }
    }

    async scanDir() {
        return fetch("./scan-dir");
    }

    async transcodeDir(extension = "") {
        return fetch(`./transcode-dir/${extension}`);
    }

    async getAllFiles() {
        return fetch("./all-files-path")
            .then(r => r.json())
            .then(r => r)
            .catch(() => []);
    }

    async summary() {
        return fetch("./summary")
            .then(r => r.json())
            .then(r => r)
            .catch(() => {});
    }

    openMedia(media) {
        if (!media) return;

        if (media.file_type === "image") {
            window.open(`/poster${media.file_path}`);
        } else if (["audio", "video"].includes(media.file_type)) {
            eventBus.fire("play-media", media);
        } else {
            window.open(`/open${media.file_path}`);
        }        
    }
}