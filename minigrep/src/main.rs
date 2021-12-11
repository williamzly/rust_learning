use std::env;
use std::process;

use minigrep::Config;

fn main() {
    
    let args :Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Passing wrong arguments: {}", err);
        process::exit(1);
    });

    if let Err(err) = minigrep::run(config) {
        println!("Application failed: {}", err);
        process::exit(1);
    };

}
