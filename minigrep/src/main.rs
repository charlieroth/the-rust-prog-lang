use std::env;
use std::fs;
use std::process;
use std::error::Error;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|error| {
        println!("Problem parsing arguments: {error}");
        process::exit(1);
    });

    if let Err(e) = run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();
        return Ok(Config { query, file_path });
    }
}

// `Box<dyn Error>` is a type that implements the error trait
// and is a pointer allocated on the heap
fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // `?` operator will return the error value from the current
    // function for the caller to handle
    let contents = fs::read_to_string(config.file_path)?;
    println!("contents: {}", contents);
    return Ok(());
}
