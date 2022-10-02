use std::fs;
use std::error::Error;
use std::env::Args;

pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    pub fn build(mut args: Args) -> Result<Config, &'static str> {
        // Skip first argument since it is the binary we are executing
        args.next();
        
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Query string not provided"),
        };
        
        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("File path not provided"),
        };

        return Ok(Config { query, file_path });
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    let search_results = search(&config.query, &contents);

    if search_results.len() == 0 {
        println!("No search results found for query: '{}'", &config.query);
        return Ok(());
    }

    for result in search_results {
        println!("'{}' found in '{}'", &config.query, result);
    }

    return Ok(());
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    return contents
        .lines()
        .filter(|line| line.contains(query))
        .collect();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}
