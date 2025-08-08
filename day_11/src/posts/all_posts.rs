use super::db::Manager_posts;
use crate::users::db::Manager;

pub fn all_posts(db_posts: &Manager_posts, name: String) -> Result<(), String> {
    if db_posts.posts.iter().any(|p| p.name == name) {
        println!("All post by {}", name);
        for p in db_posts.posts.iter().filter(|p| p.name == name) {
            println!("Headline:  {}, Text: {}, by {}", p.headline, p.text, p.name)
        }
        return Ok(());
    } else {
        return Err("Posts not found".to_string());
    }
}

    pub fn print_all(db_user: &Manager, db_posts: &Manager_posts) -> Result<(), String> {
        if !db_user.users.is_empty() {
            if !db_posts.posts.is_empty() {
                println!("All user and Posts");
                for u in db_user.users.iter() {
                    let user = u;
                    for posts in db_posts.posts.iter().filter(|p| p.name == user.name) {
                        println!("User: {}, post: {}", user.name, posts.headline);
                    }
                }
                return Ok(());
            } else {
                return Err("User_db is emty".to_string());
            }

        } else {
            return Err("Posts_db is empty".to_string());
        }
    }
