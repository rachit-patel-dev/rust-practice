struct User {
    email: String,
    username: String,
    active: bool,
    sign_in_count: u64,
}

fn main() {
    println!("Hello, world!");

    let mut user1 = User {
        email: String::from("1@email.com"),
        username: String::from("111"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("11@email.com");
    user1.username = String::from("1111");
    user1.active = false;
    user1.sign_in_count = 2;

    let user2 = build_user(String::from("2@email.com"), String::from("222"));
    println!("User2 Email = {}", user2.email);

    let user3 = User {
        email: String::from("3@email.com"),
        ..user1
    };
    println!("User3 Email = {}", user3.email);
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
