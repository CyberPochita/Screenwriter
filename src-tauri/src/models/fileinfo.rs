use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct FileInfo {
    pub file_name: String,
    pub file_format: String
}