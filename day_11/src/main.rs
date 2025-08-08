use crate::posts::{all_posts::{self, all_posts, print_all}, create_posts::create_posts};

mod users;
mod posts;
fn main() {
    let mut db = users::db::Manager::new();
    let mut db_posts = posts::db::Manager_posts::new();
    let add_user = users::register::add_user(&mut db, "Alice".to_string(), 123);
    match add_user {
        Ok(value) => println!("{}", value),
        Err(e) => println!("{}", e),
    };

    let login = users::login::login(&mut db, "Alice".to_string(), 123);
    match login {
        Ok(value) => println!("{}", value),
        Err(e) => println!("{}", e),
    };

    let create_posts = create_posts(&db, &mut db_posts, "Rust".to_string(), "ABCDIFG".to_string(), "Alice".to_string());
    match create_posts {
        Ok(value) => println!("{}", value),
        Err(e) => println!("{}", e),
    };

    let all_post_user = all_posts(&db_posts, "Alice".to_string());
    match all_post_user {
        Ok(()) => (),
        Err(e) => println!("{}", e),
    };

    let all = print_all(&db, &db_posts);
    match all {
        Ok(()) => (),
        Err(e) => println!("{}", e),
    };
   
}
