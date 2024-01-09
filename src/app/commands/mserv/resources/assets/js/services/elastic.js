const ELASTIC_URL = "ELASTIC_URL";

class ElasticMedia {
    casts = new Set();
    genres = new Set();

    getCasts() {
        return Array.from(this.casts).sort();
    }

    getGenres() {
        return Array.from(this.genres).sort();
    }

    totalCount() {
        return fetch(ELASTIC_URL + "/_count")
            .then(r => r.json())
            .then(r => r?.count ?? 0)
            .catch(() => 0);
    }

    getAll() {
        return this.totalCount()
            .then(count => this.search("*:*", 0, count))
            .catch(() => []);
    }

    search(term = "*", from = 0, size = 100) {
        term = term.trim();

        // Search all field by default
        let query = {
            "query_string": {
                "query": term || "*",
            }
        };
        // Random sort by default
        let sort = {
            "_script" : { 
                "script" : "Math.random()",
                "type" : "number",
                "order" : "asc"
            }
        };

        // sort latest
        if (term.startsWith(':latest') || term.startsWith(':last')) {
            term = term.replace(":latest", "").replace(":last", "").trim();
            query = {
                "query_string": {
                    "query": term || "*",
                }
            };
            sort = [{
                "modification_time": "desc",
            }];
        }
        // sort duration
        else if (term.startsWith(':duration')) {
            term = term.replace(":duration", "").trim();
            query = {
                "query_string": {
                    "query": term || "*",
                }
            };
            sort = [{
                "duration": "asc",
            }, "_score"];
        }
        // Search by year
        else if (/[0-9]{4}/.test(term)) {
            query = {
                "bool": {
                    "should": [
                        {
                            "multi_match": {
                                "query": term,
                                "fields": "year^1.5"
                            }
                        },
                        {
                            "multi_match": {
                                "query": term,
                                "fields": "title^1"
                            }
                        },
                    ]
                }
            };
            sort = [
                { "rating": "desc"},
                {"modification_time": "desc"}
            ];
        }
        // Search by actor
        else if (term.startsWith(':cast')) {
            term = term.replace(":cast", "").trim();
            query = {
                "match_phrase": {
                    "casts": term || "*",
                }
            };
            sort = [
                "_score",
                { "rating": "desc" }
            ];
        }
        // Search by genre
        else if (term.startsWith(':genre')) {
            term = term.replace(":genre", "").trim();
            query = {
                "match_phrase": {
                    "genres": term || "*",
                }
            };
            sort = [
                "_score",
                { "rating": "desc" }
            ];
        }
        // Search in filename
        else if (term.startsWith(':file')) {
            term = term.replace(":file", "").trim();
            query = {
                "multi_match": {
                    "query": `${term}` || "*",
                    "fields": ["file_path"],
                }
            };
            sort = [
                "_score",
                { "rating": "desc" }
            ];
        }
        // Search non empty term
        else if (term && term !== '*') {
            query = {
                "bool": {
                    "should": [
                        {
                            "multi_match": {
                                "query": term,
                                "fields": "title^1.8"
                            }
                        },
                        {
                            "multi_match": {
                                "query": term,
                                "fields": "file_path^1.5"
                            }
                        },
                        {
                            "multi_match": {
                                "query": term,
                                "fields": "genres^1"
                            }
                        },
                        {
                            "multi_match": {
                                "query": term,
                                "fields": "casts^1"
                            }
                        },
                        {
                            "multi_match": {
                                "query": term,
                                "fields": "year^0.9"
                            }
                        },
                        {
                            "multi_match": {
                                "query": term,
                                "fields": "summary^0.7"
                            }
                        },
                    ]
                }
            };
            sort = [
                "_score",
                { "rating": "desc" },
                {"modification_time": "desc"}
            ];
        }

        return fetch(ELASTIC_URL + "/_search", {
            method: "POST",
            headers: {
                "Accept": "application/json",
                "Content-Type": "application/json; charset=utf-8",
                "Accept-Encoding": "gzip, deflate, br",
            },
            body: JSON.stringify({
                "query": query,
                "size": size,
                "from": from,
                "sort": sort,
            })
        })
            .then(r => r.json())
            .then(r => r?.hits?.hits?.map(hit => hit._source) ?? [])
            .then(medias => {
                medias.map(m => m.genres).flat().forEach(g => {
                    if (g) this.genres.add(g)
                });
                medias.map(m => m.casts).flat().forEach(g => {
                    if (g) this.casts.add(g)
                });
                return medias;
            })
            .catch(() => []);
    }

}

export const elasticMedia = new ElasticMedia();
