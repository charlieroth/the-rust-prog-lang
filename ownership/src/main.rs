fn main() {
    // Create mutable variable
    let mut s1 = String::from("hello");
    println!("s1: {}", s1);
    // Pass reference to s1 as a "mutable reference" to allow for mutating
    // a reference
    mutate(&mut s1);
    println!("s1: {}", s1);
}

fn mutate(s: &mut String) {
    return s.push_str(", world!");
}
