use serde_json::Value;
use std::fs;
use std::path::PathBuf;

fn get_prefs_path() -> PathBuf {
    let mut path = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
    path.push("biblio-analyzer");
    fs::create_dir_all(&path).ok();
    path.push("preferences.json");
    path
}

pub fn save_last_file_path(path: &str) {
    let prefs_path = get_prefs_path();
    let prefs = serde_json::json!({"last_file": path});
    fs::write(prefs_path, prefs.to_string()).ok();
}

pub fn load_last_file_path() -> Option<String> {
    let prefs_path = get_prefs_path();
    if let Ok(content) = fs::read_to_string(prefs_path) {
        if let Ok(prefs) = serde_json::from_str::<Value>(&content) {
            if let Some(last_file) = prefs.get("last_file").and_then(|v| v.as_str()) {
                if PathBuf::from(last_file).exists() {
                    return Some(last_file.to_string());
                }
            }
        }
    }
    None
}