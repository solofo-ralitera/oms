import {eventBus} from '../services/EventBus.js';

export class SearchComponent extends HTMLElement {
    keyuptimer = 0;
    termsHistory = [];

    constructor() {
        super();
        this.root = this.attachShadow({mode: "closed"});
        this.render();
        this.pushHistory("");

        eventBus.register("navigate-search", ({detail}) => {
            this.setValue(detail);
        });
    }
    css() {
        return `<style type="text/css">
search {
    position: relative;
    display: grid;
    grid-template-columns: 37px 1fr;   
    opacity: 0.91;
    border-bottom: 1px solid #5f6368;
    border-top: 0px;
}
button.back {
    width: 100%;
    height: 100%;
    border: none;
    outline: none;
    border-top-right-radius: 0;
    border-bottom-right-radius: 0;
}
input[type=search] {
    width: 100%;
    border: 1px solid #5f6368;
    box-shadow: none;
    padding: 15px 1em;
    background-color: #000;
    outline: none;
}
input[type=search]:focus {
    background-color: #303134;
    border: 1px solid #303134;
    opacity: 1;
}
</style>`;
    }

    setValue(term) {
        this.root.querySelector("#search").value = term;
        this.search();
    }

    pushHistory(term) {
        if (!this.termsHistory.length) {
            this.termsHistory.push(term);
        } else if (this.termsHistory.length && this.termsHistory[this.termsHistory.length - 1] !== term) {
            this.termsHistory.push(term);
        }
        this.dsplayHistoryBtn();
    }

    popHistory() {
        this.termsHistory.pop();
        this.dsplayHistoryBtn();
        if (this.termsHistory.length) {
            return this.termsHistory[this.termsHistory.length - 1];
        }
        return undefined;
    }

    dsplayHistoryBtn() {
        if (this.termsHistory.length <= 1) {
            this.root.querySelector(".back").disabled = true;
        } else {
            this.root.querySelector(".back").disabled = false;
        }
    }

    search() {
        const term = this.root.querySelector("#search").value;
        window.clearTimeout(this.keyuptimer);
        this.keyuptimer = window.setTimeout(() => {
            this.pushHistory(term);
            if (term.startsWith(":setting")) {
                eventBus.fire("display-config", null);
                return;
            }
            eventBus.fire("movie-search", term);
        }, 350);
    }

    back() {
        const term = this.popHistory();
        if (typeof term !== "undefined") {
            this.setValue(term);
        }
    }

    render() {
        this.root.innerHTML = `${this.css()}
            <search>
                <button class="back" aria-label="History back">⬅</button>
                <input placeholder="Search" type="search" id="search" aria-label="Search" autofocus>
            </search>`;
        this.root.querySelector("#search").addEventListener("input", e => {
            this.search();
        });
        this.root.querySelector("#search").addEventListener("keypress", e => {
            if (e.code === "Enter") {
                this.search();
            }
        });
        this.root.querySelector(".back").addEventListener("click", () => {
            this.back();
        });
    }
}

window.customElements.define('app-search', SearchComponent);
