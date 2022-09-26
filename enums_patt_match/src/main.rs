#![allow(dead_code, unused_variables)]

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn reroll() {}

fn main() {
    let dice_roll = 8;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => reroll(),
    }
}
