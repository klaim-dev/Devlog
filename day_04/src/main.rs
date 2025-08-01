fn main() {
    let mut name = User {
        name: String::from("Alice"),
        password: String::from("secret123"),
        is_logged_in: false,
    };

    let mut name2= User{
        name: String::from("Bob"),
        password: String::from("hunter2"),
        is_logged_in: false,
    };

    let try1 = try_login(&mut name, "Alice".to_string(), "secret123".to_string());
    let try2 = try_login(&mut name2, "Bob".to_string(), "123".to_string());

    println!("{}", try1);
    println!("{}", try2);
    println!("{}", name.is_logged_in);
    println!("{}", name2.is_logged_in);
}

struct User {
    name: String,
    password: String,
    is_logged_in: bool,
}

fn try_login(user: &mut User, input_name: String, input_password: String) -> String {
    let status = if user.name == input_name {
        if user.password == input_password {
            user.is_logged_in = true;
            format!("Access denied for {}", input_name)
        } else {
            format!("Incorrect input password")
        }
    } else {
        format!("Incorrect input name")
    };
    status
}

