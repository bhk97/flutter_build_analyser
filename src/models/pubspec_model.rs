use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Pubspec {
    pub flutter: Option<FlutterSection>,
}

#[derive(Debug, Deserialize)]
pub struct FlutterSection {
    pub assets: Option<Vec<String>>,
}

pub struct AssetInfo {
    pub name: String,
    pub size: u64,
}