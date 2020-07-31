use std::thread;
use std::sync::mpsc;
use std::sync::{Arc, Mutex};

pub type WorkerReceiver = Arc<Mutex<mpsc::Receiver<Job>>>;

pub type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool {
  workers: Vec<Worker>,
  sender: mpsc::Sender<Job>,
}

/// Represents a ThreadPool creation error
#[derive(Debug)]
pub struct PoolCreationError {
  pub message: String,
}

impl ThreadPool {
  /// Create a new ThreadPool.
  ///
  /// The size is the number of threads in the pool.
  ///
  /// # Panics
  ///
  /// The `new` function will panic if the size is zero.
  pub fn new(size: usize) -> Result<ThreadPool, PoolCreationError> {
    if size == 0 {
      return Err(PoolCreationError {
        message: format!("Invalid ThreadPool size provided {}", size)
      });
    }

    let (sender, receiver) = mpsc::channel::<Job>();

    let receiver = Arc::new(Mutex::new(receiver));

    // preallocate space in the vector for `size`
    let mut workers = Vec::with_capacity(size);

    for id in 0..size {
      workers.push(Worker::new(id, Arc::clone(&receiver)));
    }

    Ok(ThreadPool {
      workers,
      sender
    })
  }

  pub fn execute<F>(&self, f: F)
  where
    F: FnOnce() + Send + 'static,
  {
    let job = Box::new(f);

    self.sender.send(job).unwrap();
  }
}

struct Worker {
  id: usize,
  thread: thread::JoinHandle<()>,
}

impl Worker {
  fn new(id: usize, receiver: WorkerReceiver) -> Worker {
    let thread = thread::spawn(move || {
      while let Ok(job) = receiver.lock().unwrap().recv() {
        println!("Worker {} got a job; executing", id);

        job();
      }
    });

    Worker {
      id,
      thread
    }
  }
}
