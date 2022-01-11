fn main() {
    let mut user = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    println!("{}", user.email);
    user.email = String::from("anotheremail@example.com");
    println!("{}", user.email);

    let user = build_user(String::from(""), String::from(""));
    println!("{}", user.email);

    let user1 = User {
        active: user.active,
        username: user.username,
        email: String::from("another@example.com"),
        sign_in_count: user.sign_in_count
    };
    println!("{}", user1.email);

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };
    println!("{}", user2.email);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    println!("{}", black.0);
    println!("{}", origin.1);

    let subject = AlwaysEqual;
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1
    }
}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

struct Color(i32, i32, i32);

struct Point(i32, i32, i32);

struct AlwaysEqual;