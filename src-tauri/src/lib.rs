use std::fs;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct FileMatch {
    pub path: String,
    pub match_count: usize,
    pub format: String,
}

#[derive(Serialize, Deserialize)]
pub struct ApplyResult {
    pub path: String,
    pub success: bool,
    pub error: Option<String>,
}

fn count_matches_txt(content: &str) -> usize {
    content.matches("67").count()
}

fn extract_text_from_docx(path: &Path) -> Result<String, String> {
    let file = fs::File::open(path).map_err(|e| e.to_string())?;
    let mut archive = zip::ZipArchive::new(file).map_err(|e| e.to_string())?;
    let mut doc = archive
        .by_name("word/document.xml")
        .map_err(|_| "word/document.xml not found".to_string())?;
    let mut xml = String::new();
    doc.read_to_string(&mut xml).map_err(|e| e.to_string())?;
    // Strip XML tags to get plain text
    let mut text = String::new();
    let mut in_tag = false;
    for c in xml.chars() {
        match c {
            '<' => in_tag = true,
            '>' => in_tag = false,
            _ if !in_tag => text.push(c),
            _ => {}
        }
    }
    Ok(text)
}

fn replace_in_docx(path: &Path) -> Result<usize, String> {
    let data = fs::read(path).map_err(|e| e.to_string())?;
    let reader = std::io::Cursor::new(&data);
    let mut archive = zip::ZipArchive::new(reader).map_err(|e| e.to_string())?;

    // Read all entries
    let mut entries: Vec<(String, Vec<u8>, zip::CompressionMethod)> = Vec::new();
    for i in 0..archive.len() {
        let mut entry = archive.by_index(i).map_err(|e| e.to_string())?;
        let name = entry.name().to_string();
        let method = entry.compression();
        let mut buf = Vec::new();
        entry.read_to_end(&mut buf).map_err(|e| e.to_string())?;
        entries.push((name, buf, method));
    }

    let mut count = 0usize;

    // Replace in document.xml
    for (name, data, _) in &mut entries {
        if name == "word/document.xml" {
            let xml = String::from_utf8_lossy(data).to_string();
            let replaced = xml.replace("67", "69");
            count = xml.matches("67").count();
            *data = replaced.into_bytes();
        }
    }

    if count == 0 {
        return Ok(0);
    }

    // Repack zip
    let mut out_buf = Vec::new();
    {
        let cursor = std::io::Cursor::new(&mut out_buf);
        let mut writer = zip::ZipWriter::new(cursor);
        for (name, data, _method) in &entries {
            let options = zip::write::SimpleFileOptions::default()
                .compression_method(zip::CompressionMethod::Deflated);
            writer.start_file(name, options).map_err(|e| e.to_string())?;
            writer.write_all(data).map_err(|e| e.to_string())?;
        }
        writer.finish().map_err(|e| e.to_string())?;
    }

    fs::write(path, &out_buf).map_err(|e| e.to_string())?;
    Ok(count)
}

fn scan_path(root: &Path, recursive: bool) -> Vec<FileMatch> {
    let extensions = ["txt", "docx"];
    let walker = if recursive {
        WalkDir::new(root).follow_links(false)
    } else {
        WalkDir::new(root).max_depth(1).follow_links(false)
    };

    let mut results = Vec::new();
    for entry in walker.into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if !path.is_file() {
            continue;
        }
        let ext = path
            .extension()
            .and_then(|e| e.to_str())
            .map(|e| e.to_lowercase())
            .unwrap_or_default();

        if !extensions.contains(&ext.as_str()) {
            continue;
        }

        let (count, format) = match ext.as_str() {
            "txt" => {
                let Ok(content) = fs::read_to_string(path) else { continue };
                (count_matches_txt(&content), "txt".to_string())
            }
            "docx" => {
                let Ok(text) = extract_text_from_docx(path) else { continue };
                (count_matches_txt(&text), "docx".to_string())
            }
            _ => continue,
        };

        if count > 0 {
            results.push(FileMatch {
                path: path.to_string_lossy().to_string(),
                match_count: count,
                format,
            });
        }
    }
    results
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
    let mut results = Vec::new();
    for path_str in paths {
        let path = Path::new(&path_str);
        let ext = path
            .extension()
            .and_then(|e| e.to_str())
            .map(|e| e.to_lowercase())
            .unwrap_or_default();

        if backup {
            let backup_path = format!("{}.bak", path_str);
            if let Err(e) = fs::copy(path, &backup_path) {
                results.push(ApplyResult {
                    path: path_str,
                    success: false,
                    error: Some(format!("Backup failed: {e}")),
                });
                continue;
            }
        }

        let result = match ext.as_str() {
            "txt" => {
                fs::read_to_string(path)
                    .map_err(|e| e.to_string())
                    .and_then(|content| {
                        let replaced = content.replace("67", "69");
                        fs::write(path, replaced).map_err(|e| e.to_string())
                    })
                    .map(|_| ())
            }
            "docx" => replace_in_docx(path).map(|_| ()),
            _ => Err("Unsupported format".to_string()),
        };

        results.push(ApplyResult {
            path: path_str,
            success: result.is_ok(),
            error: result.err(),
        });
    }
    Ok(results)
}

#[tauri::command]
async fn read_file_content(path: String) -> Result<String, String> {
    let p = Path::new(&path);
    let ext = p
        .extension()
        .and_then(|e| e.to_str())
        .map(|e| e.to_lowercase())
        .unwrap_or_default();

    match ext.as_str() {
        "txt" => fs::read_to_string(p).map_err(|e| e.to_string()),
        "docx" => extract_text_from_docx(p),
        _ => Err("Unsupported format".to_string()),
    }
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
            read_file_content,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
