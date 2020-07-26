use std::thread;

pub struct ThreadPool {
  threads: Vec<thread::JoinHandle<()>>,
}

/// Represents a ThreadPool creation error
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
    if !size > 0 {
      return Err(PoolCreationError {
        message: format!("Invalid ThreadPool size provided {}", size)
      });
    }

    // preallocate space in the vector for `size`
    let mut threads = Vec::with_capacity(size);

    for _ in 0..size {

    }

    Ok(ThreadPool {
      threads
    })
  }

  pub fn execute<F>(&self, f: F)
  where
    F: FnOnce() + Send + 'static,
  {}
}
