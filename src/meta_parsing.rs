use std::collections::HashMap;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct LauncherMeta {
    pub arguments: Arguments,
    #[serde(rename = "assetIndex")]
    pub asset_index: LauncherMetaAssetIndex,
    pub assets: String,
    pub downloads: HashMap<String, LauncherMetaDownload>,
    #[serde(rename = "javaVersion")]
    pub java_version: LauncherMetaJavaVersion,
    pub libraries: Vec<Library>,
    #[serde(rename = "mainClass")]
    pub main_class: String
}

#[derive(Deserialize, Debug)]
pub struct Library {
    pub downloads: LibraryDownloads,
    pub name: String,
    pub rules: Option<Vec<Rule>>,
}

#[derive(Deserialize, Debug)]
pub struct LibraryDownloads {
    pub artifact: Option<LibraryDownloadArtifact>,
    pub classifiers: Option<HashMap<String, LibraryDownloadArtifact>>,
    pub natives: Option<HashMap<String, String>>
}

#[derive(Deserialize, Debug)]
pub struct LibraryDownloadArtifact {
    pub path: String,
    pub size: i32,
    pub url: String,
    pub sha1: String
}

#[derive(Deserialize, Debug)]
pub struct LauncherMetaJavaVersion {
    #[serde(rename = "majorVersion")]
    pub major_version: i16,
}

#[derive(Deserialize, Debug)]
pub struct LauncherMetaAssetIndex {
    #[serde(rename = "totalSize")]
    pub total_size: i32,
    pub url: String,
    pub id: String
}

// not gonna bother rn
// #[derive(Deserialize, Debug)]
// pub enum LauncherMetaDownloadType {
//     Client,
//     Server
// }

#[derive(Deserialize, Debug)]
pub struct LauncherMetaDownload {
    pub size: i32,
    pub url: String,
    pub sha1: String
}

#[derive(Deserialize, Debug)]
pub struct Arguments {
    pub game: Vec<Argument>,
    pub jvm: Vec<Argument>
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum Argument {
    Simple(String),
    Rule(Rules),
}

#[derive(Deserialize, Debug)]
pub struct Rules {
    pub rules: Vec<Rule>,
    pub value: RuleValue
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum RuleValue {
    Single(String),
    Multiple(Vec<String>)
}

#[derive(Deserialize, Debug)]
pub struct Rule {
    pub action: RuleAction,
    pub features: Option<HashMap<String, bool>>,
    pub os: Option<RuleOSMatching>
}

#[derive(Deserialize, Debug)]
pub enum RuleAction {
    #[serde(rename = "allow")]
    Allow,
    #[serde(rename = "disallow")]
    Disallow,
}

#[derive(Deserialize, Debug)]
pub struct RuleOSMatching {
    pub name: Option<OperatingSystem>,
    pub version: Option<String>,
    pub arch: Option<String>
}
#[derive(Deserialize, Debug)]
pub enum OperatingSystem {
    #[serde(rename = "windows")]
    Windows,
    #[serde(rename = "osx")]
    Mac,
    #[serde(rename = "linux")]
    Linux
}

#[derive(Deserialize, Debug)]
pub struct LauncherVersionManifestV2 {
    pub latest: LauncherVersionManifestV2_LatestVersions,
    pub versions: Vec<LauncherVersionManifestV2_Version>,
}
#[derive(Deserialize, Debug)]
pub struct LauncherVersionManifestV2_LatestVersions {
    pub release: String,
    pub snapshot: String,
}

#[derive(Deserialize, Debug)]
pub struct LauncherVersionManifestV2_Version {
    pub id: String,
    // #[serde(rename = "type")]
    // version_type: LauncherVersionManifestV2_VersionType,
    pub url: String,
}

// #[derive(Deserialize)]
// pub enum LauncherVersionManifestV2_VersionType {
//     #[serde(rename = "snapshot")]
//     Snapshot,
//     #[serde(rename = "release")]
//     Release
// }



#[derive(Deserialize, Debug)]
pub struct AssetListing {
    pub objects: HashMap<String, Asset>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Asset {
    pub hash: String,
    pub size: i32,
}