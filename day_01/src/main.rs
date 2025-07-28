fn main() {
let first_name: &str = "Alice";
let last_name: &str = "Johnson";
let age: u32 = 30;

let mut greeting = String::new();
greeting.push_str("Hello, ");
greeting.push_str(first_name);
greeting.push_str(" ");
greeting.push_str(last_name);
greeting.push_str("! You are ");
greeting.push_str(&age.to_string());
greeting.push_str(" years old.");

println!("{}", greeting);
}
