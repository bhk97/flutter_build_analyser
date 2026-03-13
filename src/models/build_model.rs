
pub struct BuildOutput {
    pub stdout: String,
    pub stderr: String,
    pub total_duration_ms: u128,
    pub success: bool,
}

pub struct BuildPhaseInfo {
    pub phase_name: String,
    pub duration_ms: u128,
}

pub struct BuildTimingResult {
    pub total_duration_ms: u128,
    pub phases: Vec<BuildPhaseInfo>,
}
