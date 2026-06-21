use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Title {
    #[serde(rename = "$value")] // Запишет строку прямо внутрь тега <title>
    pub text: String,
    #[serde(default = "default_style")]
    pub style: String,
}

fn default_style() -> String { "ALL_CAPS".to_string() }