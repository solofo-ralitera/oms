import {app} from '../../services/app.js';

export class ServiceLog extends HTMLElement {
    css = `<style type="text/css">
#log-container {
    white-space: pre;
    overflow: auto;
    max-width: calc(100vw - 4em);
    font-size: 0.87em;
    color: lightslategrey;
    max-height: 400px;
}
    </style>`;
    
    constructor() {
        super();
        this.root = this.attachShadow({mode: "closed"});
        this.render();
    }

    render() {
        this.root.innerHTML = `${this.css}
        <h3>
            Service log
            <button id="refresh-button">&#10227;</button>
        </h3>
        <div id="log-container"></div>
        `;
        app.serviceLog().then(str => {
            const container = this.root.querySelector("#log-container");
            container.innerHTML = str;
            container.scrollTo({
                top: container.scrollHeight,
                behavior: 'instant'
            });

            this.root.querySelector("#refresh-button").addEventListener("click", () => {
                this.render();
            });

        })
    }
}

window.customElements.define('app-config-service-log', ServiceLog);