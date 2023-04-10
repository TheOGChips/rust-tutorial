use std::fs;
use std::error::Error;

pub struct Config {
    pub query: String,
    pub filepath: String,
}

impl Config {
    pub fn build (args: Vec<String>) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query: String = args[1].clone();
        let filepath: String = args[2].clone();
        return Ok(Config {
            query: query,
            filepath: filepath,
        });
    }
}

pub fn run (config: Config) -> Result<(), Box<dyn Error>> {
    let contents: String = fs::read_to_string(config.filepath)?;
    for line in search(&config.query, &contents) {
        println!("{line}");
    }
    return Ok(());
}

pub fn search<'a> (query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results: Vec<&str> = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    return results;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query: &str = "safe, fast, productive.";
        let contents: &str = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}
