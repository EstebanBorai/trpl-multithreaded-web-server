use std::net::{TcpListener};
use web_server::handle_connection;

fn main() -> std::io::Result<()> {
  let listener = TcpListener::bind("127.0.0.1:7878")?;

  for stream in listener.incoming() {
    let stream = stream?;

    handle_connection(stream)?;
  }

  Ok(())
}
