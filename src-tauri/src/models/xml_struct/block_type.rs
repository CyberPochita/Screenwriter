use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum BlockType {
    ActionDescription,
    Dialog,
    NameCharacter,
    Parenthetical,
    SceneHeading,
    Title
}