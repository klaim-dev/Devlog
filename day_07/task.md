🧠 Super Task — Login System with Error States
* Built a full login() function with the following logic:

* Err("User not found") — when username missing

* Err("Incorrect password") — when password mismatched

* Err("User is inactive") — if account is inactive

* Ok("Login successful") — on valid and active login

fn login(users: &Vec<User>, username: &str, password: &str) -> Result<String, String>

✅ Used: .find(), ok_or(), early returns with ?, clean match blocks.

🔍 Reflections
Today’s topic is core to writing backend-level code in Rust:

* Safe fallibility

* Domain-specific error messaging

* Clean flow control without panic

* Building robust logic under uncertainty

This is production logic, not toy code.


