import { SearchComponent } from "./components/search.js";
import { MoviesComponent } from "./components/movies.js";
import { SummaryComponent } from "./components/summary.js";
import { PLayerComponent } from "./components/player.js";

String.prototype.escape_quote = function () {
    return this.replace(/"/g, "'");
};