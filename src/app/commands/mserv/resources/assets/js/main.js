import { SearchComponent } from "./components/search.js";
import { MoviesComponent } from "./components/movies.js";
import { SummaryComponent } from "./components/summary.js";
import { PLayerComponent } from "./components/player.js";

String.prototype.escape_quote = function () {
    return this.replace(/"/g, "'");
};

Number.prototype.secondsToHMS = function() {
    if (typeof this === "undefined") return "";
    return new Date(this * 1000).toISOString().slice(11, 19);
}
