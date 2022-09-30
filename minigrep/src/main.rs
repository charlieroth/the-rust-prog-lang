use std::env;
use std::fs;

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn from(args: &[String]) -> Config {
        if args.len() < 3 {
            panic!("Not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();
        return Config { query, file_path };
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::from(&args);
    let contents = fs::read_to_string(config.file_path)
        .expect("Failed to read file");
    println!("contents: {}", contents);
}
