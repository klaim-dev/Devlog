
use super::db::Manager;
pub struct User{
    pub name: String,
    pub password: u32,
    pub active: bool,
}

pub fn add_user(db: &mut Manager, name: String, password: u32) -> Result<String, String> {
    let user = User {
        name,
        password,
        active: false,
    };

    if db.users.iter().any(|u| u.name == user.name) {
        Err("User allready registerd".to_string())
    } else {
        db.users.push(user);
        Ok("User add to DB".to_string())
    }
}