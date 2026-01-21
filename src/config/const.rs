/// Path to the configuration file.
pub const CONFIG_PATH: &str = "/home/.git_helper/config.json";

/// Package name constant.
pub(crate) const PACKAGE_NAME: &str = "gtl";

/// Package version constant.
pub(crate) const PACKAGE_VERSION: &str = "0.1.6";

/// Maximum number of retries for git operations.
pub(crate) const MAX_RETRIES: u32 = 6;

/// Delay between retries in seconds.
pub(crate) const RETRY_DELAY_SECS: u64 = 2;
