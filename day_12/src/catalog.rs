use crate::model::Book;
pub struct Catalog {
    pub items: Vec<Book>,
}
impl Catalog {
    pub fn new() -> Self {
        Catalog { items: Vec::new() }
    }

     pub fn all_books(&self) -> impl Iterator<Item = &Book> {
        self.items.iter()
    }

    pub fn add(&mut self, id: u32, title: String, author: String, year: u16, genre: String, pages: u32, in_stock: bool) -> Result<String, String> {
        let book = Book { id, title, author, year, genre, pages, in_stock, };
        if self
        .items
        .iter()
        .any(|id| id.id == book.id) {
            Err("Book allready exsists".to_string())
        } else {
            self.items.push(book);
            Ok("Book add".to_string())
        }
    }

    pub fn find(&mut self, id: u32) -> Result<&mut Book, String> {
        let book = self
        .items
        .iter_mut()
        .find(|book| book.id == id)
        .ok_or_else(|| "Book not found".to_string())?;
    Ok(book)
    }

    pub fn update_pages(&mut self, id: u32, pages: u32) -> Result<String, String> {
        match self.find(id) {
            Ok(book) => {
                book.pages = pages;
                Ok(format!("Pages in a book with id : {}, update", id))
            }
            Err(e) => Err(e),
        }
    }

    pub fn available(&self) -> Result<Vec<&Book>, String> {
        let available: Vec<&Book> = self
        .items
        .iter()
        .filter(|b| b.in_stock == true)
        .collect();
    if available.is_empty() {
        Err("Not found available books".to_string())
    } else {
        Ok(available)
    }
    }

    pub fn filter_by(&self, genre: &str, min_pages: u32) -> Result<Vec<Book>, String> {
        let filter:Vec<Book> = self
        .items
        .iter()
        .filter(|b| b.genre == genre && b.pages == min_pages)
        .map(|b| b.clone())
        .collect();

    if filter.is_empty() {
        Err("Not found the books with your filters".to_string())
    } else {
        Ok(filter)
    } 
    }

    pub fn sorted_by_year_desc(&self) -> Result<Vec<&Book>, String> {
        let mut sorted: Vec<&Book> = self
        .items
        .iter()
        .collect();
    if sorted.is_empty() {
        Err("No books in struct".to_string())
    } else {
    sorted.sort_by_key(|book| std::cmp::Reverse(book.year));
    Ok(sorted)
    }
    }


}