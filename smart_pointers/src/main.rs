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

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    // Deref coercion can convert `&String` to `&str` because String 
    // implements the Deref trait such that it returns `&str`
    let name = MyBox::new(String::from("Charlie"));
    hello(&name);

    let name = CustomSmartPointer {
        data: String::from("Charlie")
    };
    println!("name: {}", name.data);
    println!("CustomSmartPointer created.");
}

fn hello(name: &str) {
    println!("hello, {}!", name);
}
