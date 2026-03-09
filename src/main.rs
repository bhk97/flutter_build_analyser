use std::{fs, path::Path}; 
use clap::Parser;
mod precheck;
use precheck::runner::run_pre_checks;

#[derive(Parser, Debug)]
#[command(version, about = "Flutter Build Analyser")]
struct Args {
    build_command: String,
    path: String,
    output_type: Option<String>,
}

fn main() {
    let args = Args::parse();

    let usr_path = Path::new(&args.path);
    
    // Run pre checks - fs check and flutter project check
    let result = run_pre_checks(usr_path);
    
    match result {
        Ok(()) => {
            let res = print_project_files(usr_path);
            println!("File Path Count: {}", res);
        }
        Err(e) => {
            println!("Pre-check failed: {}", e);
            return;
        }
    }
}


//fn for total file checker
fn print_project_files(path: &Path) -> u32 {
    let mut file_count: u32 = 0;
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries {
            let entry = entry.unwrap();
            let path = entry.path();

            if path.is_dir() {
                file_count += print_project_files(&path);
            } else {
                file_count += 1;
            }
        }
    }
    return file_count;
}
