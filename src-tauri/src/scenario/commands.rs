use std::fs::{self, File};
use std::io::{Read, Write};
use crate::AppState;
use crate::models::fileinfo::FileInfo;
// Импортируем нашу корневую структуру документа
use crate::models::scenario_document::ScenarioDocument;
// Импортируем TitlePage для инициализации новых файлов шаблоном
use crate::models::structure_xml::TitlePage;

#[tauri::command(rename_all = "camelCase")]
pub fn enter_project(state: tauri::State<'_, AppState>, projectName: String) -> Result<String, String> {
    let opts = state.options.lock().map_err(|_| "State lock error")?;
    
    let project_path = opts.scenarios_dir.join(&projectName);
    fs::create_dir_all(&project_path).map_err(|e| e.to_string())?;
 
    Ok(format!("Вошли в проект: {}", projectName))
}

#[tauri::command]
pub fn exit_project(state: tauri::State<'_, AppState>) -> Result<String, String> {
    let mut opts = state.options.lock().map_err(|_| "State lock error")?;
    
    opts.current_dir.pop();
    
    Ok(opts.current_dir.file_name().and_then(|name| name.to_str()).unwrap_or("").to_string())
}

/// Создание файла: Теперь сразу инициализирует .writer файл валидной XML структурой
#[tauri::command(rename_all = "camelCase")]
pub async fn create_file(state: tauri::State<'_, AppState>, name: String) -> Result<String, String> {
    let opts = state.options.lock().map_err(|_| "State lock error")?;
    let file_path = opts.current_dir.join(&name);

    // Если файл уже существует — не перезаписываем его, чтобы не стереть данные
    if file_path.exists() {
        return Err("Файл с таким именем уже существует".to_string());
    }

    // Создаем пустой дефолтный шаблон документа, чтобы XML-парсер не ломался при открытии
    let default_doc = ScenarioDocument {
        title_page: TitlePage {
            formatting: Default::default(), // Вызывает реализованный нами ранее Default
            title: name.replace(".writer", "").to_uppercase(),
            author: "".to_string(),
            authorship: crate::models::xml_struct::authorship::Authorship::Original,
            contact: crate::models::xml_struct::contact_info::ContactInfo {
                left_margin: 8.25,
                name: Some("".to_string()),
                address: None,
                phone: None,
                email: None,
                agent: None,
            },
        },
        pages: vec![crate::models::scenario_document::PageContent { text: "".to_string() }],
    };

    // Сериализуем дефолтную структуру в XML строку
    let xml_string = default_doc.save_to_xml_string().map_err(|e| e.to_string())?;

    // Записываем структуру на диск
    fs::write(&file_path, xml_string).map_err(|e| format!("Не удалось инициализировать файл: {}", e))?;

    Ok("File created and initialized".to_string())
}

#[tauri::command(rename_all = "camelCase")]
pub async fn delete_file(state: tauri::State<'_, AppState>, name: String) -> Result<String, String> {
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

/// Запись: Принимает готовый типизированный ScenarioDocument, превращает в XML и пишет на диск
#[tauri::command(rename_all = "camelCase")]
pub async fn write_to_file(
    state: tauri::State<'_, AppState>, 
    document: ScenarioDocument, // Принимаем структуру вместо String
    file: String
) -> Result<String, String> {
    let opts = state.options.lock().map_err(|_| "State lock error")?;
    let file_path = opts.current_dir.join(file);

    // Сериализуем данные в XML-строку через методы нашей структуры
    let xml_content = document.save_to_xml_string().map_err(|e| e.to_string())?;

    // Записываем XML-контент на диск
    fs::write(file_path, xml_content).map_err(|e| format!("Ошибка записи файла: {}", e))?;
    
    Ok("File written successfully".to_string())
}

/// Чтение: Возвращает десериализованную структуру ScenarioDocument вместо сырой строки
#[tauri::command(rename_all = "camelCase")]
pub async fn entry_file(
    state: tauri::State<'_, AppState>, 
    name_file: String
) -> Result<Option<ScenarioDocument>, String> { // Возвращаем ScenarioDocument вместо String
    let mut opts = state.options.lock().map_err(|_| "State lock error")?;
    let path = opts.current_dir.join(&name_file);
    let metadata = fs::metadata(&path).map_err(|e| format!("Не удалось прочитать путь: {}", e))?;
    
    println!("До условий: {:?}", opts.current_dir);

    if metadata.is_dir() {
        println!("Условие - папка: {:?}", opts.current_dir.join(&name_file));
        opts.current_dir.push(&name_file);
        Ok(None)
    } else if metadata.is_file() {
        println!("Условие - файл: {:?}", opts.current_dir.join(&name_file));
        
        // Читаем сырой XML-текст из файла
        let mut xml_content = String::new();
        let mut file = File::open(&path).map_err(|e| e.to_string())?;
        file.read_to_string(&mut xml_content).map_err(|e| e.to_string())?;

        // Десериализуем XML-строку обратно в готовую структуру
        let document = ScenarioDocument::load_from_xml_string(&xml_content).map_err(|e| e.to_string())?;

        // Возвращаем структуру (Tauri превратит её в JSON-объект для фронтенда)
        Ok(Some(document))
    } else {
        println!("Ошибка в условиях {:?}\n{:?}", opts.current_dir.join(&name_file), opts.current_dir);
        Err("Указанный путь не является ни файлом, ни директорией".to_string())
    }
}

#[tauri::command]
pub async fn get_files(state: tauri::State<'_, AppState>) -> Result<Vec<FileInfo>, String> {
  let opts = state.options.lock().map_err(|_| "State lock error")?;
  
  let entries = fs::read_dir(&opts.current_dir)
    .map_err(|e| e.to_string())?;
  let mut files: Vec<FileInfo> = Vec::new();

  for entry in entries {
    let entry = entry
      .map_err(|e| e.to_string())?;
    
    let path = entry.path();
    let name = path.file_name()
      .and_then(|s| s.to_str())
      .unwrap_or("")
      .to_string();

    let format = path.extension()
      .and_then(|s| s.to_str())
      .unwrap_or("")
      .to_string();

    files.push(FileInfo {file_name: name, file_format: format});
  }

  Ok(files)
}

#[tauri::command]
pub async fn return_dir(state: tauri::State<'_, AppState>) -> Result<(), String> {
    let mut opts = state.options.lock().map_err(|_| "State lock error")?;

    opts.current_dir = opts.scenarios_dir.clone();
    Ok(())
}
