
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use crate::models::dep_model::LockPackageEntry;
use walkdir::WalkDir;

//parse pubspec.lock to get all resolved packages
pub fn parse_lockfile(project_path: &Path) -> Result<Vec<LockPackageEntry>, io::Error> {
    let lock_path = project_path.join("pubspec.lock");
    let content = fs::read_to_string(lock_path)?;

    let parsed: serde_yaml::Value = serde_yaml::from_str(&content)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

    let mut entries = Vec::new();

    if let Some(packages) = parsed.get("packages").and_then(|p| p.as_mapping()) {
        for (key, value) in packages {
            let name = match key.as_str() {
                Some(n) => n.to_string(),
                None => continue,
            };

            let dep_type = value
                .get("dependency")
                .and_then(|d| d.as_str())
                .unwrap_or("unknown")
                .to_string();

            let source = value
                .get("source")
                .and_then(|s| s.as_str())
                .unwrap_or("unknown")
                .to_string();

            let version = value
                .get("version")
                .and_then(|v| v.as_str())
                .unwrap_or("0.0.0")
                .to_string();

            entries.push(LockPackageEntry {
                name,
                version,
                dep_type,
                source,
            });
        }
    }

    Ok(entries)
}

//read a package's dependencies from its pubspec.yaml in pub cache
pub fn read_package_deps(cache_dir: &Path, name: &str, version: &str) -> Vec<String> {
    let pkg_dir = cache_dir.join(format!("{}-{}", name, version));
    let pubspec_path = pkg_dir.join("pubspec.yaml");

    let content = match fs::read_to_string(pubspec_path) {
        Ok(c) => c,
        Err(_) => return Vec::new(),
    };

    let parsed: serde_yaml::Value = match serde_yaml::from_str(&content) {
        Ok(p) => p,
        Err(_) => return Vec::new(),
    };

    let mut deps = Vec::new();

    //extract dependency names from dependencies section
    if let Some(dep_map) = parsed.get("dependencies").and_then(|d| d.as_mapping()) {
        for key in dep_map.keys() {
            if let Some(dep_name) = key.as_str() {
                deps.push(dep_name.to_string());
            }
        }
    }

    deps
}

//calculate total size of a package directory in pub cache
pub fn calculate_package_size(cache_dir: &Path, name: &str, version: &str) -> u64 {
    let pkg_dir = cache_dir.join(format!("{}-{}", name, version));

    if !pkg_dir.is_dir() {
        return 0;
    }

    let mut total_size: u64 = 0;

    for entry in WalkDir::new(&pkg_dir) {
        let entry = match entry {
            Ok(e) => e,
            Err(_) => continue,
        };

        if entry.path().is_file() {
            if let Ok(metadata) = fs::metadata(entry.path()) {
                total_size += metadata.len();
            }
        }
    }

    total_size
}

//resolve pub cache directory path
pub fn resolve_cache_dir() -> PathBuf {
    let home = std::env::var("HOME").unwrap_or_default();

    //check pub.dev first then fallback to pub.dartlang.org
    let primary = PathBuf::from(&home).join(".pub-cache/hosted/pub.dev");
    if primary.is_dir() {
        return primary;
    }

    let fallback = PathBuf::from(&home).join(".pub-cache/hosted/pub.dartlang.org");
    if fallback.is_dir() {
        return fallback;
    }

    primary
}
