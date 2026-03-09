
use std::path::Path;
use super::flutter_proj_check::flutter_proj_check;
use super::fs_check::check_path_is_valid;

pub fn run_pre_checks(path: &Path) -> Result<(), String> {
    if !check_path_is_valid(path) {
        return Err("Path validation failed".to_string());
    }
    if !flutter_proj_check(path) {
        return Err("Flutter project validation failed".to_string());
    }
    Ok(())
}
