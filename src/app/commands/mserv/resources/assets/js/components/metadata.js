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
pre#error-container {
    display:none;
    background-color: darkred;
    color: white;
    padding: 0.5em;
    fonf-size: 0.8em;
    text-align: left;
}
</style>`;

const CancelEvent = new Event("cancel");
const SavedEvent = new Event("saved");

export class MetadataComponent extends HTMLElement {
    _media = null;
    numTrySave = 0;

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
<div class="field-container" id="form-metadata-${this._media.hash}">
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
    <pre id="error-container"></pre>
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
            app.scanDir(this._media.file_path).catch(err => {
                if (confirm("Unable to update the metadata, remove this index ?")) {
                    elasticMedia.deleteItem(this._media.hash);
                }
            });
        });

        this.root.querySelector("#save")?.addEventListener("click", e => {
            this.save();
        });
    }
    
    save() {
        const btn = this.root.querySelector("#save");
        btn.disabled = true;            
        this.displayError();
        const year = parseInt(this.root.querySelector("#year").value.trim());
        app.saveMetadata(this._media.file_path, {
            "title": this.root.querySelector("#title").value.trim(),
            "summary": this.root.querySelector("#summary").value.trim(),
            "year": isNaN(year) ? 0 : year,
            "casts": this.root.querySelector("#casts").value.trim().split(",").map(c => c.trim()),
            "genres": this.root.querySelector("#genres").value.trim().split(",").map(c => c.trim()),
        })
        .then(() => elasticMedia.deleteItem(this._media.hash))
        .then(() => app.scanDir(this._media.file_path))
        .then(() => {
            if (this.formStillExists()) this.dispatchEvent(SavedEvent);
            btn.disabled = false;
        })
        .catch(err => {
            this.numTrySave++;
            if (this.numTrySave < 10) {
                this.save();
            } else {
                this.displayError(err.message);
                this.numTrySave = 0;
                btn.disabled = false;
            }
        })
    }

    formStillExists() {
        return !!this.root.querySelector(`#form-metadata-${this._media.hash}`);
    }

    displayError(text = '') {
        if (!this.formStillExists()) {
            return;
        }
        if (!text) {
            this.root.querySelector("#error-container").style.display = 'none';
            this.root.querySelector("#error-container").innerHTML = '';
        } else {
            this.root.querySelector("#error-container").style.display = 'inherit';
            this.root.querySelector("#error-container").innerHTML = text;
        }
    }
}

window.customElements.define('app-metadata', MetadataComponent);