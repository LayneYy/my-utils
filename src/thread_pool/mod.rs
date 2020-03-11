use std::thread::JoinHandle;
use std::sync::mpsc::{Receiver, Sender, channel};
use std::thread;
use std::sync::{Arc, Mutex};

pub struct ThreadPool {
    pub size: usize,
    workers: Vec<Worker>,
    sender: Sender<Message>,
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Sending terminate message to all workers.");

        let work_count = self.workers.len();

        for _ in 0..work_count {
            self.sender.send(Message::Terminal).unwrap()
        }

        for w in &mut self.workers {
            println!("shutting down {}", w.id);

            if let Some(h) = w.handle.take() {
                h.join().unwrap()
            }
        }
    }
}

impl ThreadPool {
    pub fn new(size: usize) -> Self {
        let (sender, receiver) = channel::<Message>();
        let mut workers = Vec::with_capacity(size);
        let receiver = Arc::new(Mutex::new(receiver));
        for idx in 0..size {
            let worker = Worker::new(idx, Arc::clone(&receiver));
            workers.push(worker);
        }
        Self {
            size,
            workers,
            sender,
        }
    }
    pub fn execute<F>(&self, f: F)
        where F: FnOnce() + Send + 'static
    {
        self.sender.send(Message::New(Box::new(f))).unwrap()
    }
}

type Job = Box<dyn FnOnce() + Send + 'static>;

struct Worker {
    id: usize,
    handle: Option<JoinHandle<()>>,
}

impl Worker {
    pub fn new(id: usize, receiver: Arc<Mutex<Receiver<Message>>>) -> Self {
        let handle = thread::spawn(move || {
            loop {
                let msg = receiver.lock().unwrap().recv().unwrap();
                match msg {
                    Message::New(job) => job(),
                    Message::Terminal => {
                        println!("Terminal {}", id);
                        break;
                    }
                }
            }
        });

        Worker {
            id,
            handle: Some(handle),
        }
    }
}

enum Message {
    New(Job),
    Terminal,
}