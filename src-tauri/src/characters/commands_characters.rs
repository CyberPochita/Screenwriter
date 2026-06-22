use std::fs::{self, File};
use std::path::Path;
use tauri::State;
use quick_xml::se::to_string as to_xml_string;
use quick_xml::de::from_str as from_xml_str;
use crate::AppState;
use crate::models::character::Character;

#[tauri::command]
pub async fn get_characters(state: State<'_, AppState>) -> Result<Vec<String>, String> {
    let opts = state.options.lock().map_err(|_| "Ошибка доступа к AppState".to_string())?;
    let path = &opts.characters_dir;

    // ИСПРАВЛЕНИЕ ОСНОВНОЙ ОШИБКИ: Если папки нет, создаем её, а не падаем в ошибку
    if !path.exists() {
        fs::create_dir_all(path).map_err(|e| format!("Не удалось создать папку: {}", e))?;
    }

    let entries = fs::read_dir(path).map_err(|e| e.to_string())?;
    let mut characters = Vec::new();

    for entry in entries {
        let entry = entry.map_err(|e| e.to_string())?;
        if let Ok(name) = entry.file_name().into_string() {
            // Теперь ищем только файлы сценариста .writer
            if name.ends_with(".writer") {
                characters.push(name);
            }
        }
    }
    characters.sort();
    Ok(characters)
}

#[tauri::command]
pub async fn create_character(character: Character, state: State<'_, AppState>) -> Result<String, String> {
    let opts = state.options.lock().map_err(|_| "Ошибка доступа к AppState".to_string())?;
    
    let file_base = if !character.last_name.trim().is_empty() {
        character.last_name.trim().to_string()
    } else {
        "Новый_Персонаж".to_string()
    };

    let file_name = format!("{}.writer", file_base);
    let file_path = opts.characters_dir.join(&file_name);

    // Сериализуем структуру в XML строку
    let xml_content = to_xml_string(&character).map_err(|e| format!("XML Error: {}", e))?;
    
    // Пишем в файл
    fs::write(&file_path, xml_content).map_err(|e| e.to_string())?;

    Ok(file_name)
}

#[tauri::command]
pub async fn read_character(name_file: String, state: State<'_, AppState>) -> Result<Character, String> {
    let opts = state.options.lock().map_err(|_| "Ошибка доступа к AppState".to_string())?;
    let file_path = opts.characters_dir.join(&name_file);

    let content = fs::read_to_string(&file_path)
        .map_err(|e| format!("Не удалось открыть файл персонажа: {}", e))?;
        
    // Десериализуем из XML обратно в структуру Rust
    let character: Character = from_xml_str(&content)
        .map_err(|e| format!("Ошибка чтения формата .writer (XML): {}", e))?;

    Ok(character)
}

#[tauri::command]
pub async fn write_to_character(name_file: String, character: Character, state: State<'_, AppState>) -> Result<String, String> {
    let opts = state.options.lock().map_err(|_| "Ошибка доступа к AppState".to_string())?;
    let file_path = opts.characters_dir.join(&name_file);

    let xml_content = to_xml_string(&character).map_err(|e| format!("XML Error: {}", e))?;
    fs::write(&file_path, xml_content).map_err(|e| e.to_string())?;
    
    Ok(format!("Обновлено досье: {}", name_file))
}