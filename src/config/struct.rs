use crate::*;

/// Remote repository configuration.
#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct Remote {
    /// Remote name.
    pub(crate) name: String,
    /// Remote URL.
    pub(crate) url: String,
}

#[derive(Deserialize)]
pub(crate) struct CargoToml {
    pub(crate) package: Package,
}

#[derive(Deserialize)]
pub(crate) struct Package {
    pub(crate) version: String,
}
