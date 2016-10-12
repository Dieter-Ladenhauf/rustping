
extern crate url;

use self::url::Url;

#[derive(Clone, Debug)]
pub struct Query {
    name: String,
    url: Url,
    times: u32
}

impl Query {

    pub fn new(name: String, url: Url, times: u32) -> Query {
        Query {
            name: name,
            url: url,
            times: times
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_url(&self) -> &Url {
        &self.url
    }

    pub fn get_times(&self) -> u32 {
        self.times
    }
}