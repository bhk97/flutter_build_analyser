use std::collections::HashMap;
use crate::models::apk_model::{ApkBreakdownResult, ApkCategory, NativeLibDetail, ApkFileEntry};
use crate::analysers::asset_analyser::format_size;

//categorize APK entries and build breakdown result
pub fn analyse_apk(entries: Vec<(String, u64)>, apk_size: u64, file_name: &str) -> ApkBreakdownResult {
    let mut category_sizes: HashMap<&str, (u64, usize)> = HashMap::new();
    let mut arch_sizes: HashMap<String, u64> = HashMap::new();
    let mut all_files: Vec<ApkFileEntry> = Vec::new();

    for (path, size) in &entries {
        let category = classify_entry(path);
        let entry = category_sizes.entry(category).or_insert((0, 0));
        entry.0 += size;
        entry.1 += 1;

        //track native lib architectures
        if path.starts_with("lib/") {
            if let Some(arch) = path.strip_prefix("lib/").and_then(|p| p.split('/').next()) {
                *arch_sizes.entry(arch.to_string()).or_insert(0) += size;
            }
        }

        all_files.push(ApkFileEntry {
            path: path.clone(),
            size: *size,
        });
    }

    //sort files by size descending and take top 5
    all_files.sort_by(|a, b| b.size.cmp(&a.size));
    let largest_files: Vec<ApkFileEntry> = all_files.into_iter().take(5).collect();

    //build categories sorted by size
    let category_order = [
        "Native Libraries",
        "Compiled Code",
        "App Assets",
        "Android Resources",
        "Resource Table",
        "Signing & Metadata",
        "Other",
    ];

    let mut categories: Vec<ApkCategory> = Vec::new();
    for name in &category_order {
        if let Some((size, file_count)) = category_sizes.get(name) {
            let percentage = if apk_size > 0 {
                (*size as f64 / apk_size as f64) * 100.0
            } else {
                0.0
            };
            categories.push(ApkCategory {
                name: name.to_string(),
                size: *size,
                percentage,
                file_count: *file_count,
            });
        }
    }
    categories.sort_by(|a, b| b.size.cmp(&a.size));

    //build native lib details
    let total_native: u64 = arch_sizes.values().sum();
    let mut native_libs: Vec<NativeLibDetail> = arch_sizes
        .into_iter()
        .map(|(architecture, size)| {
            let percentage = if total_native > 0 {
                (size as f64 / total_native as f64) * 100.0
            } else {
                0.0
            };
            NativeLibDetail { architecture, size, percentage }
        })
        .collect();
    native_libs.sort_by(|a, b| b.size.cmp(&a.size));

    ApkBreakdownResult {
        apk_file_name: file_name.to_string(),
        total_size: apk_size,
        categories,
        native_libs,
        largest_files,
    }
}

//classify a ZIP entry path into a category
fn classify_entry(path: &str) -> &'static str {
    if path.starts_with("lib/") {
        "Native Libraries"
    } else if path.starts_with("classes") && path.ends_with(".dex") {
        "Compiled Code"
    } else if path.starts_with("assets/") {
        "App Assets"
    } else if path.starts_with("res/") {
        "Android Resources"
    } else if path == "resources.arsc" {
        "Resource Table"
    } else if path.starts_with("META-INF/") || path == "AndroidManifest.xml" {
        "Signing & Metadata"
    } else {
        "Other"
    }
}

//print APK breakdown with visual bar chart
pub fn print_apk_breakdown(result: &ApkBreakdownResult) {
    println!("\n  APK Size Breakdown ({})", format_size(result.total_size));
    println!("  {}", "═".repeat(58));

    //find max bar width - 20 chars for the largest category
    let max_percentage = result
        .categories
        .first()
        .map(|c| c.percentage)
        .unwrap_or(0.0);

    for cat in &result.categories {
        let bar_width = if max_percentage > 0.0 {
            ((cat.percentage / max_percentage) * 20.0).ceil() as usize
        } else {
            0
        };

        let bar = if bar_width == 0 && cat.size > 0 {
            "▏".to_string()
        } else {
            "█".repeat(bar_width)
        };

        println!(
            "  {:<22} {:>8}  {:>5.1}%  {}",
            cat.name,
            format_size(cat.size),
            cat.percentage,
            bar
        );
    }

    //native library breakdown by architecture
    if !result.native_libs.is_empty() {
        println!();
        println!("  Native Libraries by Architecture");
        println!("  {}", "─".repeat(40));

        for lib in &result.native_libs {
            println!(
                "  {:<22} {:>8}  {:>5.1}%",
                lib.architecture,
                format_size(lib.size),
                lib.percentage
            );
        }
    }

    //largest files
    if !result.largest_files.is_empty() {
        println!();
        println!("  Largest Files");
        println!("  {}", "─".repeat(40));

        for file in &result.largest_files {
            //truncate long paths from the middle
            let display_path = if file.path.len() > 40 {
                let start = &file.path[..18];
                let end = &file.path[file.path.len() - 18..];
                format!("{}...{}", start, end)
            } else {
                file.path.clone()
            };

            println!("  {:<42} {:>8}", display_path, format_size(file.size));
        }
    }

    println!();
}
