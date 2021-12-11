use std::fs;
use std::error::Error;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!("search {} from {}", config.query, config.filename);

    let content = fs::read_to_string(&config.filename)?;

    println!("Content: \r\n{}", content);

    Ok(())
}

pub struct Config {
    query: String,
    filename: String
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Please at least provide 2 arguments");
        }

        Ok(Config {
            query: args[1].clone(),
            filename: args[2].clone()
        })
    }
}
