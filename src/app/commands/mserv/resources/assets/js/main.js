import { SearchComponent } from "./components/search.js";
import { MediasComponent } from "./components/medias.js";
import { SummaryComponent } from "./components/summary.js";
import { PLayerComponent } from "./components/player.js";
import { app } from "./services/app.js";
import { eventBus } from "./services/EventBus.js";

const searchTerm = app.getSearchTerm();
eventBus.fire("navigate-search", {
    initiator: "main",
    term: searchTerm,
});
