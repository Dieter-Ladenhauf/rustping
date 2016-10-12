
use net::status::Finished;
use output::Result;

pub struct ResultContainer {
    pub results: Vec<Result>,
    pub finished: Finished
}

impl ResultContainer {

    pub fn new(num_producers: u8) -> ResultContainer {
        ResultContainer {
            results: Vec::<Result>::new(),
            finished: Finished::new(num_producers)
        }
    }
}