use serde::{Serialize, Deserialize};
use super::super::block_type::BlockType;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub struct SceneHeading {
    pub block_type: BlockType,
    pub place: String,
    pub title: String,
    pub time: String,
}