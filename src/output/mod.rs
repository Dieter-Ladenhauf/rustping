
extern crate time;

pub mod elasticsearch;

use hyper::client::response::Response;
use self::time::Duration;

pub struct Result {
    name: String,
    response: Response,
    duration: Duration
}

impl Result {

    pub fn new(name: String, response: Response, duration: Duration) -> Result {
        Result {
            name: name,
            response: response,
            duration: duration
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_response(&self) -> &Response {
        &self.response
    }

    pub fn get_duration(&self) -> &Duration {
        &self.duration
    }
}

pub trait Output : Send + Sync {

    fn consume(&self, result: Result);
}