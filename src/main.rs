use std::path::Path;

use minecrust::{downloads::{download_assets, download_libraries}, meta_parsing::{self, LauncherMeta, LauncherVersionManifestV2}};

#[tokio::main]
async fn main() {
    println!("Hi from minecrust binary!!!");
    let version: LauncherMeta = serde_json::from_str(meta_parsing::download_meta_for_version("1.20.4")
        .await
        .unwrap().as_str()).unwrap();
    println!("{:?}", version);
    println!(
        "{}",
        version
            .arguments
            .game
            .into_iter()
            .map(|it| match it {
                meta_parsing::Argument::Simple(s) => s,
                meta_parsing::Argument::Rule(r) => match r.value {
                    meta_parsing::RuleValue::Single(s) => s,
                    meta_parsing::RuleValue::Multiple(m) => m.join(" "),
                },
            })
            .collect::<Vec<String>>()
            .join(" ").as_str()
    );
    // download_assets(version.assetIndex, Path::new("./assets")).await;
    download_libraries(version.libraries, Path::new("./libs")).await;
}
