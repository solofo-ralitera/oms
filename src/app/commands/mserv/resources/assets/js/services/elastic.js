const ELASTIC_URL = "ELASTIC_URL";

class ElasticMedia {
    getCasts() {
        return this.getAll().then(medias => medias
            .map(m => m.casts)
            .flat()
            .filter(cast => !!cast)
            .sort()).then(casts => Array.from(new Set(casts)));            
    }

    getGenres() {
        return this.getAll().then(medias => medias
            .map(m => m.genres)
            .flat()
            .filter(genre => !!genre)
            .sort()).then(casts => Array.from(new Set(casts)));
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

        const sort = [];

        // sort latest
        if (term.startsWith(':latest') || term.startsWith(':last')) {
            term = term.replace(":latest", "").replace(":last", "").trim();
            query = {
                "query_string": {
                    "query": term || "*",
                }
            };
            sort.push({
                "modification_time": "desc",
            });
        }
        // sort by key asc or desc
        else if (/^[><][a-z_0-9]{1,}/i.test(term)) {
            const regex = /^([><])([a-z_0-9]{1,})(.{0,})/g;
            [...term.matchAll(regex)].forEach(m => {
                const order = m[1];
                let field = m[2];
                if (field === "size") field = "file_size";
                const term = m[3];

                sort.push({
                    [field]: order === '>' ? 'desc' : 'asc',
                });
                query = {
                    "query_string": {
                        "query": term.trim() || "*",
                    }
                };
            });
            sort.push("_score");
        }
        // Search by year
        else if (/^[0-9]{4}$/.test(term)) {
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
            sort.push([
                "_score",
                { "rating": "desc"},
                {"modification_time": "desc"}
            ]);
        }
        // Search by actor
        else if (term.startsWith(':cast')) {
            term = term.replace(":cast", "").trim();
            query = {
                "match_phrase": {
                    "casts": term || "*",
                }
            };
            sort.push([
                "_score",
                { "rating": "desc" }
            ]);
        }
        // Search by genre
        else if (term.startsWith(':genre')) {
            term = term.replace(":genre", "").trim();
            query = {
                "match_phrase": {
                    "genres": term || "*",
                }
            };
            sort.push([
                "_score",
                { "rating": "desc" }
            ]);
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
            sort.push([
                "_score",
                { "rating": "desc" }
            ]);
        }
        // Search non empty term
        else if (term && term !== '*' && term && term !== '*:*') {
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
                                "fields": "summary^0.7"
                            }
                        },
                    ]
                }
            };
            sort.push([
                "_score",
                { "rating": "desc" },
                {"modification_time": "desc"}
            ]);
        }
        // Random sort by default
        if (!sort.length) {
            sort.push({
                "_script" : { 
                    "script" : "Math.random()",
                    "type" : "number",
                    "order" : "asc"
                }
            });
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
                "sort": sort.flat(),
            })
        })
            .then(r => r.json())
            .then(r => r?.hits?.hits?.map(hit => hit._source) ?? [])
            .catch(() => []);
    }

}

export const elasticMedia = new ElasticMedia();
