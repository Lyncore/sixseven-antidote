use std::path::Path;
use walkdir::WalkDir;
use serde::{Deserialize, Serialize};
use crate::formats::registry;

#[derive(Serialize, Deserialize, Clone)]
pub struct FileMatch {
    pub path: String,
    pub match_count: usize,
    pub format: String,
}

pub fn scan_path(root: &Path, recursive: bool) -> Vec<FileMatch> {
    let reg = registry();
    let exts = reg.extensions();

    let walker = if recursive {
        WalkDir::new(root).follow_links(false)
    } else {
        WalkDir::new(root).max_depth(1).follow_links(false)
    };

    let mut results = Vec::new();
    for entry in walker.into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if !path.is_file() { continue; }

        let ext = path
            .extension()
            .and_then(|e| e.to_str())
            .map(|e| e.to_lowercase())
            .unwrap_or_default();

        if !exts.contains(&ext.as_str()) { continue; }
        let Some(handler) = reg.get(&ext) else { continue };

        let Ok(count) = handler.count_matches(path) else { continue };
        if count == 0 { continue; }

        results.push(FileMatch {
            path: path.to_string_lossy().to_string(),
            match_count: count,
            format: ext,
        });
    }
    results
}
