
extern crate time;

use std::io;
use input::query::Query;
use output::Output;
use hyper::client::response::Response;
use hyper::client::Client;
use std::thread::{Builder, JoinHandle};
use std::sync::{Arc, Barrier, Mutex, Condvar};
use hyper::status::StatusCode;
use self::time::PreciseTime;
use self::time::Duration;
use net::status::Finished;
use output::Result;
use net::result::ResultContainer;

pub struct Producer;
pub struct Consumer;

impl Producer {

    pub fn create(name:         String,
                  barrier:      Arc<Barrier>,
                  client:       Arc<Client>,
                  queries:      Arc<Mutex<Vec<Query>>>,
                  results:      Arc<(Mutex<ResultContainer>, Condvar)>)
        -> io::Result<JoinHandle<()>> {

        let thread_name = String::clone(&name);

        Builder::new().name(name).spawn(move || {

            println!("{} created", thread_name);
            barrier.wait();
            println!("{} starts working", thread_name);

            loop {

                let query : Query;
                {
                    let mut data = queries.lock().unwrap();
                    query = match data.pop() {
                        Some(q) => q,
                        None => break
                    }
                } // scope ends: here we release the lock

                for _ in 0..query.get_times() {

                    println!("{} tests '{}' (URL: {})", thread_name, query.get_name(), query.get_url());
                    let start = PreciseTime::now();
                    let result = client.get(query.get_url().as_str()).send().unwrap();
                    let end = PreciseTime::now();

                    // Add the response to the result vector
                    let &(ref result_lock, ref result_cvar) = &*results;
                    let ref mut result_container = result_lock.lock().unwrap();
                    let ref mut result_vec = result_container.results;

                    result_vec.push(Result::new(String::from(query.get_name()), result, start.to(end)));

                    // wake one consumer
                    result_cvar.notify_one();
                }
            }

            println!("{} goes to bed now", thread_name);

            let &(ref result_lock, ref result_cvar) = &*results;
            let ref mut result_container = result_lock.lock().unwrap();
            let ref mut finished = result_container.finished;

            // Increment the counter of finished producers
            finished.inc();

            // The last producer notifies all consumers:
            if finished.is_finished() {
                result_cvar.notify_all();
            }
        })
    }
}

impl Consumer {

    pub fn create<T: 'static + Output>(name: String,
                                       results: Arc<(Mutex<ResultContainer>, Condvar)>,
                                       output: T) -> io::Result<JoinHandle<()>> {

        let thread_name = String::clone(&name);

        Builder::new().name(name).spawn(move || {

            println!("{} created", thread_name);

            loop {

                let &(ref result_lock, ref result_cvar) = &*results;
                let result;

                {
                    let mut result_container = result_lock.lock().unwrap();
                    let is_empty;
                    {
                        let ref mut result_vec = result_container.results;
                        is_empty = result_vec.is_empty();
                    }
                    if is_empty {

                        if result_container.finished.is_finished() {
                            result_cvar.notify_all();
                            break;
                        }

                        // wait for the queue to be non-empty
                        result_container = result_cvar.wait(result_container).unwrap();
                        continue;
                    }

                    let ref mut result_vec = result_container.results;
                    result = match result_vec.pop() {
                        Some(q) => q,
                        None => panic!("This should not happen")
                    };
                } // scope ends: here we release the lock

                println!("{} got a result", thread_name);
                output.consume(result);
            }
            println!("{} goes to bed now", thread_name);
        })
    }
}