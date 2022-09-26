fn main() {
    let s1 = gives_ownership();
    println!("s1 = {}", s1);
    let s2 = String::from("world");
    let s3 = takes_and_gives(s2);

    // compiler error: ownership moved to s3
    // println!("s2 = {}", s2);

    println!("s3 = {}", s3);
}

fn gives_ownership() -> String {
    let s = String::from("hello");
    return s;
}

fn takes_and_gives(s: String) -> String {
    s
}
