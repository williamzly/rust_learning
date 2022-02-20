use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;
use std::fs;
use std::thread;
use std::time::Duration;
use hello::ThreadPool;


fn main() {

    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    let mut pool = ThreadPool::new(4);

    for stream in listener.incoming() {

        let stream = stream.unwrap();
        pool.execute(|| handle_connection(stream));
    }

}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let (status_code, contents) = if buffer.starts_with(b"GET / HTTP/1.1") {
        ("200 OK", fs::read_to_string("hello.html").unwrap())
    } else if buffer.starts_with(b"GET /sleep HTTP/1.1") {
        thread::sleep(Duration::from_secs(5));
        ("200 OK", fs::read_to_string("hello.html").unwrap())
    } else {
        ("404 NOT FOUND", fs::read_to_string("404.html").unwrap())
    };

    let response = format!(
        "HTTP/1.1 {}\r\nContent-Length: {}\r\n\r\n{}",
        status_code,
        contents.len(),
        contents
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();

}

