## 📅 Day 11 — Modules (`mod`, `use`, `pub`)

### 🧠 What I'm Learned
Today I'm studied Rust’s module system:
- `mod` to declare modules
- `use` to import and shorten paths
- `pub` to control visibility
- File structure (`mod.rs`, submodules)
- Splitting logic into multiple focused units

This is the foundation of real project architecture in Rust — clean separation between domain, data, and business logic.

---

### 💻 What I'm Built

I'm created a **Post Management System** split into 3 modules:

- `db.rs`: owns the data (`Vec<Post>`) and tracks the next ID
- `create_posts.rs`: handles adding new posts
- `all_posts.rs`: displays all posts or a specific one

I'm used:
- `Result` and `Option` for all logic
- Ownership and borrowing correctly
- Clear data validation (no duplicates, no empty titles)

---

### 🧪 Example

```rust
let result = manager.add_post("Rust is powerful!".to_string());
match result {
    Ok(msg) => println!("{}", msg),
    Err(err) => println!("Error: {}", err),
}

manager.print_all();

