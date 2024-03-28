use std::{borrow::Borrow, fs, path::Path};

use minecrust::{
    downloads::{
        download_assets, download_libraries, download_meta_for_version, download_minecraft_jar,
    },
    launch::{launch_minecraft, LaunchArgs},
    meta_parsing::{self, LauncherMeta, LauncherVersionManifestV2},
};

#[tokio::main]
async fn main() {
    println!("Hi from minecrust binary!!!");
    let version: LauncherMeta =
        serde_json::from_str(download_meta_for_version("1.20.4").await.unwrap().as_str()).unwrap();
    // println!("{:?}", &version);
    // println!(
    //     "{}",
    //     &version
    //         .arguments
    //         .game
    //         .iter()
    //         .map(|it| match it {
    //             // shenanigans here - beware
    //             meta_parsing::Argument::Simple(s) => s.clone(),
    //             meta_parsing::Argument::Rule(r) => match r.value.borrow() {
    //                 meta_parsing::RuleValue::Single(s) => s.clone(),
    //                 meta_parsing::RuleValue::Multiple(m) => m.join(" "),
    //                 // meta_parsing::RuleValue::Multiple(m) => m.join(" "),
    //             },
    //         })
    //         .collect::<Vec<String>>()
    //         .join(" ")
    //         .as_str()
    // );
    let path = Path::new("./run");
    fs::create_dir_all(path).unwrap();
    fs::create_dir_all(path.join("./.minecraft")).unwrap();
    fs::create_dir_all(path.join("./native")).unwrap();
    fs::write(
        path.join("./assets/index.json"),
        reqwest::get(&version.assetIndex.url)
            .await
            .unwrap()
            .bytes()
            .await
            .unwrap(),
    )
    .unwrap();
    // download_assets(&version.assetIndex, path.join("./assets").as_path()).await;
    // download_libraries(&version.libraries, path.join("./libs").as_path()).await;
    fs::write(
        path.join("./client.jar"),
        download_minecraft_jar(&version, "client").await,
    ).unwrap();
    // println!("{}", path.join("./libs")
    // .canonicalize()
    // .unwrap()
    // .to_str()
    // .unwrap());
    launch_minecraft(
        &version,
        &LaunchArgs {
            jarPath: path.join("./client.jar"),
            assetsDir: path.join("./assets"),
            gameDir: path.join("./.minecraft"),
            libraryPath: path.join("./libs"),
            nativesPath: path.join("./native"),
            javaPath: String::from("/usr/lib/jvm/java-17-openjdk/bin/java"),
            username: String::from("test"),
            accessToken: None
        },
    )
}
