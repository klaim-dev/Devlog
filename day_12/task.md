                              📦 Super Task — Catalog Statistics Module

                                           🎯 Task

Implement a `stats` module for a book catalog that provides aggregated statistics, using iterators and avoiding unnecessary allocations.


# 📦 Day 12 — Super-task: Catalog Statistics Module

## Goal
Implement a `stats` module for a book catalog that provides aggregated statistics, using iterators and avoiding unnecessary allocations.

---

## Data Model
A book is represented as:

pub struct Book {
    pub title: String,
    pub genre: String,
    pub pages: u32,
    pub year: u32,
    pub in_stock: bool,
}
Functions to Implement
1. avg_pages_by_genre_sorted
* Input: &[Book]

* Output: Vec<(String, f64)> — vector of (genre, average pages) tuples.

* Genres should be sorted alphabetically.

* Average should be calculated as total_pages / count for each genre.

2. top_n_recent
* Input: &[Book], n: usize

* Output: Vec<Book> — the N most recent books.

* Sort by:

1. year descending

2. If years are equal, by pages descending.

* Return only the top N books.

3. percent_in_stock
* Input: &[Book]

* Output: f64 — percentage of books in stock.

* Formula: (number_in_stock as f64 / total_books as f64) * 100.0.
