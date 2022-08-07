struct User {
    _active: bool,
    username: String,
    email: String,
    _sign_in_count: u64,
}

struct UserAlt<'a> {
    _active: bool,
    username: &'a str,
    _email: &'a str,
    _sign_in_count: u64,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

struct AlwaysEqual;

fn _build_user(email: String, username: String) -> User {
    User {
        _active: true,
        username,
        email,
        _sign_in_count: 1,
    }
}

fn main() {
    let mut user1 = User {
        _active: true,
        username: String::from("someoneusername123"),
        email: String::from("someone@example.com"),
        _sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };

    // println!("Value of user1.username: {}", user1.username);
    println!("Value of user2.username: {}", user2.username);

    let user_alt = UserAlt {
        _active: true,
        username: "someoneusername123",
        _email: "someone@example.com",
        _sign_in_count: 1,
    };

    println!("Value of user_alt.username: {}", user_alt.username);

    let _black = Color(0, 0, 0);
    let _origin = Point(0, 0, 0);

    let _subject = AlwaysEqual;
}
