use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum Authorship {
    Original,
    Adaptation { source: String },
    BasedOn { source: String },
}