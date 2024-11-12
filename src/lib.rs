use std::error::Error;
use std::{env, fs};

#[derive(Debug)]
pub struct Config {
    pub query: String,
    pub path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        let query = args[1].clone();
        let path = args[2].clone();
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config { query, path, ignore_case })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.path)?;
    
    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for result in results {
        println!("{}", result);
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            result.push(line);
        }
    }

    result
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_data() -> &'static str {
        "\
        Rust:\nsafe, fast, productive. Pick three.\nDuct tape.\nTrust me."
    }

    #[test]
    fn case_sensitive() {
        let query = "duct";
        assert_eq!(vec!["safe, fast, productive. Pick three."], search(query, get_test_data()));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        assert_eq!(vec!["Rust:", "Trust me."], search_case_insensitive(query, get_test_data()));
    }
}