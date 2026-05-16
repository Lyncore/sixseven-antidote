use crate::formats::registry;
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use tauri::Emitter;
use walkdir::WalkDir;

#[derive(Serialize, Deserialize, Clone)]
pub struct FileMatch {
    pub path: String,
    pub match_count: usize,
    pub format: String,
}

#[derive(Serialize, Clone)]
pub struct ScanProgress {
    pub scanned: usize,
    pub total: usize,
    pub found: usize,
    pub current: String,
    pub percent: u8,
}

fn make_walker(root: &Path, recursive: bool) -> WalkDir {
    if recursive {
        WalkDir::new(root).follow_links(false)
    } else {
        WalkDir::new(root).max_depth(1).follow_links(false)
    }
}

/// Pass 1: collect all eligible file paths (extension check only, no reading).
fn collect_candidates(root: &Path, recursive: bool, exts: &[&str]) -> Vec<std::path::PathBuf> {
    make_walker(root, recursive)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| {
            if !e.path().is_file() {
                return false;
            }
            let ext = e
                .path()
                .extension()
                .and_then(|x| x.to_str())
                .map(|x| x.to_lowercase())
                .unwrap_or_default();
            exts.contains(&ext.as_str())
        })
        .map(|e| e.into_path())
        .collect()
}

pub fn scan_path(
    root: &Path,
    recursive: bool,
    app: &tauri::AppHandle,
    cancel: &AtomicBool,
) -> Vec<FileMatch> {
    let reg = registry();
    let exts = reg.extensions();

    let candidates = collect_candidates(root, recursive, &exts);
    let total = candidates.len();
    let mut results = Vec::new();

    for (i, path) in candidates.iter().enumerate() {
        if cancel.load(Ordering::Relaxed) {
            break;
        }

        let scanned = i + 1;
        let ext = path
            .extension()
            .and_then(|e| e.to_str())
            .map(|e| e.to_lowercase())
            .unwrap_or_default();

        let Some(handler) = reg.get(&ext) else {
            continue;
        };
        let Ok(count) = handler.count_matches(path) else {
            continue;
        };

        if count > 0 {
            results.push(FileMatch {
                path: path.to_string_lossy().to_string(),
                match_count: count,
                format: ext,
            });
        }

        app.emit(
            "scan-progress",
            ScanProgress {
                scanned,
                total,
                found: results.len(),
                current: path.to_string_lossy().to_string(),
                percent: ((scanned as f32 / total.max(1) as f32) * 100.0) as u8,
            },
        )
        .ok();
    }

    results
}
