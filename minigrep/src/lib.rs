use std::fs;
use std::error::Error;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!("search {} from {}", config.query, config.filename);

    let content = fs::read_to_string(&config.filename)?;

    for line in search(&config.query, &content) {
        println!("{}", line);
    }

    Ok(())
}

fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let lines = content.lines();
    let mut result = Vec::new();
    for line in lines {
        if line.contains(query) {
            result.push(line);
        }
    }
    result
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let content = "\
Rust
safe, fast, productive.
Pick three.";
        assert_eq!(
          vec!("safe, fast, productive."),
          search(query, content)
        );
    }

}
