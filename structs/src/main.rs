#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        return self.width * self.height;
    }

    fn can_fit(&self, other: &Rectangle) -> bool {
        return self.width > other.width && self.height > other.height;
    }

    // Associated function (no reference to `self`)
    fn square(size: u32) -> Self {
        Self { width: size, height: size }
    }
}

fn main() {
    let r1 = Rectangle {
        width: 30,
        height: 50
    };
    println!("Area of ({:?}) is {}", r1, r1.area());
    
    let r2 = Rectangle {
        width: 20,
        height: 40
    };
    println!("Can ({:?}) fit ({:?})? {}", r1, r2, r1.can_fit(&r2));

    // Call associated functions with `::fn_name(..)` syntax
    let s = Rectangle::square(10);
    println!("Can ({:?}) fit ({:?})? {}", r1, s, r1.can_fit(&s));
}
