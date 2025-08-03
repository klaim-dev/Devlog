### 💥 Super Task: User Message System

We built a **small user messaging system** with clear separation of logic:

```rust
struct User {
    name: String,
    messages: Vec<String>,
    is_active: bool,
}

fn send_message(user: &mut User, message: &str) {
    if user.is_active {
        user.messages.push(message.to_string());
    }
}

fn print_summary(user: &User) {
    println!("User {} has {} messages.", user.name, user.messages.len());
}

fn deactivate(user: &mut User) {
    user.is_active = false;
}

