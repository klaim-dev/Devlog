## 📅 Day 5 — References and Borrowing in Rust

### 🔍 Topic
Today I'm focused on one of Rust’s core safety features — **references** (`&T`) and **mutable references** (`&mut T`), as well as how the **borrow checker** ensures memory safety without a garbage collector.

---

### 📘 What I'm Learned

- `&T` is an immutable reference — allows read-only access without taking ownership.
- `&mut T` is a mutable reference — allows modifying data without moving ownership.
- You can have:
  - ✅ Many `&T` at the same time
  - ✅ One `&mut T` at a time
  - ❌ But never both at once
- References must never outlive the data they point to.
- Rust enforces this at **compile time**, eliminating common bugs like use-after-free and data races.

---

### 🧠 What I'm Practiced

- Writing functions that take `&str`, `&String`, `&mut String`
- Conditional logic with `.len()` and `.clear()`
- Modifying a struct field through `&mut`
- Building logic-heavy flows while respecting Rust’s memory model

---
