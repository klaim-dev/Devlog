use std::collections::HashMap;
use std::collections::hash_map::Entry;
use day_13::analytics;
use day_13::store;
use day_13::models;
use day_13::models::Book;
use day_13::store::Store;

fn main() {

    let books:Vec<Book> = vec![
    Book { id: 1,  title: "The Rust Book".into(),          author: "Steve Klabnik".into(), year: 2019, pages: 550, genre: "Programming".into(), in_stock: true },
    Book { id: 2,  title: "Rust in Action".into(),         author: "Tim McNamara".into(),  year: 2021, pages: 400, genre: "Programming".into(), in_stock: true },
    Book { id: 3,  title: "Rust Patterns".into(),          author: "John Doe".into(),      year: 2023, pages: 340, genre: "Programming".into(), in_stock: false },
    Book { id: 4,  title: "Zero to Production".into(),     author: "Luca Palmieri".into(), year: 2022, pages: 420, genre: "Programming".into(), in_stock: true },
    Book { id: 5,  title: "Advanced Rust".into(),          author: "Jane Roe".into(),      year: 2023, pages: 610, genre: "Programming".into(), in_stock: true },
    Book { id: 6,  title: "Dune".into(),                   author: "Frank Herbert".into(), year: 1965, pages: 412, genre: "Fiction".into(),     in_stock: false },
    Book { id: 7,  title: "1984".into(),                   author: "George Orwell".into(), year: 1949, pages: 328, genre: "Fiction".into(),     in_stock: true },
    Book { id: 8,  title: "Brave New World".into(),        author: "Aldous Huxley".into(), year: 1932, pages: 311, genre: "Fiction".into(),     in_stock: false },
    Book { id: 9,  title: "The Hobbit".into(),             author: "J.R.R. Tolkien".into(),year: 1937, pages: 310, genre: "Fantasy".into(),     in_stock: true },
    Book { id: 10, title: "The Lord of the Rings".into(),  author: "J.R.R. Tolkien".into(),year: 1954, pages: 1178,genre: "Fantasy".into(),     in_stock: true },
    Book { id: 11, title: "Rust Web Development".into(),   author: "Web Coder".into(),     year: 2021, pages: 300, genre: "Programming".into(), in_stock: false },
    Book { id: 12, title: "The Pragmatic Programmer".into(),author: "Andy Hunt".into(),    year: 1999, pages: 352, genre: "Programming".into(), in_stock: true },
    Book { id: 13, title: "Clean Code".into(),             author: "Robert C. Martin".into(),year: 2008, pages: 464, genre: "Programming".into(),in_stock: true },
    Book { id: 14, title: "Harry Potter and the Sorcerer's Stone".into(), author: "J.K. Rowling".into(), year: 1997, pages: 309, genre: "Fantasy".into(), in_stock: true },
    Book { id: 15, title: "Harry Potter and the Chamber of Secrets".into(), author: "J.K. Rowling".into(), year: 1998, pages: 341, genre: "Fantasy".into(), in_stock: false },
    Book { id: 16, title: "Harry Potter and the Prisoner of Azkaban".into(), author: "J.K. Rowling".into(), year: 1999, pages: 435, genre: "Fantasy".into(), in_stock: true },
    Book { id: 17, title: "Harry Potter and the Goblet of Fire".into(), author: "J.K. Rowling".into(), year: 2000, pages: 636, genre: "Fantasy".into(), in_stock: true },
    Book { id: 18, title: "Rust Async Guide".into(),       author: "Async Dev".into(),     year: 2020, pages: 150, genre: "Programming".into(), in_stock: false },
    Book { id: 19, title: "Unsafe Rust".into(),            author: "Low Level Dev".into(),year: 2022, pages: 190, genre: "Programming".into(), in_stock: false },
    Book { id: 20, title: "The Great Gatsby".into(),       author: "F. Scott Fitzgerald".into(),year: 1925, pages: 218, genre: "Fiction".into(), in_stock: true },
];

let store1 = store::Store::new();
let mut create_store = match store::Store::from_items(books) {
    Ok(s) => s,
    Err(e) => {println!("{}", e); return;}
};

match store::Store::add(&mut create_store, Book { id: 21, title: "Rust Web Development".into(),   author: "Web Coder".into(),     year: 2021, pages: 300, genre: "Programming".into(), in_stock: false }){
    Ok(_) => println!("Book add"),
    Err(e) => println!("{}", e),
};

match Store::get(&mut create_store, 15) {
    Some(val) => println!("{}", val.title),
    None => println!("Bokk not found"),
};

let books_by_genry = Store::by_genre(&create_store, "Programmig");
    for book in books_by_genry {
        println!("{},{},{}", book.id, book.title, book.in_stock);
    };

    let remove_book = Store::remove(&mut create_store, 19);
}




