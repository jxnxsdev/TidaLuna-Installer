use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReleaseVersion {
    pub version: String,
    pub download: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Release {
    pub id: String,
    pub name: String,
    pub github_url: Option<String>,
    pub versions: Vec<ReleaseVersion>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ReleaseSourceType {
    Github,
    Direct,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReleaseSource {
    pub id: Option<String>,
    pub url: String,
    #[serde(rename = "type")]
    pub source_type: ReleaseSourceType,
}
