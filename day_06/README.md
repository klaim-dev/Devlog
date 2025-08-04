## 📘 DevLog — Day 6: Option, match, if let

### 📌 Topic: `Option<T>`, Pattern Matching, Safe Unwrapping

Today I practiced deep handling of optional values using `Option<T>` in Rust.  
The key focus was **avoiding unsafe `.unwrap()`** and thinking in terms of **logic trees** instead of flat conditionals.

---

### ✅ What I Learned

- `Option<T>` is Rust’s way to express “maybe” without null
- Pattern matching with `match` gives full control over `Some`/`None`
- `if let` is a concise tool when only one arm (`Some`) matters
- Methods like `.map()` let you transform data safely inside the `Option`

---

### 🛠 Super Task: User Email System

Built a safe backend logic to send emails:

- If inactive → reject immediately
- If no email → reject
- Otherwise → confirm email sent

**Result:** Clear, readable, memory-safe code without panic risks.

---
