use std::fs;
use std::env;
use std::env::Args;
use std::error::Error;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!("search {} from {}", config.query, config.filename);

    let content = fs::read_to_string(&config.filename)?;

    for line in search(&config.query, &content, config.case_sensitive) {
        println!("{}", line);
    }

    Ok(())
}

fn search<'a>(query: &str, content: &'a str, case_sensitive: bool) -> Vec<&'a str> {
    let query_to_use = if case_sensitive {
        query.to_string()
    } else{
        query.to_lowercase()
    };
    let lines = content.lines();
    let mut result = Vec::new();
    for line in lines {
        if (!case_sensitive && line.to_lowercase().contains(&query_to_use)) || line.contains(&query_to_use) {
            result.push(line);
        }
    }
    result
}

pub struct Config {
    query: String,
    filename: String,
    case_sensitive: bool
}

impl Config {
    pub fn new(mut args: Args) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(query) => query,
            None => { return Err("Failed to get query arg"); }
        };

        let filename = match args.next() {
            Some(filename) => filename,
            None => { return Err("Failed to get filename arg"); }
        };

        let case_sensitive = match args.next() {
            Some(case_sensitive) => case_sensitive == "true",
            None => env::var("CASE_INSENSITIVE").is_err()
        };

        Ok(Config {
            query,
            filename,
            case_sensitive
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn search_case_sensitive() {
        let query = "duct";
        let content = "\
Rust
safe, fast, productive.
Pick three.
DUCT";
        assert_eq!(
          vec!("safe, fast, productive."),
          search(query, content, true)
        );
    }

    #[test]
    fn search_case_insensitive() {
        let query = "duct";
        let content = "\
Rust
safe, fast, productive.
Pick three.
DUCT";
        assert_eq!(
          vec!("safe, fast, productive.", "DUCT"),
          search(query, content, false)
        );
    }

}
