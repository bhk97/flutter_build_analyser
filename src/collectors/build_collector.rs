

use std::path::Path;
use std::process::Command;
use std::time::Instant;
use crate::models::build_model::BuildOutput;

//run flutter build with verbose flag and capture output with timing
pub fn run_flutter_build(project_path: &Path, build_type: &str) -> Result<BuildOutput, String> {
    let start = Instant::now();

    let output = Command::new("flutter")
        .arg("build")
        .arg(build_type)
        .arg("--verbose")
        .current_dir(project_path)
        .output()
        .map_err(|e| format!("Failed to run flutter build: {}", e))?;

    let total_duration_ms = start.elapsed().as_millis();

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    let success = output.status.success();

    Ok(BuildOutput {
        stdout,
        stderr,
        total_duration_ms,
        success,
    })
}
