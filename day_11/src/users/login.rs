use super::db::Manager;

pub fn login(db: &mut Manager, name: String, password: u32) -> Result<String, String> {
    let mut user= db
    .users
    .iter_mut()
    .find(|u| u.name == name)
    .ok_or_else(|| "User not found".to_string())?;

    if user.password == password {
        if user.active == false {
            user.active = true;
            Ok("User login".to_string())
        } else {
            Err("User allready logged".to_string())
        }
    } else {
        Err("Password wrong".to_string())
    }
 }
