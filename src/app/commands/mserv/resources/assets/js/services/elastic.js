const ELASTIC_URL = "ELASTIC_URL";

class ElasticMovie {
    constructor() {

    }

    searchAll(query = "*", from = 0, size = 100) {
        if (query === "") query = "*";
        return fetch(ELASTIC_URL + "/_search", {
            method: "POST",
            headers: {
                "Accept": "application/json",
                "Content-Type": "application/json; charset=utf-8",
                "Accept-Encoding": "gzip, deflate, br",
            },
            body: JSON.stringify({
                "query": {
                    "query_string": {
                        "query": query
                    }
                },
                "size": size,
                "from": from,
                "sort": {
                    "_script" : { 
                        "script" : "Math.random()",
                        "type" : "number",
                        "order" : "asc"
                    }
                }
            })
        })
            .then(r => r.json())
            .then(r => r.hits.hits.map(hit => hit._source))
    }

}

export const elasticMovie = new ElasticMovie();
