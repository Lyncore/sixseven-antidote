use std::fs;
use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};

mod formats;
mod scanner;

use formats::registry;
use scanner::{scan_path, FileMatch};

#[derive(Serialize, Deserialize)]
pub struct ApplyResult {
    pub path: String,
    pub success: bool,
    pub error: Option<String>,
}

#[tauri::command]
async fn scan_directory(path: String, recursive: bool) -> Result<Vec<FileMatch>, String> {
    let root = PathBuf::from(&path);
    if !root.exists() {
        return Err(format!("Path does not exist: {path}"));
    }
    Ok(scan_path(&root, recursive))
}

#[tauri::command]
async fn scan_all_drives() -> Result<Vec<FileMatch>, String> {
    let mut results = Vec::new();
    #[cfg(target_os = "windows")]
    for letter in b'A'..=b'Z' {
        let drive = format!("{}:\\", letter as char);
        let p = Path::new(&drive);
        if p.exists() {
            results.extend(scan_path(p, true));
        }
    }
    #[cfg(not(target_os = "windows"))]
    results.extend(scan_path(Path::new("/"), true));
    Ok(results)
}

#[tauri::command]
async fn apply_replacements(paths: Vec<String>, backup: bool) -> Result<Vec<ApplyResult>, String> {
    let reg = registry();
    let mut results = Vec::new();

    for path_str in paths {
        let path = Path::new(&path_str);
        let ext = path
            .extension()
            .and_then(|e| e.to_str())
            .map(|e| e.to_lowercase())
            .unwrap_or_default();

        let Some(handler) = reg.get(&ext) else {
            results.push(ApplyResult {
                path: path_str,
                success: false,
                error: Some(format!("Unsupported format: {ext}")),
            });
            continue;
        };

        if backup {
            if let Err(e) = fs::copy(path, format!("{path_str}.bak")) {
                results.push(ApplyResult {
                    path: path_str,
                    success: false,
                    error: Some(format!("Backup failed: {e}")),
                });
                continue;
            }
        }

        let outcome = handler.replace(path);
        results.push(ApplyResult {
            path: path_str,
            success: outcome.is_ok(),
            error: outcome.err(),
        });
    }
    Ok(results)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            scan_directory,
            scan_all_drives,
            apply_replacements,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
