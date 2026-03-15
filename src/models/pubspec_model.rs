use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct Pubspec {
    pub flutter: Option<FlutterSection>,
}

#[derive(Debug, Deserialize)]
pub struct FlutterSection {
    pub assets: Option<Vec<String>>,
}

#[derive(Serialize)]
pub struct AssetInfo {
    pub name: String,
    pub size: u64,
}

#[derive(Serialize)]
pub struct UnusedAssetInfo {
    pub name: String,
    pub path: String,
}