🧠 Super Task — Core Concepts Covered:
- Ownership and borrowing (`move`, `&`, `&mut`)
- Error handling with `Result<T, E>` and `Option<T>`
- Data structures: `Vec<T>`, `HashMap<K, V>`, and nested models
- Methods and struct architecture (`impl`, `self`, `&self`)
- Pattern matching: `match`, `if let`, `.map()`, `.get()`, `.entry()`

#### 📦 System Functionality:
- Register users (ensures no duplicates)
- Create orders (prevents duplicate IDs)
- Display all orders for a specific user
- Calculate total money spent

```rust
let mut db = Manager {
    user: vec![],
    all_order: HashMap::new(),
};

// Register user
db.register_user(User {
    email: "alice@gmail.com".to_string(),
});

// Create order
db.create_order(Order {
    id: 32,
    amount: 50.0,
}, "alice@gmail.com");

// Print order history
db.print_order("alice@gmail.com");


🔗 Level:
This task was equivalent to a production-style mini system with clear data flow and user interaction.
I feel confident in building logic-heavy CLI systems with Rust from scratch.

