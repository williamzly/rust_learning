use std::thread;
use std::thread::JoinHandle;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::sync::Mutex;
use std::sync::Arc;

type Job = Box<dyn FnOnce() + Send + 'static>;

enum Message {
    NewJob(Job),
    Terminate
}


pub struct ThreadPool {
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

    fn new(id: usize, receiver: Arc<Mutex<Receiver<Message>>>) -> Worker {
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
