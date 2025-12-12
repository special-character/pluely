use base64::{engine::general_purpose::STANDARD as B64, Engine as _};
use std::fs;
use tauri::{AppHandle, Manager};

#[tauri::command]
pub fn save_wav_base64_to_file(app: AppHandle, wav_base64: String) -> Result<String, String> {
    let bytes = B64
        .decode(wav_base64.trim())
        .map_err(|e| format!("base64 decode failed: {e}"))?;

    let dir = app
        .path()
        .app_data_dir()
        .map_err(|e| format!("app_data_dir failed: {e}"))?
        .join("recordings");

    fs::create_dir_all(&dir).map_err(|e| format!("mkdir failed: {e}"))?;

    let filename = format!("system_{}.wav", chrono::Utc::now().format("%Y%m%d_%H%M%S%.3f"));
    let path = dir.join(filename);

    fs::write(&path, bytes).map_err(|e| format!("write failed: {e}"))?;

    Ok(path.to_string_lossy().to_string())
}
