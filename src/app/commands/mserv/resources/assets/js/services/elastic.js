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
 
    getItem(hash) {
        return fetch(ELASTIC_URL + `/_doc/${hash}`)
            .then(async response => {
                if (response.status >= 400) {
                    throw new Error(await response.text());
                }
            });
    }

    deleteItem(hash) {
        return fetch(ELASTIC_URL + `/_doc/${hash}`, {
            method: "DELETE",
        });
    }

    search(term = "*", from = 0, size = 100) {
        term = term.trim();

        const query = [];
        const sort = [];

        // sort by key asc or desc (<field, >field)
        if (/[><][a-z_0-9]{1,}/i.test(term)) {
            const regex = /([><])([a-z_0-9]{1,})/ig;
            [...term.matchAll(regex)].forEach(m => {
                const order = m[1];
                let field = m[2];
                if (field === "size") field = "file_size";
                if (field === "add" || field === "added" || field === "date") field = "modification_time";                

                sort.push({
                    [field]: order === '>' ? 'desc' : 'asc',
                });
            });
            term = term.replace(/[><][a-z_0-9]{1,}/gi, "").trim();
        }

        // Filter fields (field="term")
        if (/[a-z_0-9]{1,}="[^"]{1,}"/i.test(term)) {
            const regex = /([a-z_0-9]{1,})="([^"]{1,})"/ig;
            [...term.matchAll(regex)].forEach(m => {
                const value = m[2];
                let field = m[1].toLowerCase();
                if (field === "type") field = "file_type";

                if (["ext", "extension"].includes(field)) query.push({
                    "query_string": {
                        "query": `*.${value}`,
                        "fields": ["file_path", "full_path"],
                        "boost": 20,
                    }
                });
                else query.push({
                    "multi_match": {
                        "query": value,
                        "fields": [field],
                        "type" : "phrase",
                        "boost": 20,
                    }
                });
            });
            term = term.replace(/[a-z_0-9]{1,}="[^"]{1,}"/ig, "").trim();
            sort.push("_score");
            sort.push({ "rating": "desc" });
        }

        // Search phrase ("...")
        if (/".{1,}"/i.test(term)) {
            const regex = /"(.{1,})"/ig;
            [...term.matchAll(regex)].forEach(m => {
                const value = m[1];
                query.push({
                    "multi_match": {
                        "query": value,
                        "fields": ["*"],
                        "type" : "phrase",
                        "boost": 30,
                    }
                });
            });

            term = term.replace(/".{1,}"/ig, "").trim();
            sort.push("_score");
            sort.push({ "rating": "desc" });
        }

        if (term) {
            query.push({
                "query_string": {
                    "query": term || "*",
                    "boost": 10,
                }
            });
            sort.push("_score");
        }

        // Random sort by default
        sort.push({
            "_script" : { 
                "script" : "Math.random()",
                "type" : "number",
                "order" : "asc"
            }
        });

        return fetch(ELASTIC_URL + "/_search", {
            method: "POST",
            headers: {
                "Accept": "application/json",
                "Content-Type": "application/json; charset=utf-8",
                "Accept-Encoding": "gzip, deflate, br",
            },
            body: JSON.stringify({
                "query": {
                    "bool": {
                        "should": query
                    }
                },
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
