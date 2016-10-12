
pub struct Finished {
    num_threads : u8,
    num_finished_threads : u8
}

impl Finished {

    pub fn new(num_threads : u8) -> Finished {
        Finished {
            num_threads: num_threads,
            num_finished_threads: 0
        }
    }

    pub fn is_finished(&self) -> bool {
        self.num_threads == self.num_finished_threads
    }

    pub fn inc(&mut self) {
        self.num_finished_threads += 1;
    }
}