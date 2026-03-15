use serde::Serialize;

pub struct BuildOutput {
    pub stdout: String,
    pub stderr: String,
    pub total_duration_ms: u128,
    pub success: bool,
}

#[derive(Serialize)]
pub struct BuildPhaseInfo {
    pub phase_name: String,
    pub duration_ms: u128,
}

#[derive(Serialize)]
pub struct BuildTimingResult {
    pub total_duration_ms: u128,
    pub phases: Vec<BuildPhaseInfo>,
}
