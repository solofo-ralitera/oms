use serde::Serialize;
use crate::helpers::http::post_body;

const ELASTIC_OMS_INDEX: &'static str = "oms";

pub struct Elastic {
    pub index: String,
    pub url: String,
}

impl Elastic {
    pub fn new(url: &String) -> Self {
        let mut _self = Self {
            index: ELASTIC_OMS_INDEX.to_string(),
            url: url.to_string(),
        };
        return _self;
    }

    pub fn insert<T: Serialize>(&self, body: &T) {
        let url = format!("{}/{}/_doc/", self.url, self.index);
        let _ = post_body(&url, &vec![], body);
    }
}

impl Clone for Elastic {
    fn clone(&self) -> Self {
        Elastic {
            index: self.index.clone(),
            url: self.url.clone(),
        }
    }
}
