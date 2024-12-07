struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    print_user_details();
}

fn print_user_details() {
    let mut user1 = User {
        email: String::from("pransandip@gmail.com"),
        username: String::from("sandy123"),
        active: true,
        sign_in_count: 1,
    };

    let name = user1.username;
    user1.username = String::from("kali223");

    let user2 = build_user(String::from("alex@mail.com"), String::from("alex001"));

    let user3 = User {
        email: String::from("james@gmail.com"),
        username: String::from("james321"),
        ..user2
    };

    println!(
        "name: {}, user3.email: {}, sign_in_count: {}, active: {}",
        name, user3.email, user3.sign_in_count, user3.active
    );
}

fn build_user(email: String, username: String) -> User {
    // returning new instance of User
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
