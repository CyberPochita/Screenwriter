use std::fs::{self, File};
use crate::AppState;
use tauri::State;
use quick_xml::se::to_string as to_xml_string;
use quick_xml::de::from_str as from_xml_str;
use crate::models::location::Location;

#[tauri::command]
pub async fn get_locations(state: State<'_, AppState>) -> Result<Vec<String>, String> {
    let opts = state.options.lock().map_err(|_| "Ошибка AppState".to_string())?;
    // Автоматически создаем папку locations рядом с персонажами, если её нет
    let path = &opts.locations_dir; 
    if !path.exists() {
        fs::create_dir_all(&path).map_err(|e| e.to_string())?;
    }

    let entries = fs::read_dir(path).map_err(|e| e.to_string())?;
    let mut locations = Vec::new();
    for entry in entries {
        let entry = entry.map_err(|e| e.to_string())?;
        if let Ok(name) = entry.file_name().into_string() {
            if name.ends_with(".writer") {
                locations.push(name);
            }
        }
    }
    locations.sort();
    Ok(locations)
}

#[tauri::command]
pub async fn create_location(location: Location, state: State<'_, AppState>) -> Result<String, String> {
    let opts = state.options.lock().map_err(|_| "Ошибка AppState".to_string())?;
    let path = &opts.locations_dir; 

    let mut base_name = if !location.name.trim().is_empty() {
        location.name.trim().to_string()
    } else {
        "Новая_Локация".to_string()
    };

    // Очищаем slug в нижний регистр под твой стиль
    base_name = base_name.to_lowercase()
        .replace(" ", "_")
        .chars()
        .filter(|c| c.is_alphanumeric() || *c == '_' || *c == '-')
        .collect::<String>();

    let mut file_name = format!("{}.writer", base_name);
    let mut file_path = path.join(&file_name);

    let mut counter = 1;
    while file_path.exists() {
        file_name = format!("{}__{}.writer", base_name, counter);
        file_path = path.join(&file_name);
        counter += 1;
    }

    let xml_content = to_xml_string(&location).map_err(|e| format!("XML Error: {}", e))?;
    fs::write(&file_path, xml_content).map_err(|e| e.to_string())?;
    Ok(file_name)
}

#[tauri::command]
pub async fn read_location(name_file: String, state: State<'_, AppState>) -> Result<Location, String> {
    let opts = state.options.lock().map_err(|_| "Ошибка AppState".to_string())?;
    let path = &opts.locations_dir; 
    let file_path = path.join(&name_file);

    let content = fs::read_to_string(&file_path).map_err(|e| e.to_string())?;
    let location: Location = from_xml_str(&content).map_err(|e| format!("XML Error: {}", e))?;
    Ok(location)
}

#[tauri::command]
pub async fn write_to_location(name_file: String, location: Location, state: State<'_, AppState>) -> Result<String, String> {
    let opts = state.options.lock().map_err(|_| "Ошибка AppState".to_string())?;
    let path = &opts.locations_dir; 
    let old_file_path = path.join(&name_file);

    let mut new_base_name = if !location.name.trim().is_empty() {
        location.name.trim().to_string()
    } else {
        "unnamed_location".to_string()
    };

    new_base_name = new_base_name.to_lowercase()
        .replace(" ", "_")
        .chars()
        .filter(|c| c.is_alphanumeric() || *c == '_' || *c == '-')
        .collect::<String>();

    let mut new_file_name = format!("{}.writer", new_base_name);
    let mut new_file_path = path.join(&new_file_name);

    if new_file_name != name_file {
        let mut counter = 1;
        while new_file_path.exists() {
            new_file_name = format!("{}__{}.writer", new_base_name, counter);
            new_file_path = path.join(&new_file_name);
            counter += 1;
        }
        if old_file_path.exists() {
            fs::remove_file(&old_file_path).map_err(|e| e.to_string())?;
        }
    } else {
        new_file_path = old_file_path;
    }

    let xml_content = to_xml_string(&location).map_err(|e| format!("XML Error: {}", e))?;
    fs::write(&new_file_path, xml_content).map_err(|e| e.to_string())?;
    Ok(new_file_name)
}

#[tauri::command]
pub async fn read_location_by_name(name: String, state: tauri::State<'_, AppState>) -> Result<Location, String> {
    let opts = state.options.lock().map_err(|_| "Ошибка доступа к AppState".to_string())?;
    let path = &opts.locations_dir; 

    if !path.exists() {
        return Err("Папка локаций не существует".to_string());
    }

    // Приводим искомое имя к нижнему регистру для безопасного сравнения
    let target_slug = name.to_lowercase().trim().to_string();

    let entries = fs::read_dir(&path).map_err(|e| e.to_string())?;
    let mut target_file_path = None;

    // Сканируем папку и ищем файл, игнорируя регистр букв в его названии
    for entry in entries {
        if let Ok(entry) = entry {
            let file_name_os = entry.file_name();
            if let Some(file_name_str) = file_name_os.to_str() {
                // Превращаем "Kvartira_Zubeka.writer" -> "kvartira_zubeka"
                let current_slug = file_name_str.replace(".writer", "").to_lowercase();
                if current_slug == target_slug {
                    target_file_path = Some(entry.path());
                    break;
                }
            }
        }
    }

    // Если файл нашли — читаем его, если нет — отдаем понятную ошибку
    let final_path = target_file_path.ok_or_else(|| {
        format!("Локация '{}' не найдена в папке {}", target_slug, path.display())
    })?;

    let content = fs::read_to_string(&final_path)
        .map_err(|e| format!("Не удалось открыть файл {}: {}", final_path.display(), e))?;
        
    let location: Location = quick_xml::de::from_str(&content)
        .map_err(|e| format!("Ошибка парсинга XML локации: {}", e))?;

    Ok(location)
}

#[tauri::command]
pub async fn delete_location_file(name_file: String, state: tauri::State<'_, AppState>) -> Result<String, String> {
    let opts = state.options.lock().map_err(|_| "Ошибка доступа к AppState".to_string())?;
    
    let safe_name = std::path::Path::new(&name_file).file_name().ok_or("Некорректное имя файла")?;
    let file_path = opts.locations_dir.join(safe_name);

    if file_path.exists() {
        fs::remove_file(&file_path).map_err(|e| format!("Не удалось удалить файл локации: {}", e))?;
        Ok(format!("Файл {} успешно удален", name_file))
    } else {
        Err("Файл локации не найден".to_string())
    }
}
