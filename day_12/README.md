# 📓 DevLog — Day 12

**Date:** August 13, 2025  
**Topic:** `Vec<T>` and basic iterators (`iter`, `into_iter`, `map`, `filter_map`)  
**Artifacts:**  
- `day12_vec_iter.rs` (Level 1–2)  
- `day12_catalog/` (super-task: `stats` module)  

---

## 🧠 What I Learned
- **Vec structure in memory:** `len`, `capacity`, and a pointer are stored on the stack, while the elements are stored in the heap.
- **Capacity growth:** when full, Vec allocates new memory (usually ×2).
- **`iter` vs `into_iter`:**
  - `iter()` — yields `&T`, does not take ownership.
  - `into_iter()` — yields `T`, takes ownership, and the original Vec is consumed.
- **`map` and `filter_map`:**
  - `map` — always returns the same number of elements.
  - `filter_map` — allows filtering out `None` values and transforming `Some` into a new value.
- **Iterator chains:** allow processing data without intermediate collections (lazy until `collect()`).

---

## 🔨 What I Did
**Level 1 (micro):**
- Created vectors using `.collect()` and `Vec::new()`.
- Checked capacity growth during `push`.
- Used `iter()` and `into_iter()` to iterate and print values.

**Level 2 (mini):**
- Implemented filtering and transforming even numbers into their squares.
- Filtered words longer than 3 characters, transformed to uppercase, and sorted.

**Super-task:**
- Created a `stats` module for a book catalog:
  - `avg_pages_by_genre_sorted` — average page count per genre.
  - `top_n_recent` — N most recent books (sorted by year, then pages).
  - `percent_in_stock` — percentage of books in stock.
- Separated into `model` and `stats` modules with pure functions and no unwraps.

---

## ✅ Results
- Code compiles with no unwraps in the production path.
- All super-task functions work correctly (tests will be added on Day 13).
- Confident with `map` and `filter_map` chaining.

---

## 📌 To Improve
- Remove unnecessary `.clone()` when using `to_uppercase` (already returns a `String`).
- In `stats`, consider using `BTreeMap` for genre sorting instead of manual sorting.
- Add unit tests to verify module correctness.

---

## ⏭ Next Step
Tomorrow (Day 13) — `HashMap<K, V>` and `.entry()` patterns, plus adding tests for today’s `stats` module.

