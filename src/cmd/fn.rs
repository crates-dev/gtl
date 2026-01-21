use crate::*;

/// Initializes a new git repository.
pub(crate) fn init() {
    Command::new("git")
        .arg("init")
        .status()
        .expect("Failed to execute git init");
}

/// Adds a directory to git's safe directory list globally.
///
/// # Arguments
///
/// - `&str` - The path to add as safe directory.
pub(crate) fn config_global_add_safe_directory(path: &str) {
    Command::new("git")
        .args([
            "config",
            "--global",
            "--add",
            "safe.directory",
            &format!("'{path}'"),
        ])
        .status()
        .expect("Failed to execute git config --global --add safe.directory './'");
}

/// Disables git's advice about ignored files.
pub(crate) fn config_advice_add_ignored_file_false() {
    Command::new("git")
        .args(["config", "advice.addIgnoredFile", "false"])
        .status()
        .expect("Failed to execute git config advice.addIgnoredFile false");
}

/// Adds a new git remote.
///
/// # Arguments
///
/// - `&Remote` - The remote configuration to add.
pub(crate) fn remote_add(remote: &Remote) {
    Command::new("git")
        .arg("remote")
        .arg("add")
        .arg(&remote.name)
        .arg(&remote.url)
        .status()
        .expect("Failed to add remote");
}

/// Stages all changes in the git repository.
pub(crate) fn add_all() {
    Command::new("git")
        .args(["add", "*"])
        .status()
        .expect("Failed to add *");
}

/// Commits staged changes with a message.
///
/// # Arguments
///
/// - `&str` - The commit message.
pub(crate) fn commit(msg: &str) {
    Command::new("git")
        .args(["commit", "-m", msg])
        .status()
        .unwrap_or_else(|_| panic!("Failed to commit -m {msg}"));
}

/// Pushes changes to a git remote.
///
/// # Arguments
///
/// - `&str` - The remote name to push to.
pub(crate) fn push(remote: &str) {
    Command::new("git")
        .args(["push", remote])
        .status()
        .expect("Failed to push to remote");
}

/// Displays git help information.
pub(crate) fn help() {
    let get_package_name: &str = get_package_name();
    println!("{get_package_name} extension usage: {get_package_name} acp\n");
    Command::new("git")
        .arg("help")
        .status()
        .expect("Failed to run help");
}

/// Displays package version information.
pub(crate) fn version() {
    println!("{} version: {}", get_package_name(), get_package_version());
}

/// Executes arbitrary git command with arguments.
///
/// # Arguments
///
/// - `&Vec<OsString>` - The command line arguments.
pub(crate) fn other(args: &Vec<OsString>) {
    let status: ExitStatus = Command::new("git")
        .args(args)
        .status()
        .expect("Failed to execute git command");
    exit(status.code().unwrap_or_default());
}

/// Publishes the crate to crates.io with retries.
pub(crate) fn publish_package() {
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
pub(crate) fn init_repository(config: &Config) {
    init();
    config_global_add_safe_directory("./");
    config_advice_add_ignored_file_false();
    let current_dir: PathBuf = std::env::current_dir().unwrap();
    let current_path: &str = current_dir.to_str().unwrap();
    if let Some(remotes) = config.get(current_path) {
        for remote in remotes {
            remote_add(remote);
        }
    }
}

/// Pushes to all configured git remotes.
///
/// # Arguments
///
/// - `&Config` - The configuration containing remotes.
pub(crate) fn push_to_all_remotes(config: &Config) {
    let current_dir: PathBuf = std::env::current_dir().unwrap();
    let current_path: &str = current_dir.to_str().unwrap();
    if let Some(remotes) = config.get(current_path) {
        for remote in remotes {
            push(&remote.name);
        }
    }
}

/// Generates a commit message automatically based on Cargo.toml version or timestamp.
///
/// # Returns
///
/// - `String` - The generated commit message.
pub(crate) fn generate_auto_commit_message() -> String {
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
pub(crate) fn add_commit_auto_push(config: &Config) {
    add_all();
    let commit_msg = generate_auto_commit_message();
    commit(&commit_msg);
    push_to_all_remotes(config);
}

/// Adds, commits and pushes to all configured git remotes.
///
/// # Arguments
///
/// - `&Config` - The configuration containing remotes.
pub(crate) fn add_commit_push_to_all_remotes(config: &Config) {
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
    add_all();
    commit(&commit_msg);
    if let Some(remotes) = config.get(current_path) {
        for remote in remotes {
            push(&remote.name);
        }
    }
}
