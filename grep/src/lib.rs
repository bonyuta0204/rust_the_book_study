use std::fs;
use std::{env::Args, error::Error};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(&config.filename)?;

    let lines = search(&config.query, &content);

    for line in lines {
        println!("{line}")
    }
    Ok(())
}

fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut matched: Vec<_> = Vec::new();
    for line in content.lines() {
        if line.contains(query) {
            matched.push(line)
        }
    }
    matched
}

pub struct Config {
    pub query: String,
    pub filename: String,
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

        Ok(Config { query, filename })
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
}
