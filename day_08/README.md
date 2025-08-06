## 📅 Day 8 — Final Challenge: Ownership, Option, Result, Architecture

### 🔧 Task: Order & User Management System

Today I built a full mini-system in Rust that simulates user registration and order tracking.

#### 🧠 Core Concepts Covered:
- Ownership and borrowing (`move`, `&`, `&mut`)
- Error handling with `Result<T, E>` and `Option<T>`
- Data structures: `Vec<T>`, `HashMap<K, V>`, and nested models
- Methods and struct architecture (`impl`, `self`, `&self`)
- Pattern matching: `match`, `if let`, `.map()`, `.get()`, `.entry()`

✅ Result:
Built a working simulation of a real-world system with correct logic, safety guarantees, and structure.
Handled non-trivial architecture and validation logic cleanly.

📈 Takeaways:
* Ownership rules now feel intuitive — I know when values move and when I need a reference.

* I'm confidently using Result and Option in real code, not just toy examples.

* I built a system, not just separate functions. Every method works together, handles state, and returns feedback.
