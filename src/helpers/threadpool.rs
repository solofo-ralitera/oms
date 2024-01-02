use std::{cmp::max, thread, sync::{mpsc, Arc, Mutex}};

type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

impl ThreadPool {
    pub fn new(size: usize) -> Self {
        let size = max(1, size);
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        
        let mut workers = Vec::with_capacity(size);
        for _ in 0..size {
            workers.push(Worker::new(Arc::clone(&receiver)));
        }

        ThreadPool {
            workers,
            sender: Some(sender),
        }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        if let Some(sender) = self.sender.as_ref() {
            sender.send(job).unwrap();
        }
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());
        for worker in &mut self.workers {
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

struct Worker {
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        // TODO: use std::thread::Builder
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv();
            match message {
                Ok(job) => {
                    job();
                }
                Err(_) => {
                    break;
                }
            }
        });

        Worker { 
            thread: Some(thread) ,
        }
    }
}
