## 🧠 Super Task — Task Manager System (Day 10)

### 🎯 Description

You are building a simple task management system. The system should support:

- Adding users
- Assigning tasks to users
- Marking tasks as completed
- Printing task status for a user

---

### 🧱 Data Structures

```rust
struct Task {
    id: u32,
    title: String,
    completed: bool,
}

struct User {
    name: String,
    tasks: Vec<Task>,
}

struct TaskManager {
    users: Vec<User>,
    next_id: u32,
}

