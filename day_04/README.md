## 📅 Day 4 — Ownership in Rust (Move, Copy, Clone)

### 🧠 Focus
Today’s focus was on one of Rust’s core concepts — **Ownership**.  
We explored how values move, when they get copied, and how to manage memory safely **without cloning**.

---

### ✅ What We Covered

| Concept     | Status | Notes |
|-------------|--------|-------|
| `move`      | ✅     | Ownership transfer for `String` and other heap data |
| `copy`      | ✅     | `i32`, `bool`, and other `Copy` types stay usable after assignment |
| `clone()`   | ✅     | Deep copy of heap data — used explicitly only when needed |
| `&str` vs `String` | ✅ | Used `.as_str()` to compare owned strings with literals safely |

---

### 🧪 Mini Tasks Completed

- Moved and cloned `String` values properly
- Fixed borrow-checker violations with mutable/immutable references
- Differentiated between move and copy types in practice
- Explained what happens in memory (stack/heap) during assignments

---
