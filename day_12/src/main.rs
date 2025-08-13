use day_12::catalog;
use day_12::catalog::Catalog;
use day_12::model;
use day_12::model::Book;
use day_12::stats::{self, percent_in_stock, top_n_recent, avg_pages_by_genre};
fn main() {
    let books = vec![
    Book { id: 1, title: "The Rust Book".into(),       author: "Steve Klabnik".into(), year: 2019, pages: 550, genre: "Programming".into(), in_stock: true },
    Book { id: 2, title: "Programming in Rust".into(), author: "Jane Doe".into(),      year: 2021, pages: 320, genre: "Programming".into(), in_stock: false },
    Book { id: 3, title: "Rust for Pros".into(),       author: "Alex Smith".into(),    year: 2023, pages: 280, genre: "Programming".into(), in_stock: true },
    Book { id: 4, title: "Zero to Production".into(),  author: "Luca Palmieri".into(), year: 2022, pages: 420, genre: "Programming".into(), in_stock: true },
    Book { id: 5, title: "Rust Async Guide".into(),    author: "Async Dev".into(),     year: 2020, pages: 150, genre: "Programming".into(), in_stock: false },
    Book { id: 6, title: "Advanced Rust".into(),       author: "Expert Dev".into(),    year: 2023, pages: 610, genre: "Programming".into(), in_stock: true },
    Book { id: 7, title: "Rust by Example".into(),     author: "Rustacean".into(),     year: 2018, pages: 230, genre: "Programming".into(), in_stock: true },
    Book { id: 8, title: "Rust Web Dev".into(),        author: "Web Coder".into(),     year: 2021, pages: 300, genre: "Programming".into(), in_stock: true },
    Book { id: 9, title: "Unsafe Rust".into(),         author: "Low Level Dev".into(), year: 2022, pages: 190, genre: "Programming".into(), in_stock: false },
    Book { id: 10, title: "Rust Patterns".into(),      author: "Pattern Master".into(),year: 2023, pages: 340, genre: "Programming".into(), in_stock: true },

];

    let mut catalog = Catalog::new();
    let add_book = for b in books {
        let add = Catalog::add(&mut catalog, b.id, b.title, b.author, b.year, b.genre, b.pages, b.in_stock);
          match add {
       Ok(value) => println!("{}", value),
       Err(e) => println!("{}", e), 
    } 
   };

   let find_book = Catalog::find(&mut catalog, 1);
   match find_book {
       Ok(b) => println!("{}", b.title),
       Err(e) => println!("{}", e),
   };

   let update_pages = Catalog::update_pages(& mut catalog, 1, 521);
   if let Ok(update) = update_pages {
    println!("{}", update)
   };

   let available = Catalog::available(&catalog);
   match available {
       Ok(v) => {
        for book in v {
            println!("{}", book.id)
        };
       }
         Err(e) => println!("{}", e)
   }

   let filter_by = Catalog::filter_by(&catalog, "Language", 521);
   match filter_by {
       Ok(value) => {
        for b in &value {
            println!("{}", b.title)
        } 
       },
       Err(e) => println!("{}", e),
   };

   let by_years = Catalog::sorted_by_year_desc(&catalog);
   match by_years {
       Ok(books) => {
        for book in books {
            println!("{}, {}", book.year, book.title)
        }
       },
       Err(e) => println!("{}", e),
   };



   for (genry, avg) in avg_pages_by_genre(Catalog::all_books(&catalog)) {
    println!("{}, {} avg pages ", genry, avg);
   };

   println!("In stock: {}%", percent_in_stock(Catalog::all_books(&catalog)));
   println!("Top 3 recent book: ");
   for b in top_n_recent(Catalog::all_books(&catalog), 3) {
    println!("{},{},{}", b.title, b.year, b.pages)
   };




}
