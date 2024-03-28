use std::{collections::HashMap, fs, path::Path};

use bytes::Bytes;
use serde::Deserialize;
use sha1::{Digest, Sha1};

use crate::meta_parsing::LauncherMetaAssetIndex;

pub async fn download_assets(assets: LauncherMetaAssetIndex, dir: &Path) {
    let listing: AssetListing = reqwest::get(assets.url)
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    fs::create_dir_all(dir).unwrap();
    for (_, asset) in listing.objects {
        fs::create_dir_all(dir.join(&asset.hash[..2])).unwrap();
        let asset_path_str = format!("{}/{}", &asset.hash[..2], asset.hash);
        let asset_path = dir.join(asset_path_str);
        if asset_path.exists() {
            println!("Asset {} already exists!", asset_path.to_str().unwrap());
            let mut hasher = Sha1::new();
            hasher.update(fs::read(asset_path).unwrap());
            let hash = hasher.finalize();
            println!(
                "Asset hash: {}\nComputed: {}\nMatch: {:?}",
                asset.hash,
                hex::encode(hash),
                asset.hash == hex::encode(hash)
            );
            if asset.hash == hex::encode(hash) {
                continue;
            } else {
                println!("Corrupted file {}! Redownloading...", asset.hash)
            }
        }
        println!("Downloading asset {}", asset.hash);
        fs::write(
            dir.join(format!("{}/{}", &asset.hash[..2], asset.hash)),
            download_asset(asset).await,
        )
        .unwrap();
    }
}

pub async fn download_asset(asset: Asset) -> Bytes {
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

#[derive(Deserialize, Debug)]
pub struct AssetListing {
    objects: HashMap<String, Asset>,
}

#[derive(Deserialize, Debug)]
pub struct Asset {
    hash: String,
    size: i32,
}
