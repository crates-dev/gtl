//! gtl
//!
//! gtl is a Git-based tool designed to simplify the management
//! of multiple remote repositories. It extends Git's functionality
//! by providing one-click initialization and pushing to multiple
//! remote repositories, making it especially useful for developers
//! who need to maintain multiple remote repositories simultaneously.

mod cmd;
mod config;

pub(crate) use {cmd::*, config::*};

pub(crate) use std::{
    collections::HashMap,
    env,
    ffi::OsString,
    fs,
    io::{self, Write},
    path::{Path, PathBuf},
    process::{Command, ExitStatus, exit},
};

pub(crate) use {chrono::Local, serde::Deserialize, serde::Serialize};

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
        help();
    } else if args_first == "-v" || args_first == "version" || args_first == "--version" {
        version();
    } else {
        other(&args);
    }
}
