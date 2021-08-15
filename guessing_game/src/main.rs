use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Welcome to guessing game");

    let secret = rand::thread_rng().gen_range(0..101);

    loop {
        println!("Please type a number[0-100] to guess");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };

        match guess.cmp(&secret) {
            Ordering::Greater => println!("Too bigger!"),
            Ordering::Less => println!("Too smaller!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }

}
