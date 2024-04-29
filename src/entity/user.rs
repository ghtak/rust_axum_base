use serde::{Deserialize, Serialize};


#[derive(Debug, Default, Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
}

impl User {
    pub fn new(name: &'_ str, email: &'_ str) -> Self {
        User{
            id: i32::default(),
            name: name.to_owned(),
            email: email.to_owned()
        }
    }
}