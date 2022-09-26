struct Color(i32, i32, i32);

struct Coordinate(i32, i32, i32);

fn main() {
    let black = Color(0,0,0);
    let origin = Coordinate(0,0,0);
    println!("r value in black: {}", black.0);
    println!("x value in origin: {}", origin.0);
}
