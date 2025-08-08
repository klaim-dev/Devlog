use super::create_posts::Posts;
pub struct Manager_posts {
    pub posts: Vec<Posts>
}

impl Manager_posts {
    pub fn new() -> Self {
        Manager_posts { posts: Vec::new() }
    }
}