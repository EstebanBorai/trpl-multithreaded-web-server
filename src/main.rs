use std::fs;
use std::thread;
use std::io::prelude::*;
use std::time::Duration;
use std::net::{TcpListener, TcpStream};

const GET_REQ: &[u8; 16] = b"GET / HTTP/1.1\r\n";
const HTTP_OK: &str = "HTTP/1.1 200 OK\r\n\r\n";
const HTTP_NF: &str = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
const SLEEP: &[u8; 21] = b"GET /sleep HTTP/1.1\r\n";

fn main() -> std::io::Result<()> {
  let listener = TcpListener::bind("127.0.0.1:7878")?;

  for stream in listener.incoming() {
    let stream = stream?;

    handle_connection(stream)?;
  }

  Ok(())
}

fn handle_connection(mut stream: TcpStream) -> std::io::Result<()> {
  let mut buff = [0; 1024];

  stream.read(&mut buff)?;

  log(&buff);

  let (status_line, filename) = if buff.starts_with(GET_REQ) {
    (HTTP_OK, "./static/hello.html")
  } else if buff.starts_with(SLEEP) {
    thread::sleep(Duration::from_secs(5));
    (HTTP_OK, "./static/hello.html")
  } else {
    (HTTP_NF, "./static/404.html")
  };

  respond_with_static_resource(stream, status_line.to_string(), filename.to_string())?;

  Ok(())
}

fn respond_with_static_resource(mut stream: TcpStream, status_line: String, filename: String) -> std::io::Result<()> {
  let contents = fs::read_to_string(filename)?;
  let response = format!("{}{}", status_line, contents);

  stream.write(response.as_bytes())?;
  stream.flush()?;

  Ok(())
}

fn log(buff: &[u8; 1024]) {
  println!("{}", String::from_utf8_lossy(buff));
}