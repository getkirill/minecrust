use std::path::Path;

use minecrust::{downloads::download_assets, meta_parsing};

#[tokio::main]
async fn main() {
    println!("Hi from minecrust binary!!!");
    let version = meta_parsing::download_meta_for_version("1.20.4")
        .await
        .unwrap();
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
    download_assets(version.assetIndex, Path::new("./assets")).await;
}
