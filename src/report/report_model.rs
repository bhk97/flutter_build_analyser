use serde::Serialize;
use crate::models::pubspec_model::{AssetInfo, UnusedAssetInfo};
use crate::models::dep_model::DepGraphResult;
use crate::models::build_model::BuildTimingResult;

#[derive(Serialize)]
pub struct AnalysisReport {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assets: Option<AssetReport>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub dependencies: Option<DepGraphResult>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_timing: Option<BuildTimingResult>,
}

#[derive(Serialize)]
pub struct AssetReport {
    pub assets: Vec<AssetInfo>,
    pub unused_assets: Vec<UnusedAssetInfo>,
}

impl AnalysisReport {
    pub fn new() -> Self {
        AnalysisReport {
            assets: None,
            dependencies: None,
            build_timing: None,
        }
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string_pretty(self).unwrap_or_else(|e| format!("{{\"error\": \"{}\"}}", e))
    }
}
