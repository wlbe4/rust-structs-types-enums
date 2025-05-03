struct User {
    username: String,
    email: String,
    uri: String,
    active: bool,
}

impl User {
    fn new(username: String, email: String, uri: String) -> Self {
        Self {
            username,
            email,
            uri,
            active: true,
        }
    }
    fn from_email(email: String) -> Self {
        Self {
            username: email.split('@').next().unwrap_or_default().to_string(),
            email,
            uri: String::new(),
            active: true,
        }
    }
    fn deactivate(&mut self) {
        self.active = false;
    }
}

fn main() {
    let mut new_user = User::new(
        String::from("alfredodeza"),
        String::from("alfreodeza@example.com"),
        String::from("https://alfredodeza.com"),
    );
    println!("Hello, {}!", new_user.username);
    println!("Account {} status is: {}", new_user.username, new_user.active);
    new_user.deactivate();
    println!("Account {} status is: {}", new_user.username, new_user.active);
    let user_from_email = User::from_email(String::from("lior@gmail.com"));
    println!("Hello, {}!", user_from_email.username);
}
