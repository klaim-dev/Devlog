#[derive(Debug, Clone, PartialEq)]
pub struct Book {
    pub id:        u32,
    pub title:     String,
    pub author:    String,
    pub year:      u16,
    pub genre:     String,   // без enum сегодня
    pub pages:     u32,
    pub in_stock:  bool,
}
