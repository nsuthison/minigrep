use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
            println!("{}", String::from(err));
            process::exit(1);
    });

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
    fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        Ok(Config {
            to_search: args[1].clone(),
            filename: args[2].clone(),
        })
    }
}
