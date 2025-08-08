use super::register::User;
pub struct Manager {
    pub users: Vec<User>,
}

impl Manager {
    pub fn new() -> Self {
        Manager{users: Vec::new()}
    }
}