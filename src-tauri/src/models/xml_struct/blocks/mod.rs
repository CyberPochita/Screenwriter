pub mod title;
pub mod scene_heading;
pub mod parenthetical;
pub mod name_character;
pub mod character_dialog;
pub mod action_description;

pub use action_description::ActionDescription;
pub use character_dialog::CharacterDialog;
pub use name_character::NameCharacter;
pub use parenthetical::Parenthetical;
pub use scene_heading::SceneHeading;
pub use title::Title;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum ScenarioBlock {
    ActionDescription(ActionDescription),
    Dialog(CharacterDialog),
    NameCharacter(NameCharacter),
    Parenthetical(Parenthetical),
    SceneHeading(SceneHeading),
    Title(Title),
}