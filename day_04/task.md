### 🧠 Super Task — User Login System

I'm built a small **authentication system** that simulates real backend logic using ownership mechanics.

#### Features:
- User struct with name, password, login status
- Safe string comparisons using `.as_str()`
- No `.clone()` used — ownership handled manually
- `try_login` function updates login state based on input

#### Example Output:
```rust
Welcome, alice!
Access denied for bob
user1 logged in: true
user2 logged in: false

