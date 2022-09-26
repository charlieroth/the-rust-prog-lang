#![allow(dead_code, unused_variables)]

fn main() {
    let config_max = Some(32);

    if let Some(max) = config_max {
        println!("The max is configured to be {}", max);
    }
}
