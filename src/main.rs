pub(crate) mod cmd;
pub(crate) mod config;
pub(crate) use crate::{cmd::git, config::r#type::*};
pub(crate) use config::{constant::*, func::*};
pub(crate) use serde::Deserialize;
pub(crate) use serde::Serialize;
pub(crate) use std::{
    collections::HashMap,
    env,
    ffi::OsString,
    fs,
    io::{self, Write},
    path::{Path, PathBuf},
    process::{exit, Command, ExitStatus},
};

pub(crate) static PACKAGE_NAME: &str = "gtl";
pub(crate) static PACKAGE_VERSION: &str = "0.1.4";

pub(crate) fn get_package_name() -> &'static str {
    PACKAGE_NAME
}

pub(crate) fn get_package_version() -> &'static str {
    PACKAGE_VERSION
}

fn main() {
    let args: Vec<_> = env::args_os().skip(1).collect();
    if args.is_empty() {
        eprintln!("Usage: {} help", get_package_name());
        exit(1);
    }
    let config: Config = read_config(CONFIG_PATH);
    let args_first: OsString = args[0].clone();
    if args_first == OsString::from("init") {
        init_repository(&config);
    } else if args_first == OsString::from("push") {
        push_to_all_remotes(&config);
    } else if args_first == OsString::from("acp") {
        add_commit_push_to_all_remotes(&config);
    } else if args_first == OsString::from("help") {
        git::help();
    } else if args_first == OsString::from("version") || args_first == OsString::from("-v") {
        git::version();
    } else {
        git::other(&args);
    }
}

fn init_repository(config: &Config) {
    git::init();
    git::config_global_add_safe_directory("./");
    git::config_advice_add_ignored_file_false();
    let current_dir: PathBuf = std::env::current_dir().unwrap();
    let current_path: &str = current_dir.to_str().unwrap();
    if let Some(remotes) = config.get(current_path) {
        for remote in remotes {
            git::remote_add(remote);
        }
    }
}

fn push_to_all_remotes(config: &Config) {
    let current_dir: PathBuf = std::env::current_dir().unwrap();
    let current_path: &str = current_dir.to_str().unwrap();
    if let Some(remotes) = config.get(current_path) {
        for remote in remotes {
            git::push(&remote.name);
        }
    }
}

fn add_commit_push_to_all_remotes(config: &Config) {
    let current_dir: PathBuf = std::env::current_dir().unwrap();
    let current_path: &str = current_dir.to_str().unwrap();
    io::stdout().flush().unwrap();
    let mut commit_msg: String = String::new();
    io::stdin().read_line(&mut commit_msg).unwrap();
    let commit_msg: &str = commit_msg.trim();
    git::add_all();
    git::commit(commit_msg);
    if let Some(remotes) = config.get(current_path) {
        for remote in remotes {
            git::push(&remote.name);
        }
    }
}
