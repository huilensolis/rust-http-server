use std::{
    sync::{mpsc, Arc, Mutex},
    thread::{self},
};

type Job = Box<dyn FnOnce() + Send + 'static>;

struct Worker {
    id: usize,
    join_permission: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Result<Worker, String> {
        let thread_factory = thread::Builder::new();
        let spawned_thread = thread_factory.spawn(move || loop {
            let thread_message = receiver.lock().expect("could not lock the receiver").recv();

            match thread_message {
                Ok(job) => {
                    println!("worker with id {id} got a job; executing");

                    job()
                }
                Err(_) => {
                    println!("worker with id {id} disconnected; shutting down");
                    break;
                }
            }
        });

        match spawned_thread {
            Ok(thread_join_permission) => Ok(Worker {
                id,
                join_permission: Some(thread_join_permission),
            }),
            Err(error) => Err(format!(
                "thread with id {id} could not be spawned. error message: {error}"
            )),
        }
    }
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

impl ThreadPool {
    /// Create a new thread pool
    ///
    ///the max_threads argument is the maximum number of threads allowed to run simultaneously.
    ///
    /// # panics
    ///
    /// panics if the value of max_threads is 0
    pub fn new(max_threads: usize) -> ThreadPool {
        assert!(max_threads > 0);

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(max_threads);

        for index in 0..max_threads {
            let new_worker = Worker::new(index, Arc::clone(&receiver));

            match new_worker {
                Ok(worker) => {
                    workers.push(worker);
                }
                Err(error) => {
                    println!("error spawning thread. error: {}", error)
                }
            }
        }

        ThreadPool {
            workers,
            sender: Some(sender),
        }
    }

    pub fn execute<C>(&self, clousure: C)
    where
        C: FnOnce() + Send + 'static,
    {
        let job = Box::new(clousure);

        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());

        for worker in &mut self.workers {
            println!("shutting down worker {}", worker.id);

            if let Some(thread) = worker.join_permission.take() {
                thread.join().unwrap()
            };
        }
    }
}
