fn main() {
    let s1 = String::from("hello");
    takes_ownership(s1);

    // Cannot use s1 here because prev function call
    // moves ownership
    // println!("s1 = {}", s1);
    
    let x = 42;
    makes_copy(x);
    println!("x = {}", x);
}

fn takes_ownership(some_str: String) {
    println!("some_str = {}", some_str);
    // `some_str` comes into scope and once function is about to return it calls
    // `drop()` on `some_str`
}

fn makes_copy(some_int: i32) {
    println!("some_int = {}", some_int);
}
