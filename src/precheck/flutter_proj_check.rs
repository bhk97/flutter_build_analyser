use std::fs;
use std::path::Path;
use std::process::Command;

pub fn flutter_proj_check(path: &Path) -> bool {
    is_flutter_installed() && verify_is_path_to_flutter(path)
}

//fn to check the flutter project existence
fn verify_is_path_to_flutter(path: &Path) -> bool {
    let pubspec = path.join("pubspec.yaml");

    if !pubspec.exists() {
        return false;
    }

    let content = match fs::read_to_string(pubspec) {
        Ok(c) => c,
        Err(_) => return false,
    };
    if !content.contains("sdk: flutter") {
        return false;
    }
    path.join("lib").is_dir()
}

//fn to detect flutter installed or not in both mac/windows
fn is_flutter_installed() -> bool {
    if check_flutter_command("flutter") {
        return true;
    }

    if cfg!(windows) && check_flutter_command("flutter.bat") {
        return true;
    }
    check_common_locations()
}

fn check_flutter_command(cmd: &str) -> bool {
    match Command::new(cmd).arg("--version").output() {
        Ok(output) => output.status.success(),
        Err(_) => false,
    }
}

//check locations
fn check_common_locations() -> bool {
    let possible_paths = [
        // Linux / macOS
        "/usr/bin/flutter",
        "/usr/local/bin/flutter",
        "/opt/flutter/bin/flutter",
        "/snap/bin/flutter",
        // Windows
        "C:\\flutter\\bin\\flutter.bat",
        "C:\\src\\flutter\\bin\\flutter.bat",
    ];
    for path in possible_paths {
        if Path::new(path).exists() {
            return true;
        }
    }
    return false;
}
