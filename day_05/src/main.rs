fn main() {
    let mut user = User {
        name: "Alice".to_string(),
        messages: vec![],
        is_active: true,
    };

    let add_message = send_message(&mut user, "Hello");
    println!("{}", add_message);

    let summary = print_summary(&user);
    println!("{}", summary);

    let deactivate = deactivate(&mut user);
    println!("{}", deactivate);



}

struct User {
    name: String,
    messages: Vec<String>,
    is_active: bool,
}

fn send_message(user: &mut User, message: &str) -> String {
    let add = if user.is_active {
        user.messages.push(message.to_string());
        format!("Message add")
    } else {
        format!("User not active")
    };
    add
}

fn print_summary(user: &User) -> String {
    let summary = user.messages.iter().len();
    let a = format!("User {} has {} messages", user.name, summary);
    a
}

fn deactivate(user: &mut User) -> String {
    let deactivate = if user.is_active {
        user.is_active = false;
        format!("User deactivate")
    } else {
        format!("User allready deactivated")
    };
    deactivate
}


