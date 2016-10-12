
mod status;
mod worker;
mod result;

extern crate time;

use input::query::Query;
use output::Output;
use hyper::client::Client;
use std::sync::{Arc, Barrier, Mutex, Condvar};
use net::worker::{Producer, Consumer};
use net::result::ResultContainer;

const DEFAULT_NUM_PRODUCERS: u8 = 5;
const DEFAULT_NUM_CONSUMERS: u8 = 3;

pub struct Builder {
    num_consumers: u8,
    num_producers: u8,
}

impl Builder {

    pub fn new() -> Builder {
        Builder {
            num_producers: DEFAULT_NUM_PRODUCERS,
            num_consumers: DEFAULT_NUM_CONSUMERS
        }
    }

    pub fn num_producers(&mut self, num_producers : u8) -> &mut Builder {
        self.num_producers = num_producers;
        self
    }

    pub fn num_consumers(&mut self, num_consumers : u8) -> &mut Builder {
        self.num_consumers = num_consumers;
        self
    }

    pub fn build(&mut self, queries: Vec<Query>) -> NetClient {
        NetClient {
            num_consumers: self.num_consumers,
            num_producers: self.num_producers,
            queries: Arc::new(Mutex::new(queries))
        }
    }
}

pub struct NetClient {
    num_consumers: u8,
    num_producers: u8,
    queries: Arc<Mutex<Vec<Query>>>
}

impl NetClient {

    pub fn perform_queries<T: 'static + Output>(&self, factory: Box<Fn() -> T>) {

        let mut producer_handles = Vec::with_capacity(self.num_producers as usize);
        let mut consumer_handles = Vec::with_capacity(self.num_consumers as usize);

        let producer_barrier = Arc::new(Barrier::new(self.num_producers as usize));
        let result_queue = Arc::new((Mutex::new(ResultContainer::new(self.num_producers)), Condvar::new()));

        let client = Arc::new(Client::new());

        for i in 0..self.num_producers {
            producer_handles.push(Producer::create(
                "Producer-".to_string() + &i.to_string(),
                producer_barrier.clone(),
                client.clone(),
                self.queries.clone(),
                result_queue.clone()).unwrap());
        }

        for i in 0..self.num_consumers {
            consumer_handles.push(Consumer::create(
                "Consumer-".to_string() + &i.to_string(),
                result_queue.clone(),
                factory()).unwrap());
        }

        // Wait for other threads to finish.
        for handle in consumer_handles {
            handle.join().unwrap();
        }
        for handle in producer_handles {
            handle.join().unwrap();
        }

    }
}