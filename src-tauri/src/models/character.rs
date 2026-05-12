use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Character {
    pub first_name: String,
    pub last_name: String,
    pub middle_name: String,
    pub age: u8,
    pub habits: String,
    pub likes: String,
    pub dislikes: String,
    pub description: String,
}