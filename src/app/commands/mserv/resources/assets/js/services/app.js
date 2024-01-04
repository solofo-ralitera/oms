import {eventBus} from './EventBus.js';

export const app = new class {
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

    openItem(item) {
        if (!item) return;
        if (item?.file_type === "image") {
            window.open(`/poster${item.file_path}`);
        } else if (item?.file_type === "movie") {
            eventBus.fire("play-movie", item);
        } else if (item?.file_type === "audio") {
            eventBus.fire("play-audio", item);
        } else {
            window.open(`/open${item.file_path}`);
        }        
    }
}