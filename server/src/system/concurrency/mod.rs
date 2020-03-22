mod worker;

use worker::Worker;

pub struct ThreadPool {
    workers: Vec<Worker>,
}

impl ThreadPool {
    pub fn create(size: usize) -> ThreadPool {

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id));
        }

        ThreadPool {
            workers
        }
    }
}
