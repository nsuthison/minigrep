use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let to_find = &args[1];
    let filename = &args[2];

    println!("Search for {}", to_find);
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename).expect("Cannot read file.");

    println!("Content is {}", contents);
}
