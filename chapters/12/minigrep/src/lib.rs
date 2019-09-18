use std::error::Error;
use std::fs;
use std::env;
use std::io::{ErrorKind, Error as IOError};

pub mod parser;

use crate::parser::Params;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(params: &mut Params) -> Result<Config, ()> {
        params.next()?; // First param is program name
        
        Ok(
            Config {
                query: params.next()?,
                filename: params.next()?,
                case_sensitive: env::var("CASE_INSENSITIVE").is_err(),
            }
        )
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut matches = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            matches.push(line);
        }
    }

    matches
}

pub fn run(cfg: Config) -> Result<(), Box<dyn Error>> {
    //println!("Searching for {} in file {}", cfg.query, cfg.filename);

    let contents = fs::read_to_string(cfg.filename)?;
        
    //println!("Contents: {}", contents);

    let matches = if cfg.case_sensitive {
        search(&cfg.query, &contents)
    } else {
        search_case_insensitive(&cfg.query, &contents)
    };

    for line in matches {
        println!("{}", line);
    }

    Ok(())
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

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
