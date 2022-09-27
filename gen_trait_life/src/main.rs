#![allow(dead_code, unused_variables)]

struct Point<T> {
    x: T,
    y: T,
}

// Generics on struct `impl` block
impl<T> Point<T> {
    fn x(&self) -> &T {
        return &self.x;
    }
    
    fn y(&self) -> &T {
        return &self.y;
    }
}

// Specify constraints on generic types
// This implementation is only availble on Point 
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        return (self.x.powi(2) + self.y.powi(2)).sqrt();
    }
}

fn main() {
    let p1 = Point { x: 4, y: 3 };
    let p2 = Point { x: 4.0, y: 3.8 };
}
