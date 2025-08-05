fn main() {

    let users = vec![
    User { username: "alice".to_string(), password: "1234".to_string(), active: true },
    User { username: "bob".to_string(), password: "abcd".to_string(), active: false },
];

let input_username = "alice";
let input_password = "1234";

let login = login(&users, input_username, input_password);
match login {
    Ok(value) => println!("{}", value),
    Err(e) => println!("{}", e),
}



}

struct User {
    username: String,
    password: String,
    active: bool,
}

fn login(users: &Vec<User>, username: &str, password: &str) -> Result<String, String> {
    let user = users.iter().find(|u| u.username == username).ok_or_else(|| format!("User not found"))?;
    if user.active {
        if user.password == password {
            Ok("Login successful".to_string())
        } else {
            Err("Incorrect password".to_string())
        }
    } else {
        Err("User is inactive".to_string())
    }

}



