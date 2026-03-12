use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use crate::models::pubspec_model::{AssetInfo, UnusedAssetInfo};
use walkdir::WalkDir;

//calculate asset size to identify heavy asset
pub fn asset_size_calculator(files: Vec<PathBuf>) -> Vec<AssetInfo> {
    let mut result = Vec::new();

    for path in files {
        if let Ok(metadata) = fs::metadata(&path) {
            let size = metadata.len();

            let name: String = path
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

//find assets declared in pubspec but not referenced anywhere in project
pub fn find_unused_assets(project_root: &Path, asset_files: &[PathBuf]) -> Vec<UnusedAssetInfo> {
    //build hashmap of asset filenames to track which are still unused
    let mut pending: HashMap<String, UnusedAssetInfo> = HashMap::new();

    for asset_path in asset_files {
        let file_name = asset_path
            .file_name()
            .unwrap()
            .to_string_lossy()
            .to_string();

        let relative_path = asset_path
            .strip_prefix(project_root)
            .unwrap_or(asset_path)
            .to_string_lossy()
            .to_string();

        pending.insert(
            file_name.clone(),
            UnusedAssetInfo {
                name: file_name,
                path: relative_path,
            },
        );
    }

    //dirs to skip during scan
    let skip_dirs = ["build", ".dart_tool", ".git", "target", ".idea", ".vscode"];

    //file extensions to scan for asset references
    let scan_extensions = ["dart", "yaml", "yml", "xml", "json", "gradle", "plist", "html"];

    //single pass - walk entire project once
    for entry in WalkDir::new(project_root) {
        //early exit if all assets are found
        if pending.is_empty() {
            break;
        }

        let entry = match entry {
            Ok(e) => e,
            Err(_) => continue,
        };

        let path = entry.path();

        //skip excluded directories
        if path.is_dir() {
            if let Some(dir_name) = path.file_name() {
                let name = dir_name.to_string_lossy();
                if skip_dirs.iter().any(|d| *d == name.as_ref()) {
                    continue;
                }
            }
            continue;
        }

        //only scan relevant file types
        let is_scannable = path
            .extension()
            .map_or(false, |ext| scan_extensions.iter().any(|e| *e == ext.to_string_lossy().as_ref()));

        if !is_scannable {
            continue;
        }

        //skip the asset files themselves
        if asset_files.iter().any(|a| a == path) {
            continue;
        }

        let content = match fs::read_to_string(path) {
            Ok(c) => c,
            Err(_) => continue,
        };

        //check all remaining pending assets against this file content
        let found_keys: Vec<String> = pending
            .keys()
            .filter(|name| content.contains(name.as_str()))
            .cloned()
            .collect();

        //remove found assets from pending map
        for key in found_keys {
            pending.remove(&key);
        }
    }

    pending.into_values().collect()
}