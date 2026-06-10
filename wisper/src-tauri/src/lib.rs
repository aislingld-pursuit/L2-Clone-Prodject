use std::path::PathBuf;

use tauri::Manager;
use wisper_core::{
    compute_info, resolve_model_path, transcribe_file, ComputeBackend, ComputeInfo, TranscriptSegment,
};

fn models_dir(app: &tauri::AppHandle) -> PathBuf {
    app.path()
        .app_data_dir()
        .unwrap_or_else(|_| PathBuf::from("."))
        .join("models")
}

fn model_path(app: &tauri::AppHandle) -> PathBuf {
    resolve_model_path(&models_dir(app))
}

#[tauri::command]
fn get_model_path(app: tauri::AppHandle) -> Result<String, String> {
    Ok(model_path(&app).to_string_lossy().into_owned())
}

#[tauri::command]
fn get_compute_info() -> ComputeInfo {
    compute_info()
}

#[tauri::command]
fn transcribe_audio(
    app: tauri::AppHandle,
    audio_path: String,
    use_gpu: bool,
) -> Result<Vec<TranscriptSegment>, String> {
    let backend = if use_gpu {
        ComputeBackend::Gpu
    } else {
        ComputeBackend::Cpu
    };
    let model = model_path(&app);
    transcribe_file(&model, PathBuf::from(&audio_path).as_path(), backend)
        .map_err(|e| e.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            std::fs::create_dir_all(models_dir(app.handle())).ok();
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_model_path,
            get_compute_info,
            transcribe_audio
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
