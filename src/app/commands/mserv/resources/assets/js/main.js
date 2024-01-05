import { SearchComponent } from "./components/search.js";
import { MediasComponent } from "./components/medias.js";
import { SummaryComponent } from "./components/summary.js";
import { PLayerComponent } from "./components/player.js";

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

Number.prototype.secondsToHMS = function() {
    if (typeof this === "undefined") return "";
    return new Date(this * 1000).toISOString().slice(11, 19);
}

