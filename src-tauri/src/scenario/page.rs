use serde::{Deserialize, Serialize};
use crate::models::xml_struct::blocks::ScenarioBlock;

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PageDto {
    pub number: u32,
    pub blocks: Vec<ScenarioBlock>,
}