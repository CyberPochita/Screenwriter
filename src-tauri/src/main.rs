// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::fs::{self, File};
use std::io::Write;
use std::io::Read;
use app_lib::AppState;
use app_lib::options::Options;
use app_lib::scenario::commands_scenario;
use app_lib::characters::commands_characters;
use std::sync::Mutex;
use tauri::Manager;

fn init_configuration() -> Options {
    let mut local_path = dirs_next::data_local_dir()
        .expect("Не удалось получить локальную директорию данных");
    local_path.push("ScreenwriterCFGS");
    
    let configs_path = local_path.join("configs");
    let config_file_path = configs_path.join("config.json");

    if config_file_path.exists() {
        if let Ok(mut file) = File::open(&config_file_path) {
            let mut contents = String::new();
            if file.read_to_string(&mut contents).is_ok() {
                if let Ok(existing_options) = serde_json::from_str::<Options>(&contents) {
                    fs::create_dir_all(&existing_options.scenarios_dir).ok();
                    fs::create_dir_all(&existing_options.characters_dir).ok();
                    return existing_options;
                }
            }
        }
    }

    fs::create_dir_all(&local_path).ok();
    fs::create_dir_all(&configs_path).ok();

    let scenarios_path = local_path.join("scenarios");
    fs::create_dir_all(&scenarios_path).ok();

    let characters_path = local_path.join("characters");
    fs::create_dir_all(&characters_path).ok();

    let default_options = Options {
        main_dir: local_path,
        scenarios_dir: scenarios_path.clone(),
        characters_dir: characters_path,
        current_dir: scenarios_path,
    };

    if let Ok(mut config_file) = File::create(&config_file_path) {
        let json = serde_json::to_string_pretty(&default_options).unwrap();
        let _ = config_file.write_all(json.as_bytes());
    }

    default_options
}

fn main() {
  tauri::Builder::default()
    .setup(|app| {
      if cfg!(debug_assertions) {
        app.handle().plugin(
          tauri_plugin_log::Builder::default()
            .level(log::LevelFilter::Info)
            .build(),
        )?;
      }
      let app_options = init_configuration();
      app.manage(AppState {
                options: Mutex::new(app_options),
      });

      Ok(())
    })
    .invoke_handler(tauri::generate_handler![
      commands_scenario::enter_project, commands_scenario::exit_project, commands_scenario::return_dir,
      commands_scenario::create_file, commands_scenario::delete_file, commands_scenario::write_to_file, commands_scenario::entry_file, commands_scenario::get_files,
      commands_characters::get_characters, commands_characters::create_character, commands_characters::write_to_character, commands_characters::read_character
      ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
