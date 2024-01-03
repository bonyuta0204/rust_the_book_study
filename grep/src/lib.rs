use std::{env, fs};
use std::{env::Args, error::Error};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(&config.filename)?;

    let lines = if config.case_sensitive {
        search(&config.query, &content)
    } else {
        case_insensitive_search(&config.query, &content)
    };

    for line in lines {
        println!("{line}")
    }
    Ok(())
}

fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    content
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

fn case_insensitive_search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    content
        .lines()
        .filter(|line| line.to_lowercase().contains(&query.to_lowercase()))
        .collect()
}

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn build(mut args: Args) -> Result<Self, &'static str> {
        args.next();

        let query = match args.next() {
            Some(query) => query,
            None => return Err("You must specify query as first argument"),
        };
        let filename = match args.next() {
            Some(filename) => filename,
            None => return Err("You must specify filename as second argument"),
        };

        let case_sensitive = match env::var("IGNORE_CASE") {
            Ok(_) => true,
            Err(_) => false,
        };

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents))
    }

    #[test]
    fn case_sensitive() {
        let query = "rust";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["Rust:"], case_insensitive_search(query, contents))
    }
}
