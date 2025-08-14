#[derive(Debug, Clone)]
pub struct Book {
    pub id: u32,
    pub title: String,
    pub author: String,
    pub year: u32,
    pub genre: String,
    pub pages: u32,
    pub in_stock: bool,
}
