use std::fs::{self, File};
use std::io::Read;
use crate::AppState;

#[tauri::command(rename_all = "camelCase")]
pub fn enter_project(state: tauri::State<'_, AppState>, project_name: String) -> Result<String, String> {
    let opts = state.options.lock().map_err(|_| "State lock error")?;
    
    let project_path = opts.scenarios_dir.join(&project_name);
    fs::create_dir_all(&project_path).map_err(|e| e.to_string())?;
 
    Ok(format!("Вошли в проект: {}", project_name))
}

#[tauri::command]
pub fn exit_project(state: tauri::State<'_, AppState>) -> Result<String, String> {
    let mut opts = state.options.lock().map_err(|_| "State lock error")?;
    
    opts.current_dir.pop();
    
    Ok(opts.current_dir.file_name().and_then(|name| name.to_str()).unwrap_or("").to_string())
}

#[tauri::command]
pub async fn create_file(state: tauri::State<'_, AppState>, name: String) -> Result<String, String> {
  let opts = state.options.lock().map_err(|_| "State lock error")?;

  let file_path = opts.current_dir.join(name);
  File::create(file_path).map_err(|_| "Failed create file")?;

  Ok("File created".to_string())
}

#[tauri::command]
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

//TODO: Переписать функцию с использованием opts.current_dir вместо жесткого пути
#[tauri::command]
pub async fn write_to_file(state: tauri::State<'_, AppState>, msg: String, file: String) -> Result<String, String> {
  let opts = state.options.lock().map_err(|_| "State lock error")?;
  let file_path = opts.current_dir.join(file);

  fs::write(file_path, msg)
    .map_err(|_| "Error write to file")?;
  Ok("File written".to_string())
}

#[tauri::command]
pub async fn entry_file(state: tauri::State<'_, AppState>, name_file: String) -> Result<Option<String>, String> {
  let mut opts = state.options.lock().map_err(|_| "State lock error")?;
  let path = opts.current_dir.join(&name_file);
  let metadata = fs::metadata(&path)
    .map_err(|e| format!("Не удалось прочитать путь: {}", e))?;
  println!("До условий: {:?}", opts.current_dir);

  if metadata.is_dir() {
    println!("Условие - папка: {:?}", opts.current_dir.join(&name_file));
    opts.current_dir.push(&name_file);
    Ok(None)
  }
  else if metadata.is_file() {
    println!("Условие - файл: {:?}", opts.current_dir.join(&name_file));
    let mut content = String::new();
    let mut file = File::open(&path).map_err(|e| e.to_string())?;
    file.read_to_string(&mut content).map_err(|e| e.to_string())?;

    Ok(Some(content))
  }
  else {
    println!("Ошибка в условиях {:?}\n{:?}", opts.current_dir.join(&name_file), opts.current_dir);
    Err("Указанный путь не является ни файлом, ни директорией".to_string())
  }
}

#[tauri::command]
pub async fn get_files(state: tauri::State<'_, AppState>) -> Result<Vec<String>, String> {
  let opts = state.options.lock().map_err(|_| "State lock error")?;
  
  let entries = fs::read_dir(&opts.current_dir)
    .map_err(|e| e.to_string())?;
  let mut files = Vec::new();

  for entry in entries {
    let entry = entry
      .map_err(|e| e.to_string())?;
    
    if let Ok(name) = entry.file_name().into_string() {
      files.push(name);
    }
  }

  Ok(files)
}

#[tauri::command]
pub async fn return_dir(state: tauri::State<'_, AppState>) -> Result<(), String> {
    let mut opts = state.options.lock().map_err(|_| "State lock error")?;

    opts.current_dir = opts.scenarios_dir.clone();
    Ok(())
}