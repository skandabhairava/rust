use std::time::Duration;
use std::{fs, thread};
use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;
use std::error::Error;

use webserver::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    let pool = ThreadPool::new(4);

    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }

    println!("Shutting down.")
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    println!("Someone connected to the server.");

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    if buffer.starts_with(get) {
        send_html("index.html", "200 OK", &mut stream).unwrap()
    }else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(10));
        send_html("index.html", "200 OK", &mut stream).unwrap()
    }

    send_html("404.html", "404 NOT FOUND", &mut stream).unwrap()
}

fn send_html(html: &str, status: &str, stream: &mut TcpStream) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(html)?;
    let response = format!(
        "HTTP/1.1 {}\r\nContent-Length: {}\r\n\r\n{}",
        status,
        contents.len(),
        contents
    );
    stream.write(response.as_bytes())?;
    stream.flush()?;
    Ok(())
}