fn main() {
    let user_1 = User {
        name: "Alice".to_string(),
        email: Some ("alice@gmaol.com".to_string()),
        is_active: true,
    };

    let user_2 = User {
        name: "Bob".to_string(),
        email: None,
        is_active: true,
    };

    let send = send_email(&user_2);
    println!("{}", send);
}

struct User {
    name: String,
    email: Option<String>,
    is_active: bool,
}

fn send_email(user: &User) -> String {
    let status = match &user {
        User{is_active: false, ..} => {
        format!("Error: user not active")
    }
    User{email: None, ..} => {
        format!("Error: no email provided")
    }
    _ => match &user.email {
        Some(a) => format!("Email sent to {}", a.to_string()),
        None => format!("No email provide"),
    }
    };
    status
}