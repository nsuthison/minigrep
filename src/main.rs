use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);

    println!("Search for {}", config.to_search);
    println!("In file {}", config.filename);

    let contents = fs::read_to_string(config.filename).expect("Cannot read file.");

    println!("Content is {}", contents);
}

struct Config {
    to_search: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Config {
        Config {
            to_search: args[1].clone(),
            filename: args[2].clone(),
        }
    }
}