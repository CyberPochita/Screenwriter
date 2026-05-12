// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::fs::{self, File};
use std::path::Path;

#[tauri::command]
async fn create_file(name: String) -> Result<String, String> {
  File::create(format!(r"C:\Users\Grusha\Desktop\scenarios\{}.md", name)).map_err(|e| e.to_string())?;
  Ok("File created".to_string())
}

#[tauri::command]
async fn write_to_file(msg: String, file: String) -> Result<String, String> {
  fs::write(format!(r"C:\Users\Grusha\Desktop\scenarios\{}", file), msg.as_bytes()).map_err(|e| e.to_string())?;
  Ok("File written".to_string())
}

#[tauri::command]
async fn read_file(name_file: String) -> Result<String, String> {
  println!("Reading file: {}", name_file);
  let content = fs::read_to_string(format!(r"C:\Users\Grusha\Desktop\scenarios\{}", name_file))
    .map_err(|e| e.to_string())?;

  Ok(content)
}

#[tauri::command]
async fn get_scenarios() -> Result<Vec<String>, String> {
  let path = Path::new(r"C:\Users\Grusha\Desktop\scenarios");
  
  let entries = fs::read_dir(path).map_err(|e| e.to_string())?;
  let mut scenarios = Vec::new();

  for entry in entries {
    let entry = entry.map_err(|e| e.to_string())?;
    
    if let Ok(name) = entry.file_name().into_string() {
      scenarios.push(name);
    }
  }

  Ok(scenarios)
}


fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![create_file, write_to_file, get_scenarios, read_file])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
