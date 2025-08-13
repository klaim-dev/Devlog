use std::collections::HashMap;
use crate::model::Book;
use crate::catalog::Catalog;

pub fn avg_pages_by_genre<'a, I>(books: I) -> Vec<(String, f64)> 
where
    I: IntoIterator<Item = &'a Book>,
    {

    let mut stats: HashMap<&'a str, (u32, u32)> = HashMap::new();

    for book in books {
        let(sum, cnt) = stats.entry(book.genre.as_str()).or_insert((0, 0));
        *sum += book.pages;
        *cnt += 1;

    };

    let mut out:Vec<(String, f64)> = stats
    .into_iter()
    .map(|(g, (sum, cnt))| (g.to_string(), sum as f64 / cnt as f64))
    .collect();

    out.sort_by(|(g1, _), (g2, _)| g1.cmp(g2));
    out
    
    }

    pub fn top_n_recent<'a, I>(books: I, n: usize) -> Vec<&'a Book>
where
    I: IntoIterator<Item = &'a Book>,
    {
        let mut book_by_year:Vec<&'a Book> = books.into_iter().collect();
        book_by_year.sort_by(|a, b| {
            b.year
            .cmp(&a.year)
            .then_with(|| b.pages.cmp(&a.pages))
        });
        book_by_year.truncate(n);
        book_by_year
    }


    pub fn percent_in_stock<'a, I>(books: I) -> f64
where
    I: IntoIterator<Item = &'a Book>,
    {

        let mut total = 0usize;
        let mut have = 0usize;

        for b in books {
            total += 1;
            if b.in_stock {have += 1;}
        }

        if total == 0 {
            0.0
        } else {
            have as f64
        }

    }

