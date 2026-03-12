use clap::Parser;
use std::path::Path;
mod analysers;
mod collectors;
mod models;
mod precheck;
use crate::analysers::asset_analyser::{asset_size_calculator, format_size, find_unused_assets};
use crate::collectors::asset_collector::{expand_assets, read_pubspec};
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
        Ok(()) => match read_pubspec(usr_path) {
            Ok(assets) if assets.is_empty() => {
                println!("No assets declared");
            }
            Ok(assets) => {
                println!("Assets");
                let expanded: Vec<std::path::PathBuf> = expand_assets(usr_path, assets);
                let analysed_res = asset_size_calculator(expanded.clone());
                for asset in analysed_res {
                    println!("{:<20} {}", asset.name, format_size(asset.size));
                }

                //detect unused assets
                let unused = find_unused_assets(usr_path, &expanded);
                if unused.is_empty() {
                    println!("\nNo unused assets found");
                } else {
                    println!("\nUnused Assets");
                    for asset in &unused {
                        println!("{:<20} {}", asset.name, asset.path);
                    }
                    println!("\nTotal unused: {}", unused.len());
                }
            }
            Err(e) => println!("Error: {}", e),
        },
        Err(e) => {
            println!("Pre-check failed: {}", e);
            return;
        }
    }
}
