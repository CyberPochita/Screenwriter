use serde::{Serialize, Deserialize};
use super::super::block_type::BlockType;
use super::super::super::character::Character;
use super::parenthetical::Parenthetical;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub struct CharacterDialog {
    pub block_type: BlockType,
    pub text: String,
}