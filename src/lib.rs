use std::error::Error;
use std::{env, fs};

#[derive(Debug)]
pub struct Config {
    pub query: String,
    pub path: String,
    pub ignore_case: bool,
}

impl Config {
    /// Create config
    /// # Example
    /// ```
    /// use std::env;
    /// use derek_minigrep::Config;
    ///
    /// let config = Config::build(vec!["minigrep".to_string(), "query".to_string(), "path".to_string()].into_iter()).expect("create config error");
    /// assert_eq!(config.query, "query".to_string());
    /// assert_eq!(config.path, "path".to_string());
    /// assert_eq!(config.ignore_case, false);
    /// ```
    pub fn build(mut args: impl Iterator<Item=String>) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };

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
    contents.lines().filter(|line| { line.contains(query) }).collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines().filter(|line| { line.to_lowercase().contains(&query.to_lowercase()) }).collect()
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