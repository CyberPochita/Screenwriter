use serde::{Serialize, Deserialize};
use super::super::block_type::BlockType;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub struct ActionDescription {
    pub block_type: BlockType,
    pub text: String
}