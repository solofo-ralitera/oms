use typedb::KV;

pub struct KVStore {
    store: Option<KV<String, String>>,
}

impl KVStore {
    //
    // Erreur static str -> Box::leak
    // https://stackoverflow.com/questions/60580754/rust-borrowed-value-does-not-live-long-enough-while-assigning-to-a-static-vari
    //
    pub fn new(file_path: String) -> Self {
        let store = KV::<String, String>::new(Box::leak(file_path.into_boxed_str()));
        return KVStore {
            store: match store {
                Err(_) => None,
                Ok(store) => Some(store),
            },
        }
    }

    pub fn add(&mut self, key: &String, value: &String) {
        if let Some(store) = self.store.as_mut() {
            let _ = store.insert(key.to_string(), value.to_string());
        }
    }

    pub fn get(&mut self, key: &String) -> Option<String>{
        match self.store.as_mut() {
            None => None,
            Some(store) => match store.get(key) {
                Ok(v) => v,
                Err(_) => None,
            }
        }
    }
}