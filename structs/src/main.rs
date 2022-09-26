#[allow(dead_code)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64
}

fn main() {
    let charlie = User {
        active: true,
        username: String::from("charlie"),
        email: String::from("charlie@email.com"),
        sign_in_count: 1,
    };
    let active_msg = if charlie.active { "is" } else { "is not" };
    println!("user {} {} active", charlie.username, active_msg);

    let mut miranda = User {
        active: false,
        username: String::from("miranda"),
        email: String::from("miranda@email.com"),
        sign_in_count: 0,
    };

    let active_msg = if miranda.active { "is" } else { "is not" };
    println!("user {} {} active", miranda.username, active_msg);

    miranda.active = true;

    let active_msg = if miranda.active { "is" } else { "is not" };
    println!("user {} {} active", miranda.username, active_msg);
}
