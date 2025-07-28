## 🧠 Rust Dev Log — Day 1: Strings, Ownership, and Memory

### 📅 Topic:
- `let`, `mut` and variable bindings
- `String` vs `&str`
- Stack, heap, and read-only memory
- Introduction to ownership and move semantics

---

### ✅ What I did today:
- Revisited the basics of variable binding and immutability in Rust
- Explored the difference between `String` and `&str`
- Practiced creating and manipulating strings with `.push_str()`
- Learned how and why ownership is transferred when assigning `String`
- Converted numeric types into strings using `.to_string()`
- Built a custom greeting message manually from components

---

### 🔬 Key Concepts I Learned:

#### 1. `&str` vs `String`

| Concept          | `&str`                            | `String`                                |
|------------------|-----------------------------------|------------------------------------------|
| Mutability       | Immutable                         | Mutable (if declared with `mut`)         |
| Memory location  | Read-only binary segment (`.rodata`) | Heap-allocated                          |
| Size             | Fixed                             | Growable                                |
| Ownership        | Borrowed reference (`Copy`)       | Owned type (`Move`)                     |

- `&str` is a string **slice** — just a pointer and length pointing to static data
- `String` is a **heap-allocated**, growable structure with full ownership over its data
- Copying a `&str` just copies the reference. Copying a `String` moves ownership.

---

#### 2. Stack vs Heap (Memory Model)
- The `String` struct (ptr, len, capacity) is stored on the **stack**
- The actual characters of the `String` are stored on the **heap**
- String literals (`&str`) are stored in **read-only memory** and cannot be mutated

```rust
let name = "Alice"; // &str — fixed in binary memory
let mut greeting = String::from("Hello, ");
greeting.push_str(name); // Works because push_str accepts &str

