use std::{fs, path::Path}; 
use clap::Parser;
mod precheck;
use precheck::flutter_proj_check::flutter_proj_check;
use precheck::fs_check::check_path_is_valid;

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
    let path_check_res = check_path_is_valid(usr_path);
    if path_check_res {
        let res = print_project_files(usr_path);
        println!("File Path Count: {}", res);
        let proj_type_check = flutter_proj_check(usr_path);
        println!("Fluter proj check: {}", proj_type_check);
    } else {
        println!("Please pass correct path");
        return;
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
