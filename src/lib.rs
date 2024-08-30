use std::{
    thread::{self, sleep},
    time::Duration,
};

struct Worker {
    id: usize,
    join_permission: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize) -> Result<Worker, String> {
        let thread_factory = thread::Builder::new();
        let spawned_thread = thread_factory.spawn(|| sleep(Duration::new(10, 0)));

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

        let mut workers = Vec::with_capacity(max_threads);

        for index in 0..max_threads {
            let new_worker = Worker::new(index);

            match new_worker {
                Ok(worker) => {
                    workers.push(worker);
                }
                Err(error) => {
                    println!("error spawning thread. error: {}", error)
                }
            }
        }

        println!("workers len = {}", workers.len());

        ThreadPool { workers }
    }

    pub fn execute<C>(&self, clousure: C)
    where
        C: FnOnce() + Send + 'static,
    {
        // clousure()
    }
}
