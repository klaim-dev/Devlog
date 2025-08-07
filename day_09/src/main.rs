use std::{fmt::format, ops::DerefMut};

fn main() {
    let mut user = User {
        name: "Alice".to_string(),
        borrowed: None,
    };

    let mut  book = Book {
        title: "Rust".to_string(),
        author: "Alice".to_string(),
        available: true,
    };

    let borrow = book.borrow_book(&mut user);
    match borrow {
        Ok(value) => println!("{}", value ),
        Err(e) => println!("{}", e),
    };

    let return_book = book.return_book(&mut user);
    match return_book {
        Ok(value) => println!("{}", value),
        Err(e) =>  println!("{}", e),
    };

    let print = user.print_status();

}

struct Book {
    title: String,
    author: String,
    available: bool,
}
struct User {
    name: String,
    borrowed: Option<String>,
}

impl User {
    fn print_status(&self) {
        match &self.borrowed {
            Some(value) => println!("{}" , value),
            None => println!("No borrowed book"),
        };
    }
}

impl Book {
    fn borrow_book(&mut self, user: &mut User) -> Result<String, String> {
        if user.borrowed.is_some() {
            Err("User allready borrowed book".to_string())
        } else {
            user.borrowed = Some(self.title.to_string());
            self.available = false;
            Ok(format!("Book borrow"))
        }
    }

    fn return_book(&mut self, user: &mut User ) -> Result<String, String> {
        if user.borrowed.as_ref() == Some(&self.title) {
            user.borrowed = None;
            self.available = true;
            Ok("User return book".to_string())
        } else {
            Err("No book return".to_string())
        }
    }
}

