// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::fs::{self, File};
use std::io::Write;
use std::io::Read;
use app_lib::AppState;
use app_lib::options::Options;
use app_lib::scenario::commands_scenario;
use app_lib::characters::commands_characters;
use app_lib::locations::commands_location;
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
                    fs::create_dir_all(&existing_options.locations_dir).ok(); // 🌟 ДОБАВЛЕНО
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

    // 🌟 ДОБАВЛЕНО: Автоматическое создание папки locations при первом запуске
    let locations_path = local_path.join("locations");
    fs::create_dir_all(&locations_path).ok();

    let default_options = Options {
        main_dir: local_path,
        scenarios_dir: scenarios_path.clone(),
        characters_dir: characters_path,
        locations_dir: locations_path, // 🌟 ДОБАВЛЕНО
        current_dir: scenarios_path,
    };

    if let Ok(mut config_file) = File::create(&config_file_path) {
        let json = serde_json::to_string_pretty(&default_options).unwrap();
        let _ = config_file.write_all(json.as_bytes());
    }

    default_options
}

#[tauri::command]
async fn get_app_options(state: tauri::State<'_, AppState>) -> Result<Options, String> {
    let opts = state.options.lock().map_err(|_| "Ошибка доступа к настройкам".to_string())?;
    Ok(opts.clone())
}

#[tauri::command]
async fn save_app_options(new_opts: Options, state: tauri::State<'_, AppState>) -> Result<String, String> {
    let mut opts = state.options.lock().map_err(|_| "Ошибка доступа к настройкам".to_string())?;
    
    // Проверяем и создаем все директории, включая новую локаций
    fs::create_dir_all(&new_opts.scenarios_dir).map_err(|e| e.to_string())?;
    fs::create_dir_all(&new_opts.characters_dir).map_err(|e| e.to_string())?;
    fs::create_dir_all(&new_opts.locations_dir).map_err(|e| e.to_string())?; // 🌟 ДОБАВЛЕНО

    *opts = new_opts.clone();

    let mut local_path = dirs_next::data_local_dir().ok_or("Не удалось найти локальную директорию")?;
    local_path.push("ScreenwriterCFGS");
    let config_file_path = local_path.join("configs").join("config.json");

    let mut config_file = File::create(&config_file_path).map_err(|e| e.to_string())?;
    let json = serde_json::to_string_pretty(&new_opts).map_err(|e| e.to_string())?;
    config_file.write_all(json.as_bytes()).map_err(|e| e.to_string())?;

    Ok("Настройки успешно применены и сохранены".to_string())
}

fn main() {
  tauri::Builder::default()
    .plugin(tauri_plugin_dialog::init()) 
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
      commands_characters::get_characters, commands_characters::create_character, commands_characters::write_to_character, commands_characters::read_character, commands_characters::read_character_by_name, commands_characters::delete_character_file,
      commands_location::get_locations, commands_location::create_location, commands_location::read_location, commands_location::write_to_location, commands_location::read_location_by_name, commands_location::delete_location_file,
      save_app_options, get_app_options,
      ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
