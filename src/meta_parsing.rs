use serde::{Deserialize};

async fn download_meta_for_version(version: &str) -> Result<LauncherMeta, ()> {
    let meta: LauncherVersionManifestV2 = reqwest::get("https://piston-meta.mojang.com/mc/game/version_manifest_v2.json").await.unwrap().json().await.unwrap();
    let version = meta.versions.into_iter().filter(|it| it.id.as_str() == version).nth(0).unwrap();
    Ok(reqwest::get(version.url).await.unwrap().json().await.unwrap())
}

#[derive(Deserialize)]
struct LauncherMeta {

}

#[derive(Deserialize)]
struct LauncherVersionManifestV2 {
    latest: LauncherVersionManifestV2_LatestVersions,
    versions: Vec<LauncherVersionManifestV2_Version>
}

#[derive(Deserialize)]
struct LauncherVersionManifestV2_LatestVersions {
    release: String,
    snapshot: String
}

#[derive(Deserialize)]
struct LauncherVersionManifestV2_Version {
    id: String,
    // #[serde(rename = "type")]
    // version_type: LauncherVersionManifestV2_VersionType,
    url: String
}

// #[derive(Deserialize)]
// enum LauncherVersionManifestV2_VersionType {
//     #[serde(rename = "snapshot")]
//     Snapshot,
//     #[serde(rename = "release")]
//     Release
// }