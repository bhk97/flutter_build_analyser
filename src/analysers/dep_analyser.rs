
use std::collections::HashMap;
use std::path::Path;
use crate::models::dep_model::{LockPackageEntry, DepPackageInfo, DepGraphResult};
use crate::collectors::dep_collector::{read_package_deps, calculate_package_size};
use crate::analysers::asset_analyser::format_size;

//build full dependency graph from lock entries and pub cache
pub fn analyse_dep_graph(entries: &[LockPackageEntry], cache_dir: &Path) -> DepGraphResult {
    //collect dependencies and sizes for each package
    let mut dep_map: HashMap<String, Vec<String>> = HashMap::new();
    let mut size_map: HashMap<String, u64> = HashMap::new();

    for entry in entries {
        let deps = if entry.source == "hosted" {
            read_package_deps(cache_dir, &entry.name, &entry.version)
        } else {
            Vec::new()
        };

        let size = if entry.source == "hosted" {
            calculate_package_size(cache_dir, &entry.name, &entry.version)
        } else {
            0
        };

        dep_map.insert(entry.name.clone(), deps);
        size_map.insert(entry.name.clone(), size);
    }

    //build reverse map - who depends on each package
    let mut dependents_map: HashMap<String, Vec<String>> = HashMap::new();

    for entry in entries {
        dependents_map.insert(entry.name.clone(), Vec::new());
    }

    for (pkg_name, deps) in &dep_map {
        for dep in deps {
            if let Some(dependents) = dependents_map.get_mut(dep.as_str()) {
                dependents.push(pkg_name.clone());
            }
        }
    }

    //build final package info list
    let mut packages: Vec<DepPackageInfo> = Vec::new();
    let mut total_size: u64 = 0;

    for entry in entries {
        let dependencies = dep_map.get(&entry.name).cloned().unwrap_or_default();
        let dependents = dependents_map.get(&entry.name).cloned().unwrap_or_default();
        let size = *size_map.get(&entry.name).unwrap_or(&0);

        total_size += size;

        packages.push(DepPackageInfo {
            name: entry.name.clone(),
            version: entry.version.clone(),
            dep_type: entry.dep_type.clone(),
            dependencies,
            dependents,
            size,
        });
    }

    //sort by most dependents first
    packages.sort_by(|a, b| b.dependents.len().cmp(&a.dependents.len()));

    let total_packages = packages.len();

    DepGraphResult {
        packages,
        total_packages,
        total_size,
    }
}

//print dependency graph summary
pub fn print_dep_summary(result: &DepGraphResult) {
    println!("\nDependency Graph");
    println!("{}", "─".repeat(60));
    println!("  Total packages: {}    Total size: {}", result.total_packages, format_size(result.total_size));
    println!("{}", "─".repeat(60));

    //direct dependencies with their sub-dependencies as tree
    let direct: Vec<&DepPackageInfo> = result
        .packages
        .iter()
        .filter(|p| p.dep_type == "direct main")
        .collect();

    if !direct.is_empty() {
        println!("\n  Direct Dependencies ({})", direct.len());
        println!();

        for pkg in &direct {
            println!("  {} v{}  {}", pkg.name, pkg.version, format_size(pkg.size));

            let dep_count = pkg.dependencies.len();
            for (i, dep) in pkg.dependencies.iter().enumerate() {
                //find the dep package info for size
                let dep_size = result
                    .packages
                    .iter()
                    .find(|p| p.name == *dep)
                    .map(|p| format_size(p.size))
                    .unwrap_or_else(|| "sdk".to_string());

                let connector = if i == dep_count - 1 { "└─" } else { "├─" };
                println!("    {} {}  {}", connector, dep, dep_size);
            }
            println!();
        }
    }

    //dev dependencies
    let dev: Vec<&DepPackageInfo> = result
        .packages
        .iter()
        .filter(|p| p.dep_type == "direct dev")
        .collect();

    if !dev.is_empty() {
        println!("  Dev Dependencies ({})", dev.len());
        println!();

        for pkg in &dev {
            println!("  {} v{}  {}", pkg.name, pkg.version, format_size(pkg.size));

            let dep_count = pkg.dependencies.len();
            for (i, dep) in pkg.dependencies.iter().enumerate() {
                let dep_size = result
                    .packages
                    .iter()
                    .find(|p| p.name == *dep)
                    .map(|p| format_size(p.size))
                    .unwrap_or_else(|| "sdk".to_string());

                let connector = if i == dep_count - 1 { "└─" } else { "├─" };
                println!("    {} {}  {}", connector, dep, dep_size);
            }
            println!();
        }
    }

    //most depended-on - which packages are pulled by others
    println!("  Most Depended-On (pulled by other packages)");
    println!("{}", "─".repeat(60));

    let top_depended: Vec<&DepPackageInfo> = result
        .packages
        .iter()
        .filter(|p| !p.dependents.is_empty())
        .take(10)
        .collect();

    if top_depended.is_empty() {
        println!("  No dependency relationships found");
    } else {
        for pkg in &top_depended {
            println!();
            println!("  {} v{}  ← pulled by {} packages", pkg.name, pkg.version, pkg.dependents.len());
            let dep_count = pkg.dependents.len();
            for (i, dependent) in pkg.dependents.iter().enumerate() {
                let connector = if i == dep_count - 1 { "└─" } else { "├─" };
                println!("    {} {}", connector, dependent);
            }
        }
    }

    //heaviest packages
    let mut by_size: Vec<&DepPackageInfo> = result
        .packages
        .iter()
        .filter(|p| p.size > 0)
        .collect();
    by_size.sort_by(|a, b| b.size.cmp(&a.size));

    println!();
    println!("  Heaviest Packages");
    println!("{}", "─".repeat(60));

    for pkg in by_size.iter().take(10) {
        println!(
            "  {:<35} {:>10}",
            format!("{} v{}", pkg.name, pkg.version),
            format_size(pkg.size)
        );
    }
}

