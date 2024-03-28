use std::{
    fmt::format,
    path::{Path, PathBuf},
    process::Command,
};

use crate::meta_parsing::{Argument, LauncherMeta};

pub fn launch_minecraft(manifest: &LauncherMeta, launchArgs: &LaunchArgs) {
    let minecraft_args = manifest
        .arguments
        .game
        .iter()
        .filter_map(|it| {
            if let Argument::Simple(s) = it {
                return Some(s.clone());
            } else {
                return None;
            }
        })
        .collect::<Vec<String>>()
        .join(" ")
        .replace("${auth_player_name}", &launchArgs.username)
        .replace(
            "${game_directory}",
            format!(
                "\"{}\"",
                &launchArgs.gameDir.canonicalize().unwrap().to_str().unwrap()
            )
            .as_str(),
        )
        .replace(
            "${assets_root}",
            format!(
                "\"{}\"",
                &launchArgs
                    .assetsDir
                    .canonicalize()
                    .unwrap()
                    .to_str()
                    .unwrap()
            )
            .as_str(),
        )
        .replace(
            "${assets_index_name}",
            format!(
                "\"{}\"",
                &manifest.assets
            )
            .as_str(),
        ).replace(
            "${auth_access_token}",
            &launchArgs
                    .accessToken.as_ref().or(Some(&String::from("offline"))).unwrap(),
        ).replace(
            "--uuid ${auth_uuid}",
            "",
        );
    let java_args = manifest
        .arguments
        .jvm
        .iter()
        .filter_map(|it| {
            if let Argument::Simple(s) = it {
                return Some(s.clone());
            } else {
                return None;
            }
        })
        .collect::<Vec<String>>()
        .join(" ")
        .replace(
            "${natives_directory}",
            format!(
                "\"{}\"",
                &launchArgs
                    .nativesPath
                    .canonicalize()
                    .unwrap()
                    .to_str()
                    .unwrap()
            )
            .as_str(),
        )
        .replace(
            "${classpath}",
            format!(
                "\"{}\"",
                (manifest
                    .libraries
                    .iter()
                    .map(|it| {
                        launchArgs
                            .libraryPath
                            .canonicalize()
                            .unwrap()
                            .join(&it.downloads.artifact.as_ref().unwrap().path).to_str().unwrap().to_string()
                    })
                    .collect::<Vec<String>>()
                    .join(":")
                    + ":"
                    + &launchArgs.jarPath.canonicalize().unwrap().to_str().unwrap())
                    .as_str()
            )
            .as_str(),
        );
    // println!("{}", minecraft_args);
    // println!("{}", java_args);
    let command = format!(
        "{} {} {}",
        &java_args, &manifest.mainClass, &minecraft_args
    );
    println!("{}", &command);
    let mut cmd = Command::new(&launchArgs.javaPath);
    cmd.args(shell_words::split(&command).unwrap());
    cmd.current_dir(&launchArgs.gameDir);
    cmd.spawn().unwrap().wait().unwrap();
}

pub struct LaunchArgs {
    pub javaPath: String,
    pub jarPath: PathBuf,
    pub assetsDir: PathBuf,
    pub gameDir: PathBuf,
    pub libraryPath: PathBuf,
    pub nativesPath: PathBuf,
    pub username: String,
    pub accessToken: Option<String>
}
