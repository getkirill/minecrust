use std::{fs, path::Path};

use bytes::Bytes;
use sha1::{Digest, Sha1};

use crate::{
    meta_parsing::{
        Asset, AssetListing, LauncherMeta, LauncherMetaAssetIndex, LauncherVersionManifestV2,
        Library, LibraryDownloadArtifact,
    },
    ProgressCallback,
};

/**
 * Downloads all assets and index into appropriate folder
 */
pub async fn download_assets(
    assets: &LauncherMetaAssetIndex,
    dir: &Path,
    progress: Option<&ProgressCallback<usize, (String, Asset)>>,
) {
    let index = reqwest::get(&assets.url)
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    fs::create_dir_all(dir.join("indexes")).unwrap();
    fs::write(dir.join(format!("indexes/{}.json", assets.id)), &index).unwrap();
    let listing: AssetListing = serde_json::from_str(&index.as_str()).unwrap();
    fs::create_dir_all(dir).unwrap();
    let mut asset_counter = 0;
    let asset_total = listing.objects.keys().len();
    for (path, asset) in listing.objects {
        // let asset_path_str = format!("{}/{}", &asset.hash[..2], asset.hash);
        let asset_path = dir.join(format!("objects/{}/{}", &asset.hash[..2], &asset.hash));
        fs::create_dir_all(asset_path.parent().unwrap()).unwrap();
        if asset_path.exists() {
            // println!("Asset {} already exists!", asset_path.to_str().unwrap());
            asset_counter += 1;
            if let Some(c) = progress {
                c(
                    asset_counter.clone(),
                    asset_total.clone(),
                    (path.clone(), asset.into()),
                );
            }
            continue;
        }
        // println!("Downloading asset {}", &asset.hash);
        fs::write(asset_path, download_asset(&asset).await).unwrap();
        asset_counter += 1;
        if let Some(c) = progress {
            c(
                asset_counter.clone(),
                asset_total.clone(),
                (path.clone(), asset.into()),
            );
        }
    }
}

pub async fn verify_assets(
    assets: &LauncherMetaAssetIndex,
    dir: &Path,
    progress: Option<&ProgressCallback<usize, (String, Asset)>>,
) -> Result<(), ()> {
    let listing: AssetListing = reqwest::get(&assets.url)
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    let mut asset_counter = 0;
    let asset_total = listing.objects.keys().len();
    for (path, asset) in listing.objects {
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
                asset_counter += 1;
                if let Some(c) = progress {
                    c(
                        asset_counter.clone(),
                        asset_total.clone(),
                        (path.clone(), asset.into()),
                    );
                }
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

pub async fn download_libraries(
    libraries: &Vec<Library>,
    dir: &Path,
    progress: Option<&ProgressCallback<usize, Library>>,
) {
    let mut lib_counter = 0;
    let lib_total = libraries.len();
    for library in libraries {
        // libary.downloads
        if let Some(artifact) = &library.downloads.artifact {
            let path = dir.join(&artifact.path);
            fs::create_dir_all(path.parent().unwrap()).unwrap();
            if path.exists() {
                // println!("Artifact {} already exists!", &artifact.path);
                lib_counter += 1;
                if let Some(c) = progress {
                    c(lib_counter.clone(), lib_total.clone(), library.clone());
                }
                continue;
            }
            // println!("Downloading library artifact {}", artifact.path);
            fs::write(path, download_library_artifact(&artifact).await).unwrap();
            lib_counter += 1;
            if let Some(c) = progress {
                c(lib_counter.clone(), lib_total.clone(), library.clone());
            }
        }
    }
}

pub async fn verify_libraries(
    libraries: &Vec<Library>,
    dir: &Path,
    progress: Option<&ProgressCallback<usize, Library>>,
) -> Result<(), ()> {
    let mut lib_counter = 0;
    let lib_total = libraries.len();
    for library in libraries {
        // libary.downloads
        if let Some(artifact) = &library.downloads.artifact {
            let path = dir.join(&artifact.path);
            if path.exists() {
                let mut hasher = Sha1::new();
                hasher.update(fs::read(&path).unwrap());
                let hash = hasher.finalize();
                if artifact.sha1 == hex::encode(hash) {
                    lib_counter += 1;
                    if let Some(c) = progress {
                        c(lib_counter.clone(), lib_total.clone(), library.clone());
                    }
                    continue;
                } else {
                    return Err(());
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

pub async fn download_version_manifest() -> Result<LauncherVersionManifestV2, ()> {
    Ok(
        reqwest::get("https://piston-meta.mojang.com/mc/game/version_manifest_v2.json")
            .await
            .unwrap()
            .json()
            .await
            .unwrap(),
    )
}

pub async fn download_meta_for_version(version: &str) -> Result<String, ()> {
    let meta: LauncherVersionManifestV2 = download_version_manifest().await.unwrap();
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

pub async fn download_minecraft_jar(manifest: &LauncherMeta, dl_type: &str) -> Bytes {
    reqwest::get(&manifest.downloads[dl_type].url)
        .await
        .unwrap()
        .bytes()
        .await
        .unwrap()
}
