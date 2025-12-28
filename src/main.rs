//! gtl
//!
//! gtl is a Git-based tool designed to simplify the management
//! of multiple remote repositories. It extends Git's functionality
//! by providing one-click initialization and pushing to multiple
//! remote repositories, making it especially useful for developers
//! who need to maintain multiple remote repositories simultaneously.

pub(crate) mod cmd;
pub(crate) mod config;
pub(crate) use crate::{cmd::git, config::r#type::*};
pub(crate) use chrono::Local;
pub(crate) use config::{r#const::*, func::*};
pub(crate) use serde::Deserialize;
pub(crate) use serde::Serialize;
pub(crate) use std::{
    collections::HashMap,
    env,
    ffi::OsString,
    fs,
    io::{self, Write},
    path::{Path, PathBuf},
    process::{Command, ExitStatus, exit},
};

/// Package name constant.
pub(crate) const PACKAGE_NAME: &str = "gtl";

/// Package version constant.
pub(crate) const PACKAGE_VERSION: &str = "0.1.6";

#[derive(Deserialize)]
struct CargoToml {
    package: Package,
}

#[derive(Deserialize)]
struct Package {
    version: String,
}

/// Gets the package name.
///
/// # Returns
///
/// - `&'static str` - The package name.
pub(crate) fn get_package_name() -> &'static str {
    PACKAGE_NAME
}

/// Gets the package version.
///
/// # Returns
///
/// - `&'static str` - The package version.
pub(crate) fn get_package_version() -> &'static str {
    PACKAGE_VERSION
}

/// Main entry point of the application.
///
/// # Arguments
///
/// - `Vec<OsString>` - Command line arguments.
fn main() {
    let args: Vec<_> = env::args_os().skip(1).collect();
    if args.is_empty() {
        eprintln!("Usage: {} help", get_package_name());
        exit(1);
    }
    let config: Config = read_config(CONFIG_PATH);
    let args_first: OsString = args[0].clone();
    if args_first == "init" {
        init_repository(&config);
    } else if args_first == "push" {
        push_to_all_remotes(&config);
    } else if args_first == "acp" {
        add_commit_push_to_all_remotes(&config);
    } else if args_first == "pacp" {
        publish_package();
        add_commit_auto_push(&config);
    } else if args_first == "help" {
        git::help();
    } else if args_first == "-v"
        || args_first == "version"
        || args_first == "--version"
    {
        git::version();
    } else {
        git::other(&args);
    }
}

/// Publishes the crate to crates.io with retries.
fn publish_package() {
    const MAX_RETRIES: u32 = 6;
    const RETRY_DELAY_SECS: u64 = 2;

    for attempt in 1..=MAX_RETRIES {
        let status = Command::new("cargo")
            .args(["publish", "--allow-dirty"])
            .status();

        match status {
            Ok(exit_status) if exit_status.success() => {
                println!("Successfully published package.");
                return;
            }
            Ok(exit_status) => {
                eprintln!(
                    "Attempt {attempt} failed with status: {exit_status}. Retrying in {RETRY_DELAY_SECS} seconds..."
                );
            }
            Err(e) => {
                eprintln!(
                    "Attempt {attempt} failed with error: {e}. Retrying in {RETRY_DELAY_SECS} seconds..."
                );
            }
        }

        if attempt < MAX_RETRIES {
            std::thread::sleep(std::time::Duration::from_secs(RETRY_DELAY_SECS));
        }
    }

    panic!("Failed to publish package after {MAX_RETRIES} attempts.");
}

/// Initializes a git repository with configuration.
///
/// # Arguments
///
/// - `&Config` - The configuration to use.
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

/// Pushes to all configured git remotes.
///
/// # Arguments
///
/// - `&Config` - The configuration containing remotes.
fn push_to_all_remotes(config: &Config) {
    let current_dir: PathBuf = std::env::current_dir().unwrap();
    let current_path: &str = current_dir.to_str().unwrap();
    if let Some(remotes) = config.get(current_path) {
        for remote in remotes {
            git::push(&remote.name);
        }
    }
}

/// Generates a commit message automatically based on Cargo.toml version or timestamp.
///
/// # Returns
///
/// - `String` - The generated commit message.
fn generate_auto_commit_message() -> String {
    match fs::read_to_string("Cargo.toml") {
        Ok(content) => {
            if let Ok(cargo_toml) = toml::from_str::<CargoToml>(&content) {
                format!("feat: v{}", cargo_toml.package.version)
            } else {
                let now = Local::now();
                format!("feat: {}", now.format("%Y-%m-%d %H:%M:%S"))
            }
        }
        Err(_) => {
            let now = Local::now();
            format!("feat: {}", now.format("%Y-%m-%d %H:%M:%S"))
        }
    }
}

/// Adds, commits with an auto-generated message, and pushes to all remotes.
///
/// # Arguments
///
/// - `&Config` - The configuration containing remotes.
fn add_commit_auto_push(config: &Config) {
    git::add_all();
    let commit_msg = generate_auto_commit_message();
    git::commit(&commit_msg);
    push_to_all_remotes(config);
}

/// Adds, commits and pushes to all configured git remotes.
///
/// # Arguments
///
/// - `&Config` - The configuration containing remotes.
fn add_commit_push_to_all_remotes(config: &Config) {
    let current_dir: PathBuf = std::env::current_dir().unwrap();
    let current_path: &str = current_dir.to_str().unwrap();
    io::stdout().flush().unwrap();
    let mut commit_msg_input: String = String::new();
    io::stdin().read_line(&mut commit_msg_input).unwrap();
    let commit_msg_trimmed: &str = commit_msg_input.trim();
    let commit_msg = if commit_msg_trimmed.is_empty() {
        generate_auto_commit_message()
    } else {
        commit_msg_trimmed.to_string()
    };
    git::add_all();
    git::commit(&commit_msg);
    if let Some(remotes) = config.get(current_path) {
        for remote in remotes {
            git::push(&remote.name);
        }
    }
}
