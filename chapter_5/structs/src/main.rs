fn main() {
    println!("Hello, world!");

    let mut user1 = build_user("max@jellycat.fr", "Toto");

    println!("{}'s email is {}", user1.username, user1.email);

    user1.username = String::from("Max");

    println!("User 1 name is now {}", user1.username);

    let user2 = User {
        username: String::from("Angus"),
        email: String::from("angus@baby.fr"),
        ..user1
    };

    println!("User 2 name is {}", user2.username);

    let black = Color(0, 0, 0);
    let position = Point(64.0, 32.0);
}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(email: &str, username: &str) -> User {
    User {
        username: String::from(username),
        email: String::from(email),
        sign_in_count: 0,
        active: true,
    }
}

struct Color(u8, u8, u8);
struct Point(f64, f64);
