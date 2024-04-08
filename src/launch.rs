use std::{
    path::PathBuf,
    process::Command,
};

use crate::meta_parsing::{Argument, LauncherMeta};

pub fn launch_minecraft(manifest: &LauncherMeta, launch_args: &LaunchArgs) {
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
        .replace("${auth_player_name}", &launch_args.username)
        .replace(
            "${game_directory}",
            format!(
                "\"{}\"",
                &launch_args.game_dir.canonicalize().unwrap().to_str().unwrap()
            )
            .as_str(),
        )
        .replace(
            "${assets_root}",
            format!(
                "\"{}\"",
                &launch_args
                    .assets_dir
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
            &launch_args
                    .access_token.as_ref().or(Some(&String::from("offline"))).unwrap(),
        ).replace(
            "--uuid ${auth_uuid}",
            "",
        ).replace(
            "${version_type}",
            &launch_args
                    .version_type.as_ref().or(Some(&String::from("release"))).unwrap(),
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
                &launch_args
                    .natives_dir
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
                        launch_args
                            .library_dir
                            .canonicalize()
                            .unwrap()
                            .join(&it.downloads.artifact.as_ref().unwrap().path).to_str().unwrap().to_string()
                    })
                    .collect::<Vec<String>>()
                    .join(":")
                    + ":"
                    + &launch_args.jar_path.canonicalize().unwrap().to_str().unwrap())
                    .as_str()
            )
            .as_str(),
        );
    // println!("{}", minecraft_args);
    // println!("{}", java_args);
    let command = format!(
        "{} {} {}",
        &java_args, &manifest.main_class, &minecraft_args
    );
    println!("{}", &command);
    let mut cmd = Command::new(&launch_args.java_path);
    cmd.args(shell_words::split(&command).unwrap());
    cmd.current_dir(&launch_args.game_dir);
    cmd.spawn().unwrap().wait().unwrap();
}

pub struct LaunchArgs {
    pub java_path: String,
    pub jar_path: PathBuf,
    pub assets_dir: PathBuf,
    pub game_dir: PathBuf,
    pub library_dir: PathBuf,
    pub natives_dir: PathBuf,
    pub username: String,
    pub access_token: Option<String>,
    pub version_type: Option<String>
}
