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
}