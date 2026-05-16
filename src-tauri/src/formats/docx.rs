use super::FormatHandler;
use std::fs;
use std::io::{Read, Write};
use std::path::Path;

pub struct DocxHandler;

impl DocxHandler {
    fn extract_xml(path: &Path) -> Result<String, String> {
        let file = fs::File::open(path).map_err(|e| e.to_string())?;
        let mut archive = zip::ZipArchive::new(file).map_err(|e| e.to_string())?;
        let mut entry = archive
            .by_name("word/document.xml")
            .map_err(|_| "word/document.xml not found".to_string())?;
        let mut xml = String::new();
        entry.read_to_string(&mut xml).map_err(|e| e.to_string())?;
        Ok(xml)
    }

    fn xml_to_text(xml: &str) -> String {
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
        text
    }
}

impl FormatHandler for DocxHandler {
    fn extension(&self) -> &str {
        "docx"
    }

    fn count_matches(&self, path: &Path) -> Result<usize, String> {
        let xml = Self::extract_xml(path)?;
        Ok(Self::xml_to_text(&xml).matches("67").count())
    }

    fn replace(&self, path: &Path) -> Result<usize, String> {
        let data = fs::read(path).map_err(|e| e.to_string())?;
        let mut archive =
            zip::ZipArchive::new(std::io::Cursor::new(&data)).map_err(|e| e.to_string())?;

        let mut entries: Vec<(String, Vec<u8>)> = Vec::new();
        for i in 0..archive.len() {
            let mut entry = archive.by_index(i).map_err(|e| e.to_string())?;
            let name = entry.name().to_string();
            let mut buf = Vec::new();
            entry.read_to_end(&mut buf).map_err(|e| e.to_string())?;
            entries.push((name, buf));
        }

        let mut count = 0;
        for (name, data) in &mut entries {
            if name == "word/document.xml" {
                let xml = String::from_utf8_lossy(data).to_string();
                count = Self::xml_to_text(&xml).matches("67").count();
                *data = xml.replace("67", "69").into_bytes();
            }
        }

        if count == 0 {
            return Ok(0);
        }

        let mut out_buf = Vec::new();
        {
            let mut writer = zip::ZipWriter::new(std::io::Cursor::new(&mut out_buf));
            let options = zip::write::SimpleFileOptions::default()
                .compression_method(zip::CompressionMethod::Deflated);
            for (name, data) in &entries {
                writer
                    .start_file(name, options)
                    .map_err(|e| e.to_string())?;
                writer.write_all(data).map_err(|e| e.to_string())?;
            }
            writer.finish().map_err(|e| e.to_string())?;
        }

        fs::write(path, &out_buf).map_err(|e| e.to_string())?;
        Ok(count)
    }
}
