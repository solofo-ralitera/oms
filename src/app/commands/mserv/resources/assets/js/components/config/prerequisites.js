import {eventBus} from '../../services/EventBus.js';
import {app} from '../../services/app.js';

const APP_VERSION = "APP_VERSION";

export class Prerequistes extends HTMLElement {
    css = `<style type="text/css">
ul {
    list-style-type: none;
}
ul li.ok:before {
    color: forestgreen;
    content: '✓';
}
ul li.ko:before {
    color: darkred;
    content: 'x';
}
ul li.maybe:before {
    color: darkorange;
    content: '?';
}
.ok {
    color: forestgreen;
}
.ko {
    color: darkred;
}
.maybe {
    color: darkorange;
}
.version {
    font-size: 0.8em;
    color: slategrey;
}
footer {
    font-size: 0.8em;
    color: lightslategray;
}
    </style>`;
    
    constructor() {
        super();
        this.root = this.attachShadow({mode: "closed"});
        this.render();
    }

    async checkElastic(url) {
        if (!url) {
            return Promise.reject(new Error("elastic-url is not defined"));
        }
        return fetch(url.substring(0, url.lastIndexOf('/')))
            .then(r => r.json())
            .then(elastic => elastic?.version?.number ?? "");
    }

    render() {
        this.root.innerHTML = `${this.css}
<article>
    <h3>Prerequistes</h3>
    <div id="prerequistes-container"></div>
    <footer>oms v${APP_VERSION}</footer>
</article>`;
        app.prerequistes().then(prerequistes => {
            let str = '<ul>';
            Object.keys(prerequistes).sort().map(cmd => {
                if (cmd === 'elastic') {
                    str += `<li id="elastic">
                    <span id="elastic-cmd">${cmd}</span>&nbsp;&nbsp;<span id="elastic-url" class="version">${prerequistes[cmd]}</span>
                    </li>`;
                    this.checkElastic(prerequistes[cmd]).then(version => {
                        if (version) {
                            this.root.querySelector("#elastic-url").innerHTML = version;
                            this.root.querySelector("#elastic-url").closest("li").classList.add("ok");
                        } else {
                            this.root.querySelector("#elastic-url").closest("li").classList.add("maybe");
                            this.root.querySelector("#elastic-url").innerHTML = `
                                can't retrieve version on ${prerequistes[cmd]}
                            `;                            
                        }
                    }).catch(err => {
                        this.root.querySelector("#elastic-url").closest("li").classList.add("ko");
                        this.root.querySelector("#elastic-url").innerHTML = `
                            Elasticsearch is not available, with error: "${err.message}".
                            Check if elastic configuration allows cors
                        `;
                    })
                } else {
                    str += `<li class="${prerequistes[cmd] ? "ok" : "ko"}">
                    <span>${cmd}</span>&nbsp;&nbsp;<span class="version">${prerequistes[cmd]}</span>
                    </li>`;
                }
            });
            str += '</ul>';
            this.root.querySelector("#prerequistes-container").innerHTML = str;
        });
    }
}

window.customElements.define('app-config-prerequistes', Prerequistes);
