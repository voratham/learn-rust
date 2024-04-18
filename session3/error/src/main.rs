use anyhow::Error;
use serde::Deserialize;
use std::path::Path;
use thiserror::Error;

#[derive(Debug, Error)]
enum UserError {
    #[error("No users found")]
    NoUsers,
    #[error("Too many users were found")]
    ToManyUsers,
}

fn maybe_read_a_file() -> Result<String, std::io::Error> {
    let my_file = Path::new("myfile.txt");
    std::fs::read_to_string(my_file)
}

fn file_to_uppercase() -> Result<String, std::io::Error> {
    let contents = maybe_read_a_file()?;
    Ok(contents.to_uppercase())
}

#[derive(Deserialize)]
struct User {
    name: String,
    password: String,
    // role: String,
}

fn load_users_json() -> anyhow::Result<Vec<User>> {
    let my_file = Path::new("users.json");
    let raw_text = std::fs::read_to_string(my_file)?;
    let users: Vec<User> = serde_json::from_str(&raw_text)?;
    Ok(users)
}

// #[allow(dead_code)]
// fn anyhow_load_users2() -> anyhow::Result<Vec<User>> {
//     let my_file = Path::new("users.json");
//     let raw_text = std::fs::read_to_string(my_file)?;
//     // println!("raw_text {raw_text:?}");
//     let users: Vec<User> = serde_json::from_str(&raw_text)?;
//     if users.is_empty() {
//         anyhow::bail!("No users found");
//     }
//     if users.len() > 2 {
//         return Err(anyhow::Error::msg("Too many users"));
//     }
//     Ok(users)
// }

type GenericResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

// fn load_users() -> anyhow::Result<Vec<User>> {
//     let my_path = Path::new("users.json");
//     let raw_text = std::fs::read_to_string(my_path)?;
//     let users: Vec<User> = serde_json::from_str(&raw_text);
//     anyhow::bail!("Oh no! We can't go on!");
//     Ok(users)
// }

fn load_users() -> Result<Vec<User>, UserError> {
    let my_path = Path::new("users.json");

    let raw_text = std::fs::read_to_string(my_path).map_err(|_| UserError::NoUsers)?;

    let users: Vec<User> = serde_json::from_str(&raw_text).map_err(|_| UserError::NoUsers)?;

    if users.len() > 10 {
        // anyhow::bail!("Oh no! We can't go on!");
        return Err(UserError::ToManyUsers);
    }

    Ok(users)
}

fn main() {
    let users = load_users();
    match users {
        Ok(users) => {
            for user in users {
                println!("User: {}, {}", user.name, user.password);
            }
        }
        Err(err) => {
            println!("🔥Error: {err}");
        }
    }
    // let _ = file_to_uppercase()
    // if let Ok(content) = file_to_uppercase() {
    //     println!("Contents: {content}")
    // }

    // let my_file = Path::new("myfile.text");
    // let content = std::fs::read_to_string(my_file);
    // match content {
    //     Ok(contents) => println!("{contents}"),
    //     // Err(e) => println!("ERROR: {e:#?}"),
    //     Err(e) => match e.kind() {
    //         std::io::ErrorKind::NotFound => println!("File not found - myfile.txt"),
    //         _ => println!("Error: {e:#?}"),
    //     },
    // }
}
