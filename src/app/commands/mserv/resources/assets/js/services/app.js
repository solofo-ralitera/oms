import {eventBus} from './EventBus.js';

// init start volume, and keep value during this session
let PLAYER_VOLUME = getPLayerVolume();

function getPLayerVolume() {
    let playerVolume = window.localStorage.getItem("PLAYER_VOLUME") ?? "";
    try {
        playerVolume = JSON.parse(playerVolume);
    } catch(err) {
        playerVolume = JSON.parse("[1, false]"); // volume, muted
    }
    return [
        typeof playerVolume[0] === "undefined" ? 1 : playerVolume[0], // volume
        typeof playerVolume[1] === "undefined" ? false : playerVolume[1],   // muted
    ];
}

export const app = new class {
    initPLayerVolume(mediaElement) {
        if (!mediaElement) return;

        const volume = app.playerVolume();
        mediaElement.volume = volume[0];
        mediaElement.muted = volume[1];
        mediaElement?.addEventListener("volumechange", event => {
            this.playerVolume(event.target);
        });
    }

    playerVolume(mediaElement) {
        if (typeof mediaElement === "undefined") {
            return [Math.max(0, Math.min(1, PLAYER_VOLUME[0])), PLAYER_VOLUME[1]];
        } else {
            PLAYER_VOLUME[0] = Math.max(0, Math.min(1, mediaElement.volume));
            PLAYER_VOLUME[1] = mediaElement.muted;
            window.localStorage.setItem("PLAYER_VOLUME", JSON.stringify([PLAYER_VOLUME[0], PLAYER_VOLUME[1]]));            
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
            window.open(`/poster${media.file_path.escape_path()}`);
        } else if (["audio", "video"].includes(media.file_type)) {
            eventBus.fire("play-media", media);
        } else {
            window.open(`/open${media.file_path.escape_path()}`);
        }        
    }
}