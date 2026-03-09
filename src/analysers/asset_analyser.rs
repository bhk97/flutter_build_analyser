use std::fs;
use std::path::PathBuf;
use crate::models::pubspec_model::AssetInfo;

//calculate asset size to identify heavy asset
pub fn asset_size_calculator(files: Vec<PathBuf>) -> Vec<AssetInfo> {
    let mut result = Vec::new();

    for path in files {
        if let Ok(metadata) = fs::metadata(&path) {
            let size = metadata.len();

            let name = path
                .file_name()
                .unwrap()
                .to_string_lossy()
                .to_string();

            result.push(AssetInfo { name, size });
        }
    }

    result
}


//formating asset size
pub fn format_size(bytes: u64) -> String {
    const KB: f64 = 1024.0;
    const MB: f64 = KB * 1024.0;
    const GB: f64 = MB * 1024.0;

    let b = bytes as f64;

    if b >= GB {
        format!("{:.2} GB", b / GB)
    } else if b >= MB {
        format!("{:.2} MB", b / MB)
    } else if b >= KB {
        format!("{:.2} KB", b / KB)
    } else {
        format!("{} B", bytes)
    }
}