use std::sync::{Mutex, Arc};
use std::thread;
use std::time::Duration;

fn main() {

    let lock1 = Arc::new(Mutex::new("Lock1"));
    let lock2 = Arc::new(Mutex::new("Lock2"));

    let lock1_1 = lock1.clone();
    let lock2_1 = lock2.clone();
    let t1 = thread::spawn(move || {
        let v1 = lock1_1.lock().unwrap();
        println!("Get {} from thread1", v1);
        thread::sleep(Duration::from_secs(1));
        println!("Thread1 awake from sleep");
        lock2_1.lock();
        println!("You will never see this print");
    });
    let lock1_2 = lock1.clone();
    let lock2_2 = lock2.clone();
    let t2 = thread::spawn(move || {
        let v2 = lock2_2.lock().unwrap();
        println!("Get {} from thread2", v2);
        thread::sleep(Duration::from_secs(1));
        println!("Thread2 awake from sleep");
        lock1_2.lock();
        println!("You will never see this print");
    });

    t1.join().unwrap();
    t2.join().unwrap();

}
