#[allow(dead_code)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64
}

fn main() {
    let charlie = User {
        active: false,
        email: String::from("charlie@email.com"),
        username: String::from("charlie"),
        sign_in_count: 0,
    };

    let charlie_active = User {
        active: true,
        username: String::from("charlie_active"),
        ..charlie
    };

    let active_msg = if charlie.active { "is" } else { "is not" };
    println!("{} {} active", charlie.username, active_msg);
    
    let active_msg = if charlie_active.active { "is" } else { "is not" };
    println!("{} {} active", charlie_active.username, active_msg);

    // Compiler Error: cannot use `charlie` because ownership was moved to
    // `charlie_active` with `..` syntax
    //
    // let charlie_active = User {
    //     active: true,
    //     username: String::from("charlie_active"),
    //     ..charlie
    // };
}
