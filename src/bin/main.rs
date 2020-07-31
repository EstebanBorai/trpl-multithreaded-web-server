use std::net::{TcpListener};
use web_server::{ThreadPool, handle_connection};

fn main() -> std::io::Result<()> {
  let listener = TcpListener::bind("127.0.0.1:7878")?;
  let pool = ThreadPool::new(4).unwrap();

  for stream in listener.incoming() {
    let stream = stream?;

    pool.execute(|| {
      handle_connection(stream);
    });
  }

  Ok(())
}
