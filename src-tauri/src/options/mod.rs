use std::path::PathBuf;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Options {
    pub main_dir: PathBuf,
    pub scenarios_dir: PathBuf,
    pub characters_dir: PathBuf,
    pub locations_dir: PathBuf,
    pub current_dir: PathBuf
}