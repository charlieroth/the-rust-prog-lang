#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        return self.width * self.height;
    }
}

fn main() {
    let rectangle = Rectangle {
        width: 30,
        height: 50
    };

    println!("area({:?}) = {}", rectangle, rectangle.area());
}
