import { SearchComponent } from "./components/search.js";
import { MediasComponent } from "./components/medias.js";
import { SummaryComponent } from "./components/summary.js";
import { PLayerComponent } from "./components/player.js";

const VIDEO_EXTENSIONS = ["VIDEO_EXTENSIONS"];

String.prototype.escape_quote = function () {
    return this.replace(/"/g, "'");
};

String.prototype.escape_path_attribute = function () {
    if (/^http/.test(this)) {
        return this.escape_quote();
    }
    return encodeURIComponent(this).escape_quote();
};

String.prototype.escape_path = function () {
    if (/^http/.test(this)) {
        return this.escape_quote();
    }
    return encodeURIComponent(this);
};

String.prototype.extension = function () {
    return this.split('.').pop();
};

String.prototype.isVideoFile = function () {
    if (/\./.test(this)) {
        return VIDEO_EXTENSIONS.includes(this.extension().toLowerCase());
    }
    return VIDEO_EXTENSIONS.includes(this.toLowerCase());
};

String.prototype.toClipBoard = function () {
    try {
        navigator.clipboard.writeText(this);
    } catch (_) {
        const selBox = window.document.createElement('textarea');
        selBox.name = 'sel-clip-board';
        selBox.style.position = 'fixed';
        selBox.style.left = '0';
        selBox.style.top = '0';
        selBox.style.opacity = '0';
        selBox.value = this;
        document.body.appendChild(selBox);
        selBox.focus();
        selBox.select();
        document.execCommand('copy');
        document.body.removeChild(selBox);
    }
};


Number.prototype.secondsToHMS = function() {
    if (typeof this === "undefined") return "";
    return new Date(this * 1000).toISOString().slice(11, 19);
}

