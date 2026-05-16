use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use serde::{Deserialize, Serialize};

mod formats;
mod scanner;

use formats::registry;
use scanner::{scan_path, FileMatch};

static CANCEL_FLAG: AtomicBool = AtomicBool::new(false);

#[derive(Serialize, Deserialize)]
pub struct ApplyResult {
    pub path: String,
    pub success: bool,
    pub error: Option<String>,
}

#[tauri::command]
async fn cancel_scan() {
    CANCEL_FLAG.store(true, Ordering::Relaxed);
}

#[tauri::command]
async fn scan_directory(
    app: tauri::AppHandle,
    path: String,
    recursive: bool,
) -> Result<Vec<FileMatch>, String> {
    CANCEL_FLAG.store(false, Ordering::Relaxed);
    let root = PathBuf::from(&path);
    if !root.exists() {
        return Err(format!("Path does not exist: {path}"));
    }
    Ok(scan_path(&root, recursive, &app, &CANCEL_FLAG))
}

#[tauri::command]
async fn scan_all_drives(app: tauri::AppHandle) -> Result<Vec<FileMatch>, String> {
    CANCEL_FLAG.store(false, Ordering::Relaxed);
    let mut results = Vec::new();
    #[cfg(target_os = "windows")]
    for letter in b'A'..=b'Z' {
        if CANCEL_FLAG.load(Ordering::Relaxed) { break; }
        let drive = format!("{}:\\", letter as char);
        let p = Path::new(&drive);
        if p.exists() {
            results.extend(scan_path(p, true, &app, &CANCEL_FLAG));
        }
    }
    #[cfg(not(target_os = "windows"))]
    results.extend(scan_path(Path::new("/"), true, &app, &CANCEL_FLAG));
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
            cancel_scan,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
