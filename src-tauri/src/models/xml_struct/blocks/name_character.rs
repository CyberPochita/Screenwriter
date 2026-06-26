use serde::{Serialize, Deserialize};
use super::super::block_type::BlockType;
use super::super::super::character::Character;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub struct NameCharacter {
    pub block_type: BlockType,
    pub text: String
}