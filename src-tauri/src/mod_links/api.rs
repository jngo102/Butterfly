use serde::{self, Deserialize, Serialize};

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