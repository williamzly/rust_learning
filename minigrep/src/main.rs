use std::env;
use std::fs;

fn main() {
    
    let args :Vec<String> = env::args().collect();

    let config = Config::new(&args);

    println!("search {} from {}", config.query, config.filename);

    let content = match fs::read_to_string(&config.filename) {
        Ok(c) => c,
        Err(e) => panic!("Failed to read file `{}`:{}", config.filename, e)
    };


    println!("Content: \r\n{}", content);

}

struct Config {
    query: String,
    filename: String
}

impl Config {
    pub fn new(args: &[String]) -> Config {
        Config {
            query: args[1].clone(),
            filename: args[2].clone()
        }
    }
}
