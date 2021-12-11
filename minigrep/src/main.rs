use std::env;
use std::fs;

fn main() {
    
    let args :Vec<String> = env::args().collect();

    let query = &args[1];
    let filename = &args[2];

    println!("search {} from {}", query, filename);

    let content = match fs::read_to_string(filename) {
        Ok(c) => c,
        Err(e) => panic!("Failed to read file `{}`:{}", filename, e)
    };


    println!("Content: \r\n{}", content);

}
