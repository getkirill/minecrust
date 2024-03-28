use std::{collections::HashMap, hash::Hash};

use serde::Deserialize;
use serde_json::Value;

pub async fn download_meta_for_version(version: &str) -> Result<LauncherMeta, ()> {
    let meta: LauncherVersionManifestV2 =
        reqwest::get("https://piston-meta.mojang.com/mc/game/version_manifest_v2.json")
            .await
            .unwrap()
            .json()
            .await
            .unwrap();
    let version = meta
        .versions
        .into_iter()
        .filter(|it| it.id.as_str() == version)
        .nth(0)
        .unwrap();
    Ok(reqwest::get(version.url)
        .await
        .unwrap()
        .json()
        .await
        .unwrap())
}

#[derive(Deserialize, Debug)]
pub struct LauncherMeta {
    pub arguments: Arguments,
    pub assetIndex: LauncherMetaAssetIndex,
    downloads: HashMap<String, LauncherMetaDownload>,
    javaVersion: LauncherMetaJavaVersion,
    libraries: Vec<Library>,
    mainClass: String
}

#[derive(Deserialize, Debug)]
pub struct Library {
    name: String,
    rules: Option<Vec<Rule>>,
}

// #[derive(Deserialize, Debug)]
// pub struct LibraryDownloads {
//     artifact: Option<LibraryDownloadArtifact>,
//     classifiers: Option<HashMap<String, LibraryDownloadArtifact>>,
//     natives: Option<HashMap<String, String>>
// }

#[derive(Deserialize, Debug)]
pub struct LibraryDownloadArtifact {
    path: String,
    size: i32,
    url: String
}

#[derive(Deserialize, Debug)]
pub struct LauncherMetaJavaVersion {
    majorVersion: i16,
}

#[derive(Deserialize, Debug)]
pub struct LauncherMetaAssetIndex {
    totalSize: i32,
    pub url: String
}

// not gonna bother rn
// #[derive(Deserialize, Debug)]
// pub enum LauncherMetaDownloadType {
//     Client,
//     Server
// }

#[derive(Deserialize, Debug)]
pub struct LauncherMetaDownload {
    size: i32,
    url: String
}

#[derive(Deserialize, Debug)]
pub struct Arguments {
    pub game: Vec<Argument>,
    jvm: Vec<Argument>
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
    action: RuleAction,
    features: Option<HashMap<String, bool>>,
    os: Option<RuleOSMatching>
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
    name: Option<OperatingSystem>,
    version: Option<String>,
    arch: Option<String>
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
    latest: LauncherVersionManifestV2_LatestVersions,
    versions: Vec<LauncherVersionManifestV2_Version>,
}

#[derive(Deserialize, Debug)]
pub struct LauncherVersionManifestV2_LatestVersions {
    release: String,
    snapshot: String,
}

#[derive(Deserialize, Debug)]
pub struct LauncherVersionManifestV2_Version {
    id: String,
    // #[serde(rename = "type")]
    // version_type: LauncherVersionManifestV2_VersionType,
    url: String,
}

// #[derive(Deserialize)]
// pub enum LauncherVersionManifestV2_VersionType {
//     #[serde(rename = "snapshot")]
//     Snapshot,
//     #[serde(rename = "release")]
//     Release
// }
