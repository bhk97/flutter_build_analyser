use std::fs;
use std::path::{Path, PathBuf};
use zip::ZipArchive;

//find the APK file in the build output directory
//prefers release over debug
pub fn find_apk(project_root: &Path) -> Option<PathBuf> {
    let apk_dir = project_root.join("build/app/outputs/flutter-apk");

    if !apk_dir.exists() {
        return None;
    }

    //prefer release APK
    let release = apk_dir.join("app-release.apk");
    if release.exists() {
        return Some(release);
    }

    //fallback to debug APK
    let debug = apk_dir.join("app-debug.apk");
    if debug.exists() {
        return Some(debug);
    }

    //fallback - find any .apk file in the directory
    if let Ok(entries) = fs::read_dir(&apk_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().map_or(false, |ext| ext == "apk") {
                return Some(path);
            }
        }
    }

    None
}

//read APK as a ZIP and return all entries with their compressed sizes
pub fn read_apk_entries(apk_path: &Path) -> Result<Vec<(String, u64)>, String> {
    let file = fs::File::open(apk_path)
        .map_err(|e| format!("Failed to open APK: {}", e))?;

    let mut archive = ZipArchive::new(file)
        .map_err(|e| format!("Failed to read APK as ZIP: {}", e))?;

    let mut entries = Vec::new();

    for i in 0..archive.len() {
        let entry = archive
            .by_index(i)
            .map_err(|e| format!("Failed to read ZIP entry {}: {}", i, e))?;

        if !entry.is_dir() {
            entries.push((entry.name().to_string(), entry.compressed_size()));
        }
    }

    Ok(entries)
}

//get total APK file size on disk
pub fn get_apk_size(apk_path: &Path) -> Result<u64, String> {
    fs::metadata(apk_path)
        .map(|m| m.len())
        .map_err(|e| format!("Failed to read APK size: {}", e))
}
