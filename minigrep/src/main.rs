use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let search_str = &args[1];
    let file_path = &args[2];
    let contents = fs::read_to_string(file_path)
        .expect("Failed to read file");
    println!("contents: {}", contents);
}
