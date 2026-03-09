

use std::fs;
use std::path::PathBuf;
use std::io;
use std::{ path::Path}; 
use crate::models::pubspec_model::Pubspec;
use walkdir::WalkDir;
pub fn read_pubspec(proj_path: &Path) -> Result<Vec<String>, io::Error> {
    let mut path = PathBuf::from(proj_path);
    path.push("pubspec.yaml");

    let content = fs::read_to_string(path)?;
    
    let parsed: Pubspec = serde_yaml::from_str(&content).map_err(|e| io::Error::new(io::ErrorKind::InvalidData,e))?;
    
    let assets = parsed.flutter.and_then(|f| f.assets).unwrap_or_default();
    
    Ok(assets)
}

//expand the assets
pub fn expand_assets(project_root: &Path, assets: Vec<String>) -> Vec<PathBuf> {
    let mut result = Vec::new();

    for asset in assets {
        let full_path = project_root.join(&asset);

        if full_path.is_file() {
            result.push(full_path);
        } else if full_path.is_dir() {
            for entry in WalkDir::new(full_path) {
                let entry = entry.unwrap();
                if entry.path().is_file() {
                    result.push(entry.path().to_path_buf());
                }
            }
        }
    }

    result
}