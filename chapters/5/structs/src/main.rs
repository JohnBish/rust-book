struct User {
    email: String,
    username: String,
    logins: u32,
    active: bool,
}

struct Point(i32, i32, i32);

fn main() {
    let mut user1 = User {
        email: String::from("johnbrucebishop@gmail.com"),
        username: String::from("Atlas"),
        logins: 1,
        active: true,
    };

    let mut user2 = User {
        email: String::from("leocrow137@gmail.com"),
        username: String::from("Lethos"),
        ..user1
    };

    let origin = Point(0, 0, 0);
}

// User owns strings as we don't want them to go out of scope
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        logins: 1,
        active: true,
    }
}
