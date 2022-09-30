use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let search_str = &args[1];
    println!("search string: {}", search_str);

    let file_path = &args[2];
    println!("file path: {}", file_path);
}
