use serde::{Deserialize, Serialize};
use std::fs;
use std::error::Error;
use std::fmt::Debug;
use std::path::Path;

#[derive(Debug, Deserialize, Clone)]
pub struct MainConfig {
    pub db_path: String,
    pub registry_path: String,
    pub user_path: String,
    pub key_path: String,
    pub api_port: u16,
    pub mode: String, // "dev", "prod", "test"...
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct User {
    pub id: String,
    pub name: String,
    pub email: String,
    pub password_hash: String,
    pub role: String, // "admin", "user", "guest"...
    pub init: bool, // true if the user is initialized
}

impl User {
    // Create a new user
    pub fn new(
        id: String,
        name: String,
        email: String,
        password_hash: String,
        role: String,
        user_path: impl AsRef<Path>,
        default: bool,
    ) -> Result<Self, Box<dyn Error>> {
        // create a user.toml file
        let user = User {
            id: id.clone(),
            name: name.clone(),
            email: email.clone(),
            password_hash: password_hash.clone(),
            role: role.clone(),
            init: !default,
        };
        let user_toml = toml::to_string(&user).unwrap();
        fs::write(&user_path, user_toml).expect("Unable to write file");
        Ok(Self {
            id,
            name,
            email,
            password_hash,
            role,
            init: !default,
        })
    }
    pub fn load_from_file(path: impl AsRef<Path>) -> Result<Self, Box<dyn Error>> {
        let raw = fs::read_to_string(path)?;
        let user: User = toml::from_str(&raw)?;
        Ok(user)
    }
}

impl MainConfig {
    pub fn load() -> Result<Self, Box<dyn Error>> {
        let raw = fs::read_to_string("config.toml")?;
        let config: MainConfig = toml::from_str(&raw)?;
        Ok(config)
    }
}
