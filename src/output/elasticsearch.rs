
use output::Output;
use output::Result;

pub struct Elastic {

}

impl Output for Elastic {

    fn consume(&self, result: Result) {
        println!("Url '{}' took {} milliseconds", result.get_response().url, result.get_duration().num_milliseconds());
    }
}

impl Elastic {

    pub fn factory() -> Box<Fn() -> Elastic> {
        Box::new(|| Elastic {})
    }
}


