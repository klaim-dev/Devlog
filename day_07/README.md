## 📘 DevLog — Day 7: `Result<T, E>`, Safe Error Handling in Rust

### 📅 Topic
Advanced error handling using `Result<T, E>`, with full pattern matching, `.map_err()`, `?`, and safe architecture.

---

### ✅ What I Learned

- `Result<T, E>` is Rust’s way to express fallible operations — used in I/O, DB calls, parsing, etc.
- `unwrap()` will panic on error — should be avoided in real code
- `expect("msg")` adds custom panic message but still unsafe
- The `?` operator allows elegant early return of `Err`, propagating errors upward
- `.map_err()` converts low-level errors into higher-level domain-specific ones
- Nested results (`Result<Option<T>>`) require careful pattern matching
- Clear error messages help separate logic from failure causes

---

### 🧩 Mini Tasks Completed

1. **Safe Division**
   ```rust
   fn safe_divide(a: i32, b: i32) -> Result<i32, String>

2. Parsing + Adding Two Strings
Used ?, .parse::<i32>(), .map_err() to return custom error.

3. Result<Option<T>> Pattern Matching
Transformed Result<Option<String>> into a clean Result<String, String> interface.
