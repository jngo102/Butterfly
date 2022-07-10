use serde::{self, Deserialize, Serialize};

/// The object listing all the dependencies of a mod
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ModDependencies {
    #[serde(rename = "Dependency", default)]
    pub dependencies: Vec<String>,
}

/// A mod link item containing hash and URL data
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ModLink {
    #[serde(rename = "SHA256", default)]
    pub sha256: String,
    #[serde(rename = "$value", default)]
    pub link: String,
}

/// The object listing all the tags of a mod
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ModTags {
    #[serde(rename = "Tag", default)]
    pub tags: Vec<String>,
}

/// The manifest object containing data about an individual mod;
/// local to settings file
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct LocalModManifest {
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
    #[serde(rename = "Enabled")]
    pub enabled: bool,
    #[serde(rename = "Installed")]
    pub installed: bool,
}

/// The main mod links object loaded from settings file;
/// local to settings file
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct LocalModLinks {
    #[serde(rename = "Manifest", default)]
    pub manifests: Vec<LocalModManifest>,
}

impl Default for LocalModLinks {
    fn default() -> Self {
        LocalModLinks {
            manifests: vec![],
        }
    }
}

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

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct ApiLink { 
    #[serde(rename = "SHA256", default)] 
    pub sha256: String,
    #[serde(rename = "$value", default)] 
    pub link: String 
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct ApiPlatformLinks {
    #[serde(rename = "Linux")]
    pub linux: ApiLink,
    #[serde(rename = "Mac")]
    pub mac: ApiLink,
    #[serde(rename = "Windows")]
    pub windows: ApiLink,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct ApiFiles {
    #[serde(rename = "File")]
    pub files: Vec<String>,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct ApiManifest {
    #[serde(rename = "Version", default)]
    pub version: String,
    #[serde(rename = "Links")]
    pub links: ApiPlatformLinks,
    #[serde(rename = "Files")]
    pub files: ApiFiles,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct ApiLinks {
    #[serde(rename = "Manifest")]
    pub manifest: ApiManifest
}

impl ApiLinks {
    /// Create a new instance of an api links object
    pub fn new() -> ApiLinks {
        ApiLinks {
            manifest: ApiManifest {
                version: "".to_string(),
                links: ApiPlatformLinks {
                    linux: ApiLink {
                        sha256: "".to_string(),
                        link: "".to_string(),
                    },
                    mac: ApiLink {
                        sha256: "".to_string(),
                        link: "".to_string(),
                    },
                    windows: ApiLink {
                        sha256: "".to_string(),
                        link: "".to_string(),
                    }
                },
                files: ApiFiles {
                    files: vec![],
                }
            },
        }
    }
}