struct PreCheckReport {
    project_valid: bool,
    dart_file_count: u32,
    warning: Option<String>,
    error: Option<String>
}

