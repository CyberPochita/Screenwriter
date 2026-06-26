use serde::{Serialize, Deserialize};
use super::super::block_type::BlockType;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum TitleStyle {
    Center,
    Left,
    Bottom,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub struct Title {
    pub block_type: BlockType,
    pub text: String,
    pub style: Option<TitleStyle>,
    pub is_spoken: Option<bool>
}
