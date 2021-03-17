use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", String::from(err));
        process::exit(1);
    });

    println!("Search for {}", config.to_search);
    println!("In file {}", config.filename);

    if let Err(error) = minigrep::run(config) {
        eprintln!("Application error: {}", error);

        process::exit(1);
    };
}
