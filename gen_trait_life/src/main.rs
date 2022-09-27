#![allow(dead_code, unused_variables)]

struct Point<T> {
    x: T,
    y: T,
}

struct TwoTypePoint<T, U> {
    x: T,
    y: U,
}

fn main() {
    let integer_point = Point { x: 4, y: 3 };
    let float_point = Point { x: 4.0, y: 3.8 };
    let float_integer_point = TwoTypePoint { x: 4.0, y: 3 };
}

enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}

