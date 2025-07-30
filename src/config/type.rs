use crate::*;

/// Configuration type mapping paths to remote configurations.
pub type Config = HashMap<String, Vec<Remote>>;

/// Remote repository configuration.
#[derive(Serialize, Deserialize, Debug)]
pub struct Remote {
    /// Remote name.
    pub name: String,
    /// Remote URL.
    pub url: String,
}
