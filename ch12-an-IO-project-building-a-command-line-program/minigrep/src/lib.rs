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
    println!("With text:\n{contents}");
    return Ok(());
}
