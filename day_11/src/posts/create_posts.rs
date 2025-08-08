use super::db::Manager_posts;
use crate::users::db::Manager;

pub struct Posts {
    pub headline: String,
    pub text: String,
    pub name: String,
}

pub fn create_posts(db_user: &Manager, db_posts: &mut Manager_posts, headline: String, text: String, name: String) -> Result<String, String> {
    
    let post = Posts {
        headline,
        text,
        name,
    };

    let user = db_user
    .users
    .iter()
    .find(|u| u.name == post.name)
    .ok_or_else(|| "User not found")?;

    if user.active {
        db_posts.posts.push(post);
        Ok("Post create".to_string())
    } else {
        Err("User not active".to_string())
    }

}
  