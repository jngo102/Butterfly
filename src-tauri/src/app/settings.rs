use crate::app::profile::Profile;
use crate::mod_links::local::LocalModLinks;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Settings {
    #[serde(rename = "Current Profile")]
    pub current_profile: String,
    #[serde(rename = "Language")]
    pub language: String,
    #[serde(rename = "Mods Path")]
    pub mods_path: String,
    #[serde(rename = "Mod Links")]
    pub mod_links: LocalModLinks,
    #[serde(rename = "Profiles")]
    pub profiles: Vec<Profile>,
    #[serde(rename = "Theme")]
    pub theme: String,
    #[serde(rename = "Theme Path")]
    pub theme_path: String,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            current_profile: "".to_string(),
            mods_path: "".to_string(),
            language: "English".to_string(),
            mod_links: LocalModLinks::default(),
            profiles: vec![],
            theme: "Dark".to_string(),
            theme_path: "".to_string(),
        }
    }
}