use std::{
    sync::{
        mpsc::{self, Receiver, Sender},
        Arc, Mutex,
    },
    thread::{self, JoinHandle},
};

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<Sender<Job>>,
}

impl ThreadPool {
    /// Create a new `ThreadPool`.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let workers = (0..size)
            .map(|id| Worker::new(id, Arc::clone(&receiver)))
            .collect::<Vec<_>>();

        ThreadPool {
            workers,
            sender: Some(sender),
        }
    }

    /// Create a new `ThreadPool`.
    ///
    /// The size is the number of threads in the pool,
    /// but it will fail if the size is given zero.
    pub fn build(size: usize) -> Result<ThreadPool, PoolCreationError> {
        match size {
            0 => Result::Err(PoolCreationError),
            _ => Result::Ok(ThreadPool::new(size)),
        }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);
            if let Some(handle) = worker.handle.take() {
                handle.join().unwrap();
            }
        }
    }
}

struct Worker {
    id: usize,
    handle: Option<JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<Receiver<Job>>>) -> Worker {
        Worker {
            id,
            handle: Some(thread::spawn(move || loop {
                match receiver.lock().unwrap().recv() {
                    Ok(job) => {
                        println!("Worker {} got a job;", id);
                        job();
                    }
                    Err(_) => {
                        println!("Worker {} disconnected; shutting down.", id);
                        break;
                    }
                }
            })),
        }
    }
}

type Job = Box<dyn FnOnce() + Send + 'static>;

/// Indicates that some error occured while creating a thread pool.
#[derive(Debug)]
pub struct PoolCreationError;
