use std::fs::{self, File};
use std::path::Path;
use tauri::State;
use quick_xml::se::to_string as to_xml_string;
use quick_xml::de::from_str as from_xml_str;
use crate::AppState;
use crate::models::character::Character;

#[tauri::command]
pub async fn read_character_by_name(name: String, state: tauri::State<'_, AppState>) -> Result<Character, String> {
    let opts = state.options.lock().map_err(|_| "Ошибка доступа к AppState".to_string())?;
    
    // Формируем имя файла из переданного slug (например, "potter_harry.writer")
    let file_name = format!("{}.writer", name.to_lowercase().trim());
    let file_path = opts.characters_dir.join(file_name);

    if !file_path.exists() {
        return Err("Персонаж не найден в архивной базе".to_string());
    }

    let content = fs::read_to_string(&file_path)
        .map_err(|e| format!("Не удалось открыть файл персонажа: {}", e))?;
        
    // Десериализуем из нашего XML формата .writer
    let character: Character = quick_xml::de::from_str(&content)
        .map_err(|e| format!("Ошибка парсинга формата .writer (XML): {}", e))?;

    Ok(character)
}

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
    
    // 1. Формируем базовое имя: "фамилия_имя" или берем то, что заполнено
    let mut base_name = match (!character.last_name.trim().is_empty(), !character.first_name.trim().is_empty()) {
        (true, true) => format!("{}_{}", character.last_name.trim(), character.first_name.trim()),
        (true, false) => character.last_name.trim().to_string(),
        (false, true) => character.first_name.trim().to_string(),
        (false, false) => "unnamed_character".to_string(),
    };

    // 2. Очищаем имя файла: переводим в нижний регистр, заменяем пробелы на '_'
    // и убираем опасные для файловой системы символы (\, /, :, *, ?, ", <, >, |)
    base_name = base_name.to_lowercase()
        .replace(" ", "_")
        .chars()
        .filter(|c| c.is_alphanumeric() || *c == '_' || *c == '-')
        .collect::<String>();

    let mut file_name = format!("{}.writer", base_name);
    let mut file_path = opts.characters_dir.join(&file_name);

    // 3. Защита от дубликатов: если "potter_harry.writer" существует, создаем "potter_harry_1.writer"
    let mut counter = 1;
    while file_path.exists() {
        file_name = format!("{}_{}.writer", base_name, counter);
        file_path = opts.characters_dir.join(&file_name);
        counter += 1;
    }

    let xml_content = quick_xml::se::to_string(&character).map_err(|e| format!("XML Error: {}", e))?;
    fs::write(&file_path, xml_content).map_err(|e| e.to_string())?;

    Ok(file_name) // Возвращаем красивое сгенерированное имя обратно фронтенду
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
    
    // 1. Формируем старый путь к файлу
    let safe_old_name = Path::new(&name_file).file_name().ok_or("Некорректное старое имя файла")?;
    let old_file_path = opts.characters_dir.join(safe_old_name);

    // 2. Генерируем новое базовое имя на основе измененных ФИО
    let mut new_base_name = match (!character.last_name.trim().is_empty(), !character.first_name.trim().is_empty()) {
        (true, true) => format!("{}_{}", character.last_name.trim(), character.first_name.trim()),
        (true, false) => character.last_name.trim().to_string(),
        (false, true) => character.first_name.trim().to_string(),
        (false, false) => "unnamed_character".to_string(),
    };

    // Очищаем от спецсимволов и пробелов
    new_base_name = new_base_name.to_lowercase()
        .replace(" ", "_")
        .chars()
        .filter(|c| c.is_alphanumeric() || *c == '_' || *c == '-')
        .collect::<String>();

    let mut new_file_name = format!("{}.writer", new_base_name);
    let mut new_file_path = opts.characters_dir.join(&new_file_name);

    // 3. Если ФИО изменились и имя файла должно стать другим
    if new_file_name != name_file {
        // Защита от перезаписи чужих файлов: 
        // если файл "potter_harry.writer" уже занят кем-то другим, делаем "potter_harry_1.writer"
        let mut counter = 1;
        while new_file_path.exists() {
            new_file_name = format!("{}_{}.writer", new_base_name, counter);
            new_file_path = opts.characters_dir.join(&new_file_name);
            counter += 1;
        }

        // Удаляем старый файл, так как теперь досье называется иначе
        if old_file_path.exists() {
            fs::remove_file(&old_file_path).map_err(|e| format!("Не удалось удалить старый файл: {}", e))?;
        }
    } else {
        // Если имя не менялось, пишем в тот же файл (путь остается старым)
        new_file_path = old_file_path;
    }

    // 4. Записываем обновленный XML в целевой файл
    let xml_content = quick_xml::se::to_string(&character).map_err(|e| format!("XML Error: {}", e))?;
    fs::write(&new_file_path, xml_content).map_err(|e| e.to_string())?;
    
    // Возвращаем актуальное имя файла (новое или прежнее), чтобы фронтенд знал его
    Ok(new_file_name)
}

#[tauri::command]
pub async fn delete_character_file(name_file: String, state: tauri::State<'_, AppState>) -> Result<String, String> {
    let opts = state.options.lock().map_err(|_| "Ошибка доступа к AppState".to_string())?;
    
    // Защита: извлекаем только имя файла, отсекая попытки Directory Traversal
    let safe_name = std::path::Path::new(&name_file).file_name().ok_or("Некорректное имя файла")?;
    let file_path = opts.characters_dir.join(safe_name);

    if file_path.exists() {
        fs::remove_file(&file_path).map_err(|e| format!("Не удалось удалить файл персонажа: {}", e))?;
        Ok(format!("Файл {} успешно удален", name_file))
    } else {
        Err("Файл персонажа не найден".to_string())
    }
}
