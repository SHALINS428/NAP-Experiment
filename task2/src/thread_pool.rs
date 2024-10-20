use std::thread::{self, JoinHandle};
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{self, Sender, Receiver};

pub struct Worker {
    pub thread: Option<JoinHandle<()>>, // 使用Option<JoinHandle>
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Sender<Box<dyn FnOnce() + Send + 'static>>,
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        let (sender, receiver): (Sender<Box<dyn FnOnce() + Send + 'static>>, Receiver<Box<dyn FnOnce() + Send + 'static>>) =
            mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        
        let mut workers = Vec::with_capacity(size);
        for _ in 0..size {
            let receiver = Arc::clone(&receiver);
            let thread = thread::spawn(move || loop {
                let job = receiver.lock().unwrap().recv();
                match job {
                    Ok(job) => {
                        job();
                    }
                    Err(_) => {
                        break;
                    }
                }
            });

            workers.push(Worker {
                thread: Some(thread), // 修正为Some(thread)
            });
        }

        ThreadPool { workers, sender }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }

    pub fn join(self) {
        drop(self.sender); // 关闭发送者
        for worker in self.workers {
            let thread = worker.thread.expect("Thread not found"); // 修正
            thread.join().unwrap(); // 修正
        }
    }
}
