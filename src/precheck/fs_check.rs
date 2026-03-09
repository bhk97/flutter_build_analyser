use std::{path::Path}; 
pub fn check_path_is_valid(path: &Path) -> bool {
    if !path.exists() {
        println!("Incorrect path! Please Specify Correct Path");
        return false;
    }
    if !path.is_dir() {
        println!("Given Path is not directory! Please Specify Flutter Project Path");
        return false;
    }
    return true;
}
