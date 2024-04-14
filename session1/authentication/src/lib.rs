use std::{collections::HashMap, path::Path};

use serde::{Deserialize, Serialize};

pub fn hash_password(password: &str) -> String {
    use sha2::Digest;
    let mut hasher = sha2::Sha256::new();
    hasher.update(password);
    format!("{:X}", hasher.finalize())
}

pub fn greet_user(name: &str) -> String {
    format!("Hello {name}")
}

pub fn read_line() -> String {
    let mut input: String = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Stdin not working");
    input.trim().to_string()
}

#[derive(PartialEq, Debug, Clone)]
pub enum LoginAction {
    Granted(LoginRole),
    Denied,
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub enum LoginRole {
    Admin,
    User,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub username: String,
    pub password: String,
    pub role: LoginRole,
}

impl User {
    pub fn new(username: &str, password: &str, role: LoginRole) -> User {
        User {
            username: username.to_lowercase(),
            password: hash_password(password),
            role,
        }
    }
}

// pub fn get_users() -> Vec<User> {
//     vec![
//         User::new("admin", "password", LoginRole::Admin),
//         User::new("dream", "password", LoginRole::User),
//     ]
// }

fn get_default_users() -> HashMap<String, User> {
    let mut user_map = HashMap::new();
    user_map.insert(
        "admin".to_string(),
        User::new("admin", "password", LoginRole::Admin),
    );

    user_map.insert(
        "dream".to_string(),
        User::new("dream", "password", LoginRole::User),
    );

    user_map
}

pub fn save_users(user_map: HashMap<String, User>) {
    let users_path = Path::new("users.json");
    let users_json = serde_json::to_string(&user_map).unwrap();
    std::fs::write(users_path, users_json).unwrap();
}

pub fn get_users() -> HashMap<String, User> {
    let users_path = Path::new("users.json");
    if users_path.exists() {
        // Load the file !
        let users_json = std::fs::read_to_string(users_path).unwrap();
        let user_map = serde_json::from_str(&users_json).unwrap();
        user_map
    } else {
        // Create a file and return it
        let user_map = get_default_users();
        let users_json = serde_json::to_string(&user_map).unwrap();
        std::fs::write(users_path, users_json).unwrap();
        user_map
    }
}

// fn get_admin_users() {
//     let users: Vec<String> = get_users()
//         .into_iter()
//         .filter(|u| u.role == LoginRole::Admin)
//         .map(|u| u.username)
//         .collect();
// }

pub fn login(username: &str, password: &str) -> Option<LoginAction> {
    let username = username.to_lowercase();
    let password = hash_password(password);

    let users = get_users();
    if let Some(user) = users.get(&username) {
        if user.password == password {
            return Some(LoginAction::Granted(user.role.clone()));
        } else {
            return Some(LoginAction::Denied);
        }
    }
    // if let Some(user) = users.iter().find(|user| user.username == username) {
    //     if user.password == password {
    //         return Some(LoginAction::Granted(user.role.clone()));
    //     } else {
    //         return Some(LoginAction::Denied);
    //     }
    // }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greet_user() {
        assert_eq!("Hello Dream", greet_user("Dream"))
    }

    #[test]
    fn test_login() {
        assert_eq!(login("ADMIN", "password"), LoginAction::Admin);
        assert_eq!(login("dream", "password"), LoginAction::User);
        // assert_eq!(login("admin-not", "password"), LoginAction::Denied);
        // assert_eq!(login("admin", "not-password"), LoginAction::Denied);
    }
}
