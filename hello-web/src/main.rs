use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;
use std::fs;
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::sync::Mutex;
use std::sync::Arc;

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


type Job = Box<dyn FnOnce() + Send + 'static>;

enum Message {
    NewJob(Job),
    Terminate
}


struct ThreadPool {
    workers: Vec<Worker>,
    sender: Sender<Message>
}

impl ThreadPool {

    pub fn new(size: usize) -> ThreadPool {
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id, receiver.clone()));
        };
        ThreadPool {
            workers,
            sender
        }
    }

    pub fn execute<T: FnOnce() + Send + 'static>(&mut self, job: T) {
        let job = Box::new(job);
        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

impl Drop for ThreadPool {

    fn drop(&mut self) {
        println!("Start terminate all threads");
        for _ in &self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }
        for worker in &mut self.workers {
            if let Some(thread) = worker.thread.take(){
                println!("Sending terminate msg to worker {}", worker.id);
                thread.join().unwrap();
            }
        }
    }
}

struct Worker {

    id: usize,
    thread: Option<JoinHandle<()>>

}

impl Worker {

    pub fn new(id: usize, receiver: Arc<Mutex<Receiver<Message>>>) -> Worker {
        Worker {
            id,
            thread: Some(thread::spawn(move || {

                loop {
                    let message = receiver.lock().unwrap().recv().unwrap();
                    match message {
                        Message::NewJob(job) => {
                            println!("Worker {} get a job; executing.", id);
                            job();
                        },
                        Message::Terminate => {
                            println!("Worker {} is going to terminate.", id);
                            break;
                        }
                    }

                };
            }))
        }
    }

}
