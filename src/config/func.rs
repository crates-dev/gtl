use crate::*;

/// Reads configuration from a file, creates default if not exists.
///
/// # Arguments
///
/// - `P: AsRef<Path>` - The path to the configuration file.
///
/// # Returns
///
/// - `Config` - The parsed configuration.
pub fn read_config<P: AsRef<Path>>(path: P) -> Config {
    let path_ref: &Path = path.as_ref();
    if !path_ref.exists() {
        if let Some(parent) = path_ref.parent() {
            fs::create_dir_all(parent).expect("Unable to create directories");
        }
        let empty_config: Config = HashMap::new();
        let data: String =
            serde_json::to_string(&empty_config).expect("Unable to serialize empty config");
        fs::write(&path, data).expect("Unable to write empty config file");
    }
    let data: String = fs::read_to_string(path).expect("Unable to read config file");
    serde_json::from_str(&data).expect("Unable to parse config file")
}
