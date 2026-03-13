
use std::collections::HashMap;
use crate::models::build_model::{BuildOutput, BuildPhaseInfo, BuildTimingResult};

//parse verbose build output to extract phase timings
pub fn analyse_build_timing(build_output: &BuildOutput) -> BuildTimingResult {
    let combined = format!("{}\n{}", build_output.stdout, build_output.stderr);
    let mut phase_durations: HashMap<String, u128> = HashMap::new();

    for line in combined.lines() {
        //parse lines with timestamp format [  +XXX ms] or [ +XXXX ms]
        let trimmed = line.trim();

        if let Some(duration_ms) = extract_timestamp(trimmed) {
            let phase = classify_phase(trimmed);
            *phase_durations.entry(phase).or_insert(0) += duration_ms;
        }
    }

    let mut phases: Vec<BuildPhaseInfo> = phase_durations
        .into_iter()
        .map(|(phase_name, duration_ms)| BuildPhaseInfo {
            phase_name,
            duration_ms,
        })
        .collect();

    //sort phases by duration descending
    phases.sort_by(|a, b| b.duration_ms.cmp(&a.duration_ms));

    BuildTimingResult {
        total_duration_ms: build_output.total_duration_ms,
        phases,
    }
}

//extract millisecond timestamp from verbose output line
fn extract_timestamp(line: &str) -> Option<u128> {
    //flutter verbose format: [  +XXX ms] or [ +XXXX ms]
    if !line.starts_with('[') {
        return None;
    }

    let end = line.find(']')?;
    let inner = &line[1..end].trim();

    //handle "+XXX ms" format
    if let Some(stripped) = inner.strip_prefix('+') {
        let parts: Vec<&str> = stripped.trim().split_whitespace().collect();
        if parts.len() == 2 && parts[1] == "ms" {
            return parts[0].parse::<u128>().ok();
        }
    }

    None
}

//classify a verbose output line into a build phase
fn classify_phase(line: &str) -> String {
    let lower = line.to_lowercase();

    if lower.contains("gradle") || lower.contains("assemblerelease") || lower.contains("assembledebug") {
        return "Gradle Build".to_string();
    }
    if lower.contains("xcode") || lower.contains("xcodebuild") {
        return "Xcode Build".to_string();
    }
    if lower.contains("dart") && (lower.contains("compil") || lower.contains("kernel") || lower.contains("snapshot")) {
        return "Dart Compilation".to_string();
    }
    if lower.contains("asset") || lower.contains("bundle") || lower.contains("font") {
        return "Asset Bundling".to_string();
    }
    if lower.contains("sign") || lower.contains("certificate") {
        return "Code Signing".to_string();
    }
    if lower.contains("r8") || lower.contains("proguard") || lower.contains("shrink") || lower.contains("minify") {
        return "Code Shrinking".to_string();
    }
    if lower.contains("dex") || lower.contains("d8") {
        return "Dexing".to_string();
    }
    if lower.contains("merge") || lower.contains("package") || lower.contains("zip") || lower.contains("apk") || lower.contains("aab") {
        return "Packaging".to_string();
    }

    "Other".to_string()
}

//format milliseconds to human readable duration
pub fn format_duration(ms: u128) -> String {
    if ms >= 60000 {
        let mins = ms / 60000;
        let secs = (ms % 60000) / 1000;
        format!("{}m {}s", mins, secs)
    } else if ms >= 1000 {
        let secs = ms / 1000;
        let remaining_ms = ms % 1000;
        format!("{}s {}ms", secs, remaining_ms)
    } else {
        format!("{}ms", ms)
    }
}
