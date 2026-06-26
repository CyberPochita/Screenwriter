use quick_xml::events::{BytesEnd, BytesStart, Event};
use quick_xml::Writer;
use std::fs::{self, File};

use crate::models::fileinfo::FileInfo;
use crate::models::xml_struct::blocks::ScenarioBlock;
use crate::scenario::page::PageDto;
use crate::xml::xml_structure::{generate_block_xml, parse_block_xml};
use crate::AppState;

#[tauri::command(rename_all = "camelCase")]
pub fn enter_project(
    state: tauri::State<'_, AppState>,
    projectName: String,
) -> Result<String, String> {
    // 1. Блокируем Mutex как МУТАБЕЛЬНЫЙ (добавлено слово mut)
    let mut opts = state.options.lock().map_err(|_| "State lock error")?;

    // 2. Формируем полный путь к папке проекта сценариев
    let project_path = opts.scenarios_dir.join(&projectName);

    // 3. Создаем директорию на диске, если её ещё нет
    fs::create_dir_all(&project_path)
        .map_err(|e| format!("Не удалось создать папку проекта: {}", e))?;

    // 4. КРИТИЧЕСКИ ВАЖНО: Переключаем текущую рабочую директорию приложения на этот проект!
    opts.current_dir = project_path;

    Ok(format!(
        "Успешно вошли в проект и обновили пути: {}",
        projectName
    ))
}

#[tauri::command]
pub fn exit_project(state: tauri::State<'_, AppState>) -> Result<String, String> {
    let mut opts = state.options.lock().map_err(|_| "State lock error")?;

    opts.current_dir.pop();

    Ok(opts
        .current_dir
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("")
        .to_string())
}

// Создание файла
#[tauri::command(rename_all = "camelCase")]
pub async fn create_file(
    state: tauri::State<'_, AppState>,
    name: String,
) -> Result<String, String> {
    let opts = state.options.lock().map_err(|_| "State lock error")?;
    let file_path = opts.current_dir.join(&name).with_extension("writer");

    if file_path.exists() {
        return Err("Файл с таким именем уже существует".to_string());
    }

    std::fs::File::create(&file_path)
        .map_err(|e| format!("Не удалось инициализировать файл: {}", e))?;

    Ok("File created and empty".to_string())
}

#[tauri::command(rename_all = "camelCase")]
pub async fn delete_file(
    state: tauri::State<'_, AppState>,
    name: String,
) -> Result<String, String> {
    let opts = state.options.lock().map_err(|_| "State lock error")?;

    let file_path = opts.current_dir.join(name);
    let metadata = fs::metadata(&file_path).map_err(|_| "Failed read file metadata")?;

    if metadata.is_file() {
        fs::remove_file(file_path).map_err(|_| "Failed delete file")?;
    } else if metadata.is_dir() {
        fs::remove_dir_all(file_path).map_err(|_| "Failed delete directory")?;
    } else {
        return Err("Specified path is neither a file nor a directory".to_string());
    }

    Ok("File deleted".to_string())
}

// Запись файла
#[tauri::command(rename_all = "camelCase")]
pub async fn write_to_file(
    state: tauri::State<'_, AppState>,
    file_path: String,
    pages: Vec<serde_json::Value>,
) -> Result<String, String> {
    let opts = state.options.lock().map_err(|_| "State lock error")?;
    let file_path = opts.current_dir.join(&file_path);

    drop(opts);

    let file = File::create(&file_path).map_err(|e| format!("Не удалось создать файл: {}", e))?;

    let mut writer = Writer::new_with_indent(file, b' ', 4);

    writer
        .write_event(Event::Start(BytesStart::new("script")))
        .map_err(|e| format!("Ошибка записи структуры XML: {}", e))?;

    for page_val in pages {
        let page: PageDto = serde_json::from_value(page_val.clone()).map_err(|e| {
            // Если упадет — эта строчка напечатает в терминал Rust ВЕСЬ объект фронтенда,
            // и вы сразу увидите, какое именно поле прислал JS (например, blockType вместо block_type)
            println!("Критическая ошибка десериализации JSON от фронтенда: {:?}", page_val);
            format!("Ошибка структуры данных страницы: {}", e)
        })?;
        
        let mut page_tag = BytesStart::new("page");
        let num_str = page.number.to_string();
        page_tag.push_attribute(("number", num_str.as_str()));

        writer
            .write_event(Event::Start(page_tag))
            .map_err(|e| format!("Не удалось записать page_tag: {}", e))?;

        for block in &page.blocks {
            generate_block_xml(&mut writer, block)?;
        }

        writer
            .write_event(Event::End(BytesEnd::new("page")))
            .map_err(|e| format!("Не удалось записать page_tag: {}", e))?;
    }

    writer
        .write_event(Event::End(BytesEnd::new("script")))
        .map_err(|e| format!("Ошибка записи структуры XML: {}", e))?;

    Ok("File written successfully".to_string())
}

/// Чтение: Возвращает десериализованную структуру ScenarioDocument вместо сырой строки
#[tauri::command(rename_all = "camelCase")]
pub async fn entry_file(
    state: tauri::State<'_, AppState>,
    name_file: String,
) -> Result<Option<Vec<PageDto>>, String> {
    // Возвращаем ScenarioDocument вместо String
    let mut opts = state.options.lock().map_err(|_| "State lock error")?;
    let mut path = opts.current_dir.join(&name_file);
    
    if !path.is_dir() && path.extension().map_or(true, |ext| ext != "writer") {
        path = path.with_extension("writer");
    }

    let metadata = fs::metadata(&path).map_err(|e| format!("Не удалось прочитать путь: {}", e))?;

    if metadata.is_dir() {
        opts.current_dir.push(&name_file);
        Ok(None)
    } else if metadata.is_file() {
        if metadata.len() == 0 {
            return Ok(Some(vec![PageDto {
                number: 1,
                blocks: vec![],
            }]));
        }

        let scenario_pages = parse_block_xml(&path)?;
        Ok(Some(scenario_pages))
    } else {
        Err("Указанный путь не является ни файлом, ни директорией".to_string())
    }
}

#[tauri::command]
pub async fn get_files(state: tauri::State<'_, AppState>) -> Result<Vec<FileInfo>, String> {
    let opts = state.options.lock().map_err(|_| "State lock error")?;

    let entries = fs::read_dir(&opts.current_dir).map_err(|e| e.to_string())?;
    let mut files: Vec<FileInfo> = Vec::new();

    for entry in entries {
        let entry = entry.map_err(|e| e.to_string())?;

        let path = entry.path();
        let name = path
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("")
            .to_string();

        let format = path
            .extension()
            .and_then(|s| s.to_str())
            .unwrap_or("")
            .to_string();

        files.push(FileInfo {
            file_name: name,
            file_format: format,
        });
    }

    Ok(files)
}

#[tauri::command]
pub async fn return_dir(state: tauri::State<'_, AppState>) -> Result<(), String> {
    let mut opts = state.options.lock().map_err(|_| "State lock error")?;

    opts.current_dir = opts.scenarios_dir.clone();
    Ok(())
}
