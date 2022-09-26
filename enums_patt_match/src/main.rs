#![allow(dead_code, unused_variables)]

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(c: Coin) -> u8 {
    match c {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn print_value_in_cents(c: Coin) {
    match c {
        Coin::Penny => {
            println!("A penny is worth 1 cent");
        },
        Coin::Nickel => {
            println!("A nickel is worth 5 cents");
        },
        Coin::Dime => {
            println!("A nickel is worth 10 cents");
        },
        Coin::Quarter => {
            println!("A nickel is worth 25 cents");
        },
    }
}

fn main() {
    let c = Coin::Quarter;
    print_value_in_cents(c);
}
