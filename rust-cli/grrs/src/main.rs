// src: https://rust-cli.github.io/book/index.html

//use std::env;
use clap::Parser;

/// Search for a pattern in a file and display the lines that contain it
#[derive(Parser, Debug)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    path: std::path::PathBuf,
}

fn main() {
    let args: Cli = Cli::parse();
    println!("{:?}", args);
}
