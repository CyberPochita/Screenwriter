use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Formatting {
    pub top_margin: u32,
    pub left_margin: f32,
    pub right_margin: f32,
    pub contact_left_margin: f32,
}

impl Default for Formatting {
    fn default() -> Self {
        Self {
            top_margin: 14,
            left_margin: 3.25,
            right_margin: 3.25,
            contact_left_margin: 8.25,
        }
    }
}