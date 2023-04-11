use std::fs;
use std::error::Error;
use std::env;

pub struct Config {
    pub query: String,
    pub filepath: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build (mut args: impl Iterator<Item=String>) -> Result<Config, &'static str> {
        args.next();

        let query: String = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };
        let filepath: String = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };
        let ignore_case: bool = env::var("IGNORE_CASE").is_ok();
        return Ok(Config {
            query: query,
            filepath: filepath,
            ignore_case: ignore_case,
        });
    }
}

pub fn run (config: Config) -> Result<(), Box<dyn Error>> {
    let contents: String = fs::read_to_string(config.filepath)?;
    let results =
        if config.ignore_case {
            search_case_insensitive(&config.query, &contents)
        }
        else {
            search(&config.query, &contents)
        };

    for line in results {
        println!("{line}");
    }
    return Ok(());
}

pub fn search<'a> (query: &str, contents: &'a str) -> Vec<&'a str> {
    return contents.lines()
                .filter(|line| line.contains(query))
                .collect();
}

pub fn search_case_insensitive<'a> (query: &str, contents: &'a str) -> Vec<&'a str> {
    return contents.lines()
        .filter(|line| line.to_lowercase()
                           .contains(&query.to_lowercase()))
        .collect();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query: &str = "duct";
        let contents: &str = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query: &str = "rUsT";
        let contents: &str = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(vec!["Rust:", "Trust me."],
                   search_case_insensitive(query, contents));
    }
}
