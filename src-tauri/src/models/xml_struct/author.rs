use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Author {
    #[serde(rename = "$value")]
    pub text: String,
    #[serde(default = "default_normal_style")]
    pub style: String,
}

fn default_normal_style() -> String { "NORMAL_CASE".to_string() }