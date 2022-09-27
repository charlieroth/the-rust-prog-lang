#![allow(dead_code, unused_variables)]

trait Summary {
    fn summarize(&self) -> String;
}

struct Tweet {
    username: String,
    content: String,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        return format!("@{} tweeted '{}'", self.username, self.content);
    }
}

fn main() {
    let t = Tweet { 
        username: String::from("charlie"),
        content: String::from("Just ate a sandwhich"),
    };
    println!("{}", t.summarize());
}
