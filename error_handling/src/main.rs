#![allow(dead_code, unused_variables)]
use std::{fs::File, io::ErrorKind};

fn main() {
    let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating file: {:?}", error);
            })
        } else {
            panic!("Problem opening file: {:?}", error)
        }
    });
}
