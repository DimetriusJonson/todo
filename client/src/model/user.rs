use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Serialize, PartialEq, Eq, Deserialize)]
pub struct User {
    pub id: Option<u64>,
    pub username: String,
    #[serde(skip)]
    pub password: Option<String>,
    pub token: Option<String>,
}

impl User {
    pub fn set_username(&mut self, value: String) {
        self.username = value;
    }

    pub fn set_password(&mut self, value: String) {
        self.password = Some(value);
    }

    pub fn is_valid(&self) -> bool {
        self.username.len() >= 3 && self.password.to_owned().unwrap_or_default().len() >= 4
    }
}

impl Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.username)
    }
}
