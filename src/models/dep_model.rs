use serde::Serialize;

pub struct LockPackageEntry {
    pub name: String,
    pub version: String,
    pub dep_type: String,
    pub source: String,
}

#[derive(Serialize)]
pub struct DepPackageInfo {
    pub name: String,
    pub version: String,
    pub dep_type: String,
    pub dependencies: Vec<String>,
    pub dependents: Vec<String>,
    pub size: u64,
}

#[derive(Serialize)]
pub struct DepGraphResult {
    pub packages: Vec<DepPackageInfo>,
    pub total_packages: usize,
    pub total_size: u64,
}
