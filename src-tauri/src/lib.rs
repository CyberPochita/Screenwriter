pub mod models;
pub mod options;
pub mod scenario;
pub mod xml;
pub mod characters;
pub mod locations;

pub struct AppState {
    pub options: std::sync::Mutex<options::Options>,
}