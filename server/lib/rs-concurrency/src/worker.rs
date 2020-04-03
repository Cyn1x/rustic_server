use std::thread;

use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;

pub type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    pub fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || {
            loop {
                let job = receiver.lock().unwrap().recv().unwrap();

                job();
            }
        });

        Worker {
            id,
            thread,
        }
    }
}
