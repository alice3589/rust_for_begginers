fn main() {
    println!("Hello, world!");
}

fn build_user(email: String, username: String) {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}