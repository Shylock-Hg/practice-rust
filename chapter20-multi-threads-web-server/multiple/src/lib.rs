use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

extern crate log;
use log::{trace, debug, info, warn, error};

pub struct ThreadPool {
        workers:Vec<Worker>,
        sender: mpsc::Sender<Job>,  // Job --> Worker
}

#[derive(Debug)]
pub enum ThreadPoolError {
        INVALID_SIZE,
}

impl ThreadPool {
        pub fn new(count: usize) -> Result<ThreadPool, ThreadPoolError> {
                if count == 0 {
                        return Err(ThreadPoolError::INVALID_SIZE);
                };
                let (sender, receiver) = mpsc::channel();
                let receiver = Arc::new(Mutex::new(receiver));
                let mut pool = ThreadPool {
                        workers: Vec::with_capacity(count),
                        sender: sender,
                };
                for id in 0..count {
                        pool.workers.push(
                                Worker::new(id, Arc::clone(&receiver)),
                        );
                };
                Ok(pool)
        }

        pub fn apply<F>(&self, f: F)
                where F: FnOnce() + Send + 'static {
                let job = Box::new(f);
                self.sender.send(job).unwrap();
        }
}

pub struct Worker {
        id: usize,
        handle: thread::JoinHandle<()>,
}

impl Worker {
        fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
                Worker {
                        id: id,
                        handle: thread::spawn(move || {
                                loop {
                                        let job = receiver.lock().expect("Lock failed!").recv().expect("Recv failed!");  // Box<F>
                                        // (*job)();
                                        trace!("Thread {} got a job.", id);
                                        job.call();
                                }
                        }),
                }
        }
}

// type Job = Box<dyn FnOnce() + Send + 'static>;
type Job = Box<dyn FnBox + Send + 'static>;

trait FnBox {
        fn call(self: Box<Self>);
}

impl<F: FnOnce()> FnBox for F {
        fn call(self: Box<Self>) {
                (*self)();
        }
        
}
