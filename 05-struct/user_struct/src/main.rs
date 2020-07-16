struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let user1 = create_user(String::from("123@example.com"), String::from("123"));

    let user2 = User {
        email: String::from("456@example.com"),
        username: String::from("456"),
        ..user1
    };

    let black = Color(0, 0, 0);
    let origin = Point(0, 10, 2);
}

fn create_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
