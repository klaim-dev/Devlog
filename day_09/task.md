## 🧠 Super Task — Book Borrowing System

### 🎯 Task

You were asked to build a simple book borrowing system with two core entities:

struct User {
    name: String,
    borrowed: Option<String>,
}

struct Book {
    title: String,
    author: String,
    available: bool,
}

📋 Functional Requirements
1. A User can borrow a Book if:

* the book is available

* the user has not borrowed any book yet

2. A User can return the book only if:

* the title matches the book they have borrowed

3. Statuses are printed via match on Result and Option

4. Output logic:

* println!("Access result: {}", status);

🧪 Implementation Summary
🔧 Methods Implemented:
* Book::borrow_book(&mut self, user: &mut User) -> Result<String, String>

* Book::return_book(&mut self, user: &mut User) -> Result<String, String>

* User::print_status(&self)


