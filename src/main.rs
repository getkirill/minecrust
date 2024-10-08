use std::{fs, path::Path};

use minecrust::{
    downloads::{
        download_assets, download_libraries, download_meta_for_version, download_minecraft_jar,
    },
    launch::{launch_minecraft, LaunchArgs},
    meta_parsing::{Asset, LauncherMeta, Library}, ProgressCallback,
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
        reqwest::get(&version.asset_index.url)
            .await
            .unwrap()
            .bytes()
            .await
            .unwrap(),
    )
    .unwrap();
    fn asset_callback(progress:usize, total:usize, obj: (String, Asset)) {
        println!("{:6.2}% ({}/{}) Downloading asset {}", (progress as f64 / total as f64) * 100.0, progress, total, obj.0)
    }
    download_assets(&version.asset_index, path.join("./assets").as_path(), Some(&(asset_callback as ProgressCallback<usize, (String, Asset)>))).await;
    fn library_callback(progress:usize, total:usize, obj: Library) {
        println!("{:6.2}% ({}/{}) Downloading library {}", (progress as f64 / total as f64) * 100.0, progress, total, obj.downloads.artifact.unwrap().path)
    }
    download_libraries(&version.libraries, path.join("./libs").as_path(), Some(&(library_callback as ProgressCallback<usize, Library>))).await;
    if path.join("./client.jar").exists() {
        println!("client.jar already there")
    } else {   
        fs::write(
            path.join("./client.jar"),
            download_minecraft_jar(&version, "client").await,
        ).unwrap();
    }
    // println!("{}", path.join("./libs")
    // .canonicalize()
    // .unwrap()
    // .to_str()
    // .unwrap());
    launch_minecraft(
        &version,
        &LaunchArgs {
            jar_path: path.join("./client.jar"),
            assets_dir: path.join("./assets"),
            game_dir: path.join("./.minecraft"),
            library_dir: path.join("./libs"),
            natives_dir: path.join("./native"),
            java_path: String::from("/usr/lib/jvm/java-17-openjdk/bin/java"),
            username: String::from("test"),
            access_token: None,
            version_type: None
        },
    )
}
