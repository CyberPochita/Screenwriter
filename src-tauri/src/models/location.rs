use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename = "location")]
pub struct Location {
    pub name: String,
    pub type_scene: String,
    pub time_day: String,
    pub lighting: String,
    pub interior_details: String,
    pub description: String,
}