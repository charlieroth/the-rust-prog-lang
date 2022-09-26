fn main() {
    let full_name = String::from("Charles Thomas Roth");
    let first = first_name(&full_name);
    println!("first name: '{}'", first);
}

fn first_name(name: &String) -> &str {
    let bytes = name.as_bytes();

    for (i, &item) in bytes.into_iter().enumerate() {
        // if item is equal to space (as byte)
        if item == b' ' {
            return &name[0..i];
        }
    }

    return &name[0..name.len()];
}
