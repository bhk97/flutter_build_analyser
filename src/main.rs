use clap::Parser;
use std::path::Path;
mod analysers;
mod collectors;
mod models;
mod precheck;
mod report;
use crate::analysers::asset_analyser::{asset_size_calculator, format_size, find_unused_assets};
use crate::analysers::build_analyser::{analyse_build_timing, format_duration};
use crate::analysers::dep_analyser::{analyse_dep_graph, print_dep_summary};
use crate::collectors::asset_collector::{expand_assets, read_pubspec};
use crate::collectors::build_collector::run_flutter_build;
use crate::collectors::dep_collector::{parse_lockfile, resolve_cache_dir};
use crate::report::report_model::{AnalysisReport, AssetReport};
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
    let is_json = args
        .output_type
        .as_deref()
        .map(|s| s.eq_ignore_ascii_case("json"))
        .unwrap_or(false);

    // Run pre checks - fs check and flutter project check
    let result = run_pre_checks(usr_path);

    match result {
        Ok(()) => {
            let mut report = AnalysisReport::new();

            //asset analysis
            match read_pubspec(usr_path) {
                Ok(assets) if assets.is_empty() => {
                    if !is_json {
                        println!("No assets declared");
                    }
                }
                Ok(assets) => {
                    let expanded: Vec<std::path::PathBuf> = expand_assets(usr_path, assets);
                    let analysed_res = asset_size_calculator(expanded.clone());
                    let unused = find_unused_assets(usr_path, &expanded);

                    if is_json {
                        report.assets = Some(AssetReport {
                            assets: analysed_res,
                            unused_assets: unused,
                        });
                    } else {
                        println!("Assets");
                        for asset in &analysed_res {
                            println!("{:<20} {}", asset.name, format_size(asset.size));
                        }

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
                }
                Err(e) => {
                    if !is_json {
                        println!("Error: {}", e);
                    }
                }
            }

            //dependency graph analysis
            match parse_lockfile(usr_path) {
                Ok(entries) => {
                    let cache_dir = resolve_cache_dir();
                    let graph = analyse_dep_graph(&entries, &cache_dir);

                    if is_json {
                        report.dependencies = Some(graph);
                    } else {
                        print_dep_summary(&graph);
                    }
                }
                Err(e) => {
                    if !is_json {
                        println!("Error reading lockfile: {}", e);
                    }
                }
            }

            //build timing analysis - only run for actual build types
            let build_types = ["apk", "appbundle", "ios", "ipa", "web", "linux", "macos", "windows"];
            if build_types.contains(&args.build_command.as_str()) {
                if !is_json {
                    println!("\nRunning build...");
                }
                match run_flutter_build(usr_path, &args.build_command) {
                    Ok(build_output) => {
                        let timing = analyse_build_timing(&build_output);

                        if is_json {
                            report.build_timing = Some(timing);
                        } else {
                            println!("\nBuild Timing");
                            println!("{}", "-".repeat(50));
                            println!("{:<30} {}", "Total Build Time", format_duration(timing.total_duration_ms));
                            println!("{}", "-".repeat(50));

                            if !timing.phases.is_empty() {
                                println!("\nPhase Breakdown");
                                for phase in &timing.phases {
                                    let percentage = (phase.duration_ms as f64 / timing.total_duration_ms as f64) * 100.0;
                                    println!("{:<30} {:>10}  ({:.1}%)", phase.phase_name, format_duration(phase.duration_ms), percentage);
                                }
                            }

                            if !build_output.success {
                                println!("\nBuild completed with errors");
                            }
                        }
                    }
                    Err(e) => {
                        if !is_json {
                            println!("Build failed: {}", e);
                        }
                    }
                }
            }

            //print json report at end if requested
            if is_json {
                println!("{}", report.to_json());
            }
        }
        Err(e) => {
            println!("Pre-check failed: {}", e);
            return;
        }
    }
}
