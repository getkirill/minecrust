use std::{collections::HashMap, fs, path::Path};

use bytes::Bytes;
use serde::Deserialize;
use sha1::{Digest, Sha1};

use crate::meta_parsing::{Asset, AssetListing, LauncherMeta, LauncherMetaAssetIndex, LauncherVersionManifestV2, Library, LibraryDownloadArtifact};

/**
 * Downloads all assets and index into appropriate foldre
 */
pub async fn download_assets(assets: &LauncherMetaAssetIndex, dir: &Path) {
    let index = reqwest::get(&assets.url)
    .await
    .unwrap().text().await.unwrap();
    fs::create_dir_all(dir.join("indexes")).unwrap();
    fs::write(dir.join(format!("indexes/{}.json", assets.id)), &index);
    let listing: AssetListing = serde_json::from_str(&index.as_str()).unwrap();
    fs::create_dir_all(dir).unwrap();
    for (_, asset) in listing.objects {
        // let asset_path_str = format!("{}/{}", &asset.hash[..2], asset.hash);
        let asset_path = dir.join(format!("objects/{}/{}", &asset.hash[..2], asset.hash));
        fs::create_dir_all(asset_path.parent().unwrap()).unwrap();
        if asset_path.exists() {
            println!("Asset {} already exists!", asset_path.to_str().unwrap());
            continue;
        }
        println!("Downloading asset {}", asset.hash);
        fs::write(asset_path, download_asset(&asset).await).unwrap();
    }
}

pub async fn verify_assets(assets: &LauncherMetaAssetIndex, dir: &Path) -> Result<(), ()> {
    let listing: AssetListing = reqwest::get(&assets.url)
    .await
    .unwrap().json().await.unwrap();
    for (_, asset) in listing.objects {
        // let asset_path_str = format!("{}/{}", &asset.hash[..2], asset.hash);
        let asset_path = dir.join(format!("objects/{}/{}", &asset.hash[..2], asset.hash));
        if asset_path.exists() {
            let mut hasher = Sha1::new();
            hasher.update(fs::read(&asset_path).unwrap());
            let hash = hasher.finalize();
            // println!(
            //     "Asset hash: {}\nComputed: {}\nMatch: {:?}",
            //     asset.hash,
            //     hex::encode(hash),
            //     asset.hash == hex::encode(hash)
            // );
            if asset.hash == hex::encode(hash) {
                continue;
            } else {
                return Err(());
            }
        }
    }
    Ok(())
}

pub async fn download_asset(asset: &Asset) -> Bytes {
    return reqwest::get(format!(
        "https://resources.download.minecraft.net/{}/{}",
        &asset.hash[..2],
        asset.hash
    ))
    .await
    .unwrap()
    .bytes()
    .await
    .unwrap();
}

pub async fn download_libraries(libraries: &Vec<Library>, dir: &Path) {
    for libary in libraries {
        // libary.downloads
        if let Some(artifact) = &libary.downloads.artifact {
            let path = dir.join(&artifact.path);
            fs::create_dir_all(path.parent().unwrap()).unwrap();
            if path.exists() {
                println!("Artifact {} already exists!", &artifact.path);
                continue;
            }
            println!("Downloading library artifact {}", artifact.path);
            fs::write(path, download_library_artifact(&artifact).await).unwrap();
        }
    }
}

pub async fn verify_libraries(libraries: &Vec<Library>, dir: &Path) -> Result<(), ()>{
    for libary in libraries {
        // libary.downloads
        if let Some(artifact) = &libary.downloads.artifact {
            let path = dir.join(&artifact.path);
            if path.exists() {
                let mut hasher = Sha1::new();
                hasher.update(fs::read(&path).unwrap());
                let hash = hasher.finalize();
                if artifact.sha1 == hex::encode(hash) {
                    continue;
                } else {
                    return Err(())
                }
            }
        }
    }
    Ok(())
}

pub async fn download_library_artifact(artifact: &LibraryDownloadArtifact) -> Bytes {
    return reqwest::get(&artifact.url)
        .await
        .unwrap()
        .bytes()
        .await
        .unwrap();
}

pub async fn download_meta_for_version(version: &str) -> Result<String, ()> {
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
        .text()
        .await
        .unwrap())
}

pub async fn download_minecraft_jar(manifest: &LauncherMeta, dlType: &str) -> Bytes {
    reqwest::get(&manifest.downloads[dlType].url).await.unwrap().bytes().await.unwrap()
}