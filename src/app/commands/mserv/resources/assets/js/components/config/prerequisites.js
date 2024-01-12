import {eventBus} from '../../services/EventBus.js';
import {app} from '../../services/app.js';

const APP_VERSION = "APP_VERSION";

export class Prerequistes extends HTMLElement {
    css = `<style type="text/css">
ul {
    list-style-type: none;
}
.ok {
    color: forestgreen;
}
.ko {
    color: darkred;
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
        return fetch(url).then(r => r.json());
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
            Object.keys(prerequistes).map(cmd => {
                if (cmd === 'elastic') {
                    str += `<li id="elastic">
                    <span id="elastic-cmd">${cmd}</span>&nbsp;&nbsp;<span id="elastic-url" class="version">${prerequistes[cmd]}</span>
                    </li>`;
                    this.checkElastic(prerequistes[cmd]).then(r => {
                        this.root.querySelector("#elastic-cmd").classList.add("ok");
                    }).catch(err => {
                        this.root.querySelector("#elastic-cmd").classList.add("ko");
                        this.root.querySelector("#elastic-url").innerHTML = `
                            Elastic is not available at ${prerequistes[cmd]}, with error: "${err.message}".
                            <br>check if elastic configuration allows cors
                        `;
                    })
                } else {
                    str += `<li>
                    <span class="${prerequistes[cmd] ? "ok" : "ko"}">${cmd}</span>&nbsp;&nbsp;<span class="version">${prerequistes[cmd]}</span>
                    </li>`;
                }
            });
            str += '</ul>';
            this.root.querySelector("#prerequistes-container").innerHTML = str;
        });
    }
}

window.customElements.define('app-config-prerequistes', Prerequistes);
