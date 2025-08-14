use crate::models::Book;
use std::collections::{HashMap, hash_map::Entry};

pub struct Store {
    items: Vec<Book>,
    by_id: HashMap<u32, usize>,            // id -> index в items
    by_genre: HashMap<String, Vec<usize>>, // genre -> индексы в items
}

impl Store {
    pub fn new() -> Self {
        Store { items: Vec::new(), by_id: HashMap::new(), by_genre: HashMap::new() }
     }

    pub fn from_items(items_vec: Vec<Book>) -> Result<Self, String> {
        let mut store = Store {
             items: vec![],
             by_id: HashMap::new(),
             by_genre: HashMap::new(),
        };

        for book in items_vec {
            if store.items.iter().any(|b| b.id == book.id) {
                return Err("Id: {} allready exsists".to_string());
            } else {
                store.items.push(book);
            }
        };

        for (idx ,book) in store.items.iter().enumerate() {
            match store.by_id.entry(book.id){
                Entry::Occupied(_) => return Err(format!("Duplicate id : {}", book.id)),
                Entry::Vacant(id) => {
                    id.insert(idx);
                }
            };
            match store.by_genre.entry(book.genre.to_owned()) {
                Entry::Occupied(mut e) => {e.get_mut().push(idx);},
                Entry::Vacant(v) => {v.insert(vec![idx]);},
            };
                
        };
        Ok(store)

    }


    pub fn add(&mut self, book: Book ) -> Result<(), String> {
        let idx = self.items.len();
        match self.by_id.entry(book.id) {
                Entry::Occupied(_) => return Err("Book id allready exsists".to_string()),
                Entry::Vacant(v) => {v.insert(idx);},
    }

    self.by_genre
    .entry(book.genre.clone())
    .or_default()
    .push(idx);

    self.items.push(book);

    Ok(())
}

    pub fn get(&self, id: u32) -> Option<&Book> {
        let book = self.by_id
        .get(&id)
        .and_then(|&i| self.items.get(i));
    book

}

pub fn by_genre(&self, genre: &str) -> Vec<&Book> {
    match self.by_genre.get(genre) {
        Some(x) => x.iter().filter_map(|&i| self.items.get(i)).collect(),
        None => Vec::new(),
    }
    }

    pub fn remove(&mut self, id: u32) -> Option<Book> {
        let idx = self.by_id.get(&id)?;
        let removed = self.items.remove(*idx);

        self.by_genre.clear();
        self.by_id.clear();

        for(i, b) in self.items.iter().enumerate() {
            self.by_id.insert(b.id, i);
            self.by_genre.entry(b.genre.clone()).or_default().push(i);
        }
        Some(removed)



    }




}
