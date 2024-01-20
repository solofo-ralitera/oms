import {eventBus} from './EventBus.js';

const TRANSCODE_OUTPUT = "TRANSCODE_OUTPUT";

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

    async scanDir(path = "") {
        return fetch("./scan-dir" + path);
    }

    async updateMetadata(media = null) {
        return fetch("./update-metadata" + (media ? media.file_path : ''));
    }

    async saveMetadata(filePath, madatada) {
        if (!filePath) return;
        return fetch(`./update-metadata${filePath}`, {
            method: "POST",
            body: JSON.stringify(madatada),
        });
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

    async serviceLog() {
        return fetch("./service-log")
            .then(r => r.text())
            .catch(() => "");
    }
    
    async prerequistes() {
        return fetch("./prerequistes")
            .then(r => r.json())
            .catch(() => "");
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

    transcodeOutput(extension = "") {
        extension = extension.toLowerCase();
        const listOuput = TRANSCODE_OUTPUT
            .split(",")
            .map(output => output.replace(">", " -> "));
        const extensionOutput = listOuput.filter(output => {
                if (extension === "") return true;
                if (output.startsWith(extension)) return true;
                return false;
            });
        if (!extensionOutput.length) {
            extensionOutput.push(listOuput.find(o => !o.includes("->")));
        }

        return extensionOutput.join("<br>");
    }
}