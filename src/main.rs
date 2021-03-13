use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = parse_config(&args);

    println!("Search for {}", config.to_search);
    println!("In file {}", config.filename);

    let contents = fs::read_to_string(config.filename).expect("Cannot read file.");

    println!("Content is {}", contents);
}

fn parse_config(args: &[String]) -> Config {
    Config {
        to_search: args[1].clone(),
        filename: args[2].clone(),
    }
}

struct Config {
    to_search: String,
    filename: String,
}
