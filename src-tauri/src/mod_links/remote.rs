use crate::mod_links::local::{ModDependencies, ModLink, ModTags};
use serde::{self, Deserialize, Serialize};

/// The manifest object containing data about an individual mod;
/// Fetched remotely from GitHub
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct RemoteModManifest {
    #[serde(rename = "Name", default)]
    pub name: String,
    #[serde(rename = "Description", default)]
    pub description: String,
    #[serde(rename = "Version", default)]
    pub version: String,
    #[serde(rename = "Link")]
    pub link: ModLink,
    #[serde(rename = "Dependencies")]
    pub dependencies: ModDependencies,
    #[serde(rename = "Repository")]
    pub repository: String,
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<ModTags>,
    #[serde(skip_deserializing, rename = "Enabled")]
    pub enabled: bool,
    #[serde(skip_deserializing, rename = "Installed")]
    pub installed: bool,
}

/// The main mod links object fetched from GitHub
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct RemoteModLinks {
    #[serde(rename = "Manifest", default)]
    pub manifests: Vec<RemoteModManifest>,
}

impl RemoteModLinks {
    /// Create a new instance of a remote mod links object
    pub fn new() -> RemoteModLinks {
        RemoteModLinks {
            manifests: vec![],
        }
    }
}