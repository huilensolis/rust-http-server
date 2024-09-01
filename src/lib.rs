use std::{
    sync::{mpsc, Arc, Mutex},
    thread::{self, sleep},
};

type Job = Box<dyn FnOnce() + Send + 'static>;

struct Worker {
    id: usize,
    join_permission: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Result<Worker, String> {
        let thread_factory = thread::Builder::new();
        let spawned_thread = thread_factory.spawn(move || {
            let job = receiver
                .lock()
                .expect("could not lock the receiver")
                .recv()
                .unwrap();

            println!("worker with id {id} got a job; executing");

            job()
        });

        match spawned_thread {
            Ok(thread_join_permission) => Ok(Worker {
                id,
                join_permission: thread_join_permission,
            }),
            Err(error) => Err(format!(
                "thread with id {id} could not be spawned. error message: {error}"
            )),
        }
    }
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
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

        ThreadPool { workers, sender }
    }

    pub fn execute<C>(&self, clousure: C)
    where
        C: FnOnce() + Send + 'static,
    {
        let job = Box::new(clousure);

        self.sender.send(job).unwrap();
    }
}
