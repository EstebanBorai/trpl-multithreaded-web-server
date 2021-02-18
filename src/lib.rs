use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::thread::JoinHandle;

pub enum Message {
    NewJob(Job),
    Terminate,
}

pub trait FnBox {
    fn call_box(self: Box<Self>);
}

impl<F: FnOnce()> FnBox for F {
    fn call_box(self: Box<F>) {
        (*self)()
    }
}

type Job = Box<dyn FnBox + Send + 'static>;

pub struct Worker {
    id: usize,
    thread: Option<JoinHandle<()>>,
}

impl Worker {
    pub fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Self {
        let thread = thread::spawn(move || loop {
            let message = receiver
                .lock()
                .unwrap()
                .recv()
                .expect("Unable to retrieve Message!");

            match message {
                Message::NewJob(job) => {
                    println!("Worker {} got a job; executing.", id);

                    job.call_box()
                }
                Message::Terminate => {
                    println!("Worker {} was told to terminate", id);

                    break;
                }
            };
        });

        Self {
            id,
            thread: Some(thread),
        }
    }
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

impl ThreadPool {
    /// Create a new ThreadPool
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero
    pub fn new(size: usize) -> Self {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Sending terminate message to all workers.");

        for _ in &mut self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        println!("Shutting down all workers.");

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            // we move the thread out of the `Worker` leaving a `None`
            // value on its place and allowing the `ThreadPool` take
            // ownership on the thread
            //
            // if a `Worker` thread is already `None`, then its thread
            // is already clean, and nothing happens in this case
            if let Some(thread) = worker.thread.take() {
                // if the call to `join` fails we are unwrapping
                // and go into ungraceful shutdown
                thread.join().unwrap();
            }
        }
    }
}
