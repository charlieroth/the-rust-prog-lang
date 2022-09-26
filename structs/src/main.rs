#[allow(dead_code)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64
}

fn main() {
    let charlie = build_user(
        String::from("charlie@email.com"),
        String::from("charlie")
    );
    let active_msg = if charlie.active { "is" } else { "is not" };
    println!("{} {} active", charlie.username, active_msg);
}

fn build_user(email: String, username: String) -> User {
    User {
        active: false,
        username,
        email,
        sign_in_count: 0,
    }
}
