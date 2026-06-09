use std::path::PathBuf;

use tauri::Manager;
use wisper_core::{transcribe_file, TranscriptSegment, DEFAULT_MODEL_FILENAME};

fn default_model_path(app: &tauri::AppHandle) -> PathBuf {
    app.path()
        .app_data_dir()
        .unwrap_or_else(|_| PathBuf::from("."))
        .join("models")
        .join(DEFAULT_MODEL_FILENAME)
}

#[tauri::command]
fn get_model_path(app: tauri::AppHandle) -> Result<String, String> {
    Ok(default_model_path(&app).to_string_lossy().into_owned())
}

#[tauri::command]
fn transcribe_audio(app: tauri::AppHandle, audio_path: String) -> Result<Vec<TranscriptSegment>, String> {
    let model = default_model_path(&app);
    transcribe_file(&model, PathBuf::from(&audio_path).as_path()).map_err(|e| e.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let models_dir = app
                .path()
                .app_data_dir()
                .map(|d| d.join("models"))
                .unwrap_or_else(|_| PathBuf::from("models"));

            std::fs::create_dir_all(&models_dir).ok();
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![get_model_path, transcribe_audio])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
