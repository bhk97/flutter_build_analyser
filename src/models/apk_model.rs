use serde::Serialize;

//a single category in the APK breakdown
#[derive(Serialize)]
pub struct ApkCategory {
    pub name: String,
    pub size: u64,
    pub percentage: f64,
    pub file_count: usize,
}

//per-architecture native library detail
#[derive(Serialize)]
pub struct NativeLibDetail {
    pub architecture: String,
    pub size: u64,
    pub percentage: f64,
}

//individual file entry for largest files
#[derive(Serialize)]
pub struct ApkFileEntry {
    pub path: String,
    pub size: u64,
}

//full APK breakdown result
#[derive(Serialize)]
pub struct ApkBreakdownResult {
    pub apk_file_name: String,
    pub total_size: u64,
    pub categories: Vec<ApkCategory>,
    pub native_libs: Vec<NativeLibDetail>,
    pub largest_files: Vec<ApkFileEntry>,
}
