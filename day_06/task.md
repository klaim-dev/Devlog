## 🧠 Super Task — User Email System (Level 3, Day 6)

### 🎯 Description

You're building part of a backend system that manages **user profiles** and **email sending**.  
Each user has an **optional email** and a **status** flag.

```rust
struct User {
    name: String,
    email: Option<String>,
    is_active: bool,
}

