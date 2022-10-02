struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        return MyBox(x);
    }
}

use std::ops::Deref;
impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        return &self.0;
    }
}

fn main() {
    let a = 5;
    let b = MyBox::new(a);
    println!("b = {}", *b);
}
