use std::fs;
use std::env;
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
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Please at least provide 2 arguments");
        }

        let mut case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        if args.len() > 3 {
            case_sensitive = args[3] == "true";
        }
        Ok(Config {
            query: args[1].clone(),
            filename: args[2].clone(),
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
