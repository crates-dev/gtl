use crate::*;

pub fn init() {
    Command::new("git")
        .arg("init")
        .status()
        .expect("Failed to execute git init");
}

pub fn config_global_add_safe_directory(path: &str) {
    Command::new("git")
        .args([
            "config",
            "--global",
            "--add",
            "safe.directory",
            &format!("'{}'", path),
        ])
        .status()
        .expect("Failed to execute git config --global --add safe.directory './'");
}

pub fn config_advice_add_ignored_file_false() {
    Command::new("git")
        .args(["config", "advice.addIgnoredFile", "false"])
        .status()
        .expect("Failed to execute git config advice.addIgnoredFile false");
}

pub fn remote_add(remote: &Remote) {
    Command::new("git")
        .arg("remote")
        .arg("add")
        .arg(&remote.name)
        .arg(&remote.url)
        .status()
        .expect("Failed to add remote");
}

pub fn add_all() {
    Command::new("git")
        .args(["add", "*"])
        .status()
        .expect("Failed to add *");
}

pub fn commit(msg: &str) {
    Command::new("git")
        .args(["commit", "-m", msg])
        .status()
        .expect(&format!("Failed to commit -m {}", msg));
}

pub fn push(remote: &str) {
    Command::new("git")
        .args(["push", remote])
        .status()
        .expect("Failed to push to remote");
}

pub fn help() {
    let get_package_name: &str = get_package_name();
    println!(
        "{} extension usage: {} acp\n",
        get_package_name, get_package_name
    );
    Command::new("git")
        .arg("help")
        .status()
        .expect("Failed to run help");
}

pub fn version() {
    println!("{} version: {}", get_package_name(), get_package_version());
}

pub fn other(args: &Vec<OsString>) {
    let status: ExitStatus = Command::new("git")
        .args(args)
        .status()
        .expect("Failed to execute git command");
    exit(status.code().unwrap_or_default());
}
