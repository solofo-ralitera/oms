import {eventBus} from '../services/EventBus.js';
import {app} from '../services/app.js';
import {elasticMedia} from '../services/elastic.js';

const CSS = `<style type="text/css">
.field-container {
    margin-bottom: 0.5em;
}
input, textarea {
    padding: 0.5em;
}
label {
    font-size: 0.8em;
    color: darkgray;
}
.f100 {
    width: calc(100% - 1em);
}
button {
    padding: 0.5em 1em;
}
footer {
    text-align: right;
}
button#scan-dir {
    font-size: 1.02em;
}
button#save {
    font-weight: bold;
    font-size: 1.02em;
}
</style>`;

const CancelEvent = new Event("cancel");
const SavedEvent = new Event("saved");

export class MetadataComponent extends HTMLElement {
    _media = null;

    constructor() {
        super();
        this.root = this.attachShadow({mode: "closed"});
        this.render();

    }

    set media(media) {
        this._media = media;
        this.render();
    }

    async render() {
        if (!this._media) {
            this.root.innerHTML = '';
            return;
        };
        try {
            await elasticMedia.getItem(this._media.hash);
        } catch (err) {
            this.root.innerHTML = `${CSS}
            <pre>
                This media file no longer exists, please reload the results
            </pre>`;
            return;
        }

        this.root.innerHTML = `${CSS}
<code>${this._media.file_path}</code>
<div class="field-container">
    <label for="title">Title</label>
    <input type="text" id="title" name="title" class="f100">
</div>
<div class="field-container">
    <label for="summary">Summary</label>
    <textarea id="summary" name="summary" rows="9" class="f100"></textarea>
</div>
<div class="field-container">
    <label for="year">Year</label>
    <input type="number" id="year" name="year">
</div>
<div class="field-container">
    <label for="casts">Casts (separated by comma)</label>
    <input type="text" id="casts" name="casts" class="f100">
</div>
<div class="field-container">
    <label for="genres">Genres (separated by comma)</label>
    <input type="text" id="genres" name="genres" class="f100">
</div>
<footer>
    <button id="cancel">Cancel</button>
    <button id="scan-dir">Update index</button>
    <button id="save">Save</button>
</footer>
`;
        this.root.querySelector("#title").value = this._media.title;
        this.root.querySelector("#summary").value = this._media.summary;
        this.root.querySelector("#year").value = this._media.year;
        this.root.querySelector("#casts").value = this._media.casts?.join(",") ?? "";
        this.root.querySelector("#genres").value = this._media.genres?.join(",") ?? "";

        this.root.querySelector("#cancel")?.addEventListener("click", () => {
            this.dispatchEvent(CancelEvent);
        });

        this.root.querySelector("#scan-dir")?.addEventListener("click", () => {
            app.scanDir(this._media.file_path);
        });

        this.root.querySelector("#save")?.addEventListener("click", e => {
            e.target.disabled = true;
            const year = parseInt(this.root.querySelector("#year").value.trim());
            app.saveMetadata(this._media.file_path, {
                "title": this.root.querySelector("#title").value.trim(),
                "summary": this.root.querySelector("#summary").value.trim(),
                "year": isNaN(year) ? 0 : year,
                "casts": this.root.querySelector("#casts").value.trim().split(","),
                "genres": this.root.querySelector("#genres").value.trim().split(","),
            })
            .then(() => elasticMedia.deleteItem(this._media.hash))
            .then(() => app.scanDir(this._media.file_path))
            .then(() => this.dispatchEvent(SavedEvent))
            .catch(() => {
                e.target.disabled = false;
            })
            .finally(() => {
                e.target.disabled = false;
            });
        });

    }
}

window.customElements.define('app-metadata', MetadataComponent);