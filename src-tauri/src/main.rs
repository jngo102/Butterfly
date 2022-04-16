#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use async_recursion::async_recursion;
use directories::BaseDirs;
use futures_util::StreamExt;
use lazy_static::lazy_static;
use log::{error, info, warn, LevelFilter};
use native_dialog::{FileDialog, MessageDialog, MessageType};
use reqwest;
use serde;
use serde::{Deserialize, Serialize};
use serde_json;
use serde_json::{json, Value};
use sha256::digest_file;
use simple_logging;
use std::cmp::min;
use std::env;
use std::fs;
use std::fs::File;
use std::io::{Cursor, Read, Write};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::str::FromStr;
use tokio;
use tokio::sync::RwLock;
use unzip::Unzipper;

/// An array of possible paths to the folder containing the Hollow Knight executable
static STATIC_PATHS: [&str; 6] = [
    "Program Files/Steam/steamapps/common/Hollow Knight",
    "Program Files (x86)/Steam/steamapps/common/Hollow Knight",
    "Program Files/GOG Galaxy/Games/Hollow Knight",
    "Program Files (x86)/GOG Galaxy/Games/Hollow Knight",
    "Steam/steamapps/common/Hollow Knight",
    "GOG Galaxy/Games/Hollow Knight",
];

/// An array of possible path suffixes to the Hollow Knight path's Managed folder
static SUFFIXES: [&str; 3] = [
    // GOG
    "Hollow Knight_Data/Managed",
    // Steam
    "hollow_knight_Data/Managed",
    // Mac
    "Contents/Resources/Data/Managed",
];

/// The object listing all the dependencies of a mod
#[derive(Debug, Deserialize, PartialEq, Serialize)]
struct ModDependencies {
    #[serde(rename = "Dependency", default)]
    dependencies: Vec<String>,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
struct ModLink {
    #[serde(rename = "SHA256", default)]
    sha256: String,
    #[serde(rename = "$value", default)]
    link: String,
}

/// The manifest object containing data about an individual mod
#[derive(Debug, Deserialize, PartialEq, Serialize)]
struct ModManifest {
    #[serde(rename = "Name", default)]
    name: String,
    #[serde(rename = "Description", default)]
    description: String,
    #[serde(rename = "Version", default)]
    version: String,
    #[serde(rename = "Link")]
    link: ModLink,
    #[serde(rename = "Dependencies")]
    dependencies: ModDependencies,
}

/// The main mod links object
#[derive(Debug, Deserialize, PartialEq, Serialize)]
struct ModLinks {
    #[serde(rename = "Manifest", default)]
    manifests: Vec<ModManifest>,
}

impl ModLinks {
    /// Create a new instance of a mod links object
    fn new() -> ModLinks {
        ModLinks {
            manifests: Vec::new(),
        }
    }
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
struct ApiLink { 
    #[serde(rename = "SHA256", default)] 
    sha256: String,
    #[serde(rename = "$value", default)] 
    link: String 
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
struct ApiPlatformLinks {
    #[serde(rename = "Linux")]
    linux: ApiLink,
    #[serde(rename = "Mac")]
    mac: ApiLink,
    #[serde(rename = "Windows")]
    windows: ApiLink,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
struct ApiFiles {
    #[serde(rename = "File")]
    files: Vec<String>,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
struct ApiManifest {
    #[serde(rename = "Version", default)]
    version: String,
    #[serde(rename = "Links")]
    links: ApiPlatformLinks,
    #[serde(rename = "Files")]
    files: ApiFiles,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
struct ApiLinks {
    #[serde(rename = "Manifest")]
    manifest: ApiManifest
}

impl ApiLinks {
    /// Create a new instance of an api links object
    fn new() -> ApiLinks {
        ApiLinks {
            manifest: ApiManifest {
                version: String::from(""),
                links: ApiPlatformLinks {
                    linux: ApiLink {
                        sha256: String::from(""),
                        link: String::from(""),
                    },
                    mac: ApiLink {
                        sha256: String::from(""),
                        link: String::from(""),
                    },
                    windows: ApiLink {
                        sha256: String::from(""),
                        link: String::from(""),
                    }
                },
                files: ApiFiles {
                    files: Vec::new(),
                }
            },
        }
    }
}

lazy_static! {
    /// The path to the output log, written to for debugging purposes
    static ref LOG_PATH: RwLock<String> = RwLock::new(String::new());
    /// The JSON object of data about mods, stringified
    static ref MODS_JSON: RwLock<String> = RwLock::new(String::new());
    /// The path to the Mods folder in the Hollow Knight game folder
    static ref MODS_PATH: RwLock<String> = RwLock::new(String::new());
    /// The path to the settings JSON file
    static ref SETTINGS_PATH: RwLock<String> = RwLock::new(String::new());
    /// The settings JSON objet
    static ref SETTINGS_JSON: RwLock<Value> = RwLock::new(json!(null));
    /// Current download progress
    static ref CURRENT_DOWNLOAD_PROGRESS: RwLock<u8> = RwLock::new(0);
}

#[tokio::main]
async fn main() {
    load_or_create_files().await;
    auto_detect().await;
    load_mod_list().await;
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            check_api_installed,
            check_for_update,
            create_profile,
            debug,
            delete_profile,
            disable_mod,
            enable_mod,
            export_profiles,
            fetch_current_download_progress,
            fetch_current_profile,
            fetch_enabled_mods,
            fetch_installed_mods,
            fetch_language,
            fetch_manually_installed_mods,
            fetch_mod_list,
            fetch_profiles,
            fetch_theme_data,
            import_profiles,
            install_mod,
            manually_install_mod,
            open_mods_folder,
            reset_settings,
            set_language,
            set_profile,
            set_theme,
            toggle_api,
            uninstall_mod,
        ])
        .run(tauri::generate_context!())
        .expect("Failed to run tauri application.");
}

/// Check and return whether the Modding API has been installed
#[tauri::command]
async fn check_api_installed() -> bool {
    let mods_path = MODS_PATH.read().await;
    let managed_path: PathBuf = [mods_path.as_str(), ".."].iter().collect();
    let vanilla_assembly: PathBuf = [managed_path.to_str().unwrap(), "Assembly-CSharp.dll.vanilla"].iter().collect();
    let modded_assembly: PathBuf = [managed_path.to_str().unwrap(), "Assembly-CSharp.dll.modded"].iter().collect();
    vanilla_assembly.exists() && !modded_assembly.exists()
}

/// Check if an installed mod is out of date
/// # Arguments
/// * `mod_name` - The name of the mod to check
/// * `current_mod_version` - The version number that is stored on mod links
#[tauri::command]
async fn check_for_update(mod_name: String, current_mod_version: String) -> bool {

    let settings_json = SETTINGS_JSON.read().await;
    let installed_mods = settings_json["InstalledMods"].as_array().unwrap();
    if installed_mods.len() <= 0 {
        return false;
    }

    let mut stored_mod_version = String::from("");
    for install in installed_mods {
        let stored_mod_name = String::from(install["Name"].as_str().unwrap());
        if stored_mod_name == mod_name {
            stored_mod_version = String::from(install["Version"].as_str().unwrap());
        }
    }

    stored_mod_version != "" && stored_mod_version != current_mod_version
}

/// Create a new profile and save it to settings
/// # Arguments
/// * `profile_name` - The name of the new profile
/// * `mod_names` - The name of the mods that will be included in the profile
#[tauri::command]
async fn create_profile(profile_name: String, mod_names: Vec<String>) {
    let mut settings_json = SETTINGS_JSON.write().await;
    
    let current_profile = String::from(settings_json["CurrentProfile"].as_str().unwrap());

    let installed_mods = settings_json["InstalledMods"].as_array().unwrap();

    let language = String::from(settings_json["Language"].as_str().unwrap());

    let mods_path = String::from(settings_json["ModsPath"].as_str().unwrap());    

    let profiles_value = &mut settings_json.clone()["Profiles"];
    let profiles = profiles_value.as_array_mut().unwrap();
    profiles.push(json!({"Name": profile_name, "Mods": mod_names}));
    
    let theme = String::from(settings_json["Theme"].as_str().unwrap());
    let theme_path = String::from(settings_json["ThemePath"].as_str().unwrap());

    *settings_json = json!({
        "CurrentProfile": current_profile,
        "InstalledMods": installed_mods,
        "Language": language,
        "ModsPath": mods_path,
        "Profiles": profiles,
        "Theme": theme,
        "ThemePath": theme_path,
    });

    let settings_path = SETTINGS_PATH.read().await;
    if PathBuf::from_str(settings_path.as_str()).unwrap().exists() {
        let settings_file = File::options().write(true).open(settings_path.as_str()).unwrap();
        match serde_json::to_writer_pretty(settings_file, &*settings_json) {
            Ok(_) => info!("Successfully added new profile to settings file."),
            Err(e) => error!("Failed to write new profile to settings file: {}", e),
        }
    }
}

/// A tauri command that may be invoked from TypeScript for debugging purposes
/// # Arguments
/// * `msg` - The message to send from TypeScript
#[tauri::command]
fn debug(msg: String) {
    info!("[DEBUG]\n\t\t{}", msg);
}

/// Delete a profile from settings
/// # Arguments
/// * `profile_name` - The name of the profile to be deleted
#[tauri::command]
async fn delete_profile(profile_name: String) {
    let mut settings_json = SETTINGS_JSON.write().await;
    let mut current_profile = String::from("");
    let mods_path = String::from(settings_json["ModsPath"].as_str().unwrap());
    if settings_json["CurrentProfile"] != profile_name {
        current_profile = String::from(settings_json["CurrentProfile"].as_str().unwrap());
    }

    let installed_mods = settings_json["InstalledMods"].as_array().unwrap();

    let language = String::from(settings_json["Language"].as_str().unwrap());

    let profiles_value = &mut settings_json.clone()["Profiles"];
    let profiles = profiles_value.as_array_mut().unwrap();
    for i in 0..profiles.len() {
        let stored_profile_name = String::from(profiles[i]["Name"].as_str().unwrap());
        if stored_profile_name == profile_name {
            profiles.remove(i);
        }
    }

    let theme = String::from(settings_json["Theme"].as_str().unwrap());
    let theme_path = String::from(settings_json["ThemePath"].as_str().unwrap());

    *settings_json = json!({
        "CurrentProfile": current_profile,
        "InstalledMods": installed_mods,
        "Language": language,
        "ModsPath": mods_path,
        "Profiles": profiles,
        "Theme": theme,
        "ThemePath": theme_path,
    });

    let settings_path = SETTINGS_PATH.read().await;
    if PathBuf::from_str(settings_path.as_str()).unwrap().exists() {
        let settings_file = File::options().write(true).open(settings_path.as_str()).unwrap();
        match serde_json::to_writer_pretty(settings_file, &*settings_json) {
            Ok(_) => info!("Successfully removed profile from settings file."),
            Err(e) => error!("Failed to remove profile from settings file: {}", e),
        }
    }
}

/// Move a mod folder into the Disabled folder if it is located in the Mods folder
/// # Argumentz`
/// `mod_name` - The name of the mod folder to be moved into the Disabled folder
#[tauri::command]
async fn disable_mod(mod_name: String) {
    info!("Disabling mod {:?}", mod_name);
    let mods_path = MODS_PATH.read().await;
    let mod_path: PathBuf = [mods_path.to_string(), mod_name.clone()].iter().collect();
    let disabled_mods_path: PathBuf = [mods_path.to_string(), String::from("Disabled")]
        .iter()
        .collect();
    let disabled_mod_path: PathBuf = [
        mods_path.to_string(),
        String::from("Disabled"),
        mod_name.clone(),
    ]
    .iter()
    .collect();
    if !disabled_mods_path.exists() {
        match fs::create_dir(disabled_mods_path.as_path()) {
            Ok(_) => info!("Successfully created Disabled folder."),
            Err(e) => error!("Failed to create Disabled folder: {}", e),
        }
    }
    if mod_path.exists() {
        match fs::rename(mod_path.as_path(), disabled_mod_path) {
            Ok(_) => info!("Successfully moved mod {} to Disabled folder.", mod_name),
            Err(e) => error!(
                "Failed to move mod directory {:?} to Disabled: {}",
                mod_path.to_str().unwrap(),
                e
            ),
        }
    } else {
        warn!(
            "Path {:?} does not exist.",
            mod_path.to_str().unwrap()
        );
    }
}

/// Move a mod folder out of the Disabled folder if it is there
/// # Arguments
/// * `mod_name` - The name of the mod folder to move out of the Disabled folder
#[tauri::command]
async fn enable_mod(mod_name: String) {
    info!("Enabling mod {:?}", mod_name);
    let mods_path = MODS_PATH.read().await;
    let mod_path: PathBuf = [mods_path.to_string(), mod_name.clone()].iter().collect();
    let disabled_mod_path: PathBuf = [
        mods_path.to_string(),
        String::from("Disabled"),
        mod_name.clone(),
    ]
    .iter()
    .collect();
    if disabled_mod_path.exists() {
        match fs::rename(disabled_mod_path.as_path(), mod_path.as_path()) {
            Ok(_) => info!(
                "Successfully moved mod {} out of Disabled folder.",
                mod_name
            ),
            Err(e) => error!(
                "Failed to move mod directory {:?} from Disabled: {}",
                mod_path.to_str().unwrap(),
                e
            ),
        }
    } else {
        warn!(
            "Path {:?} does not exist.",
            mod_path.to_str().unwrap()
        );
    }
}

/// Export a selected set of profiles to a JSON file
/// # Arguments
/// * `profile_names` - The names of the profiles to be exported
#[tauri::command]
async fn export_profiles(profile_names: Vec<String>) {
    let settings_json = SETTINGS_JSON.read().await;
    let profiles = settings_json["Profiles"].as_array().unwrap();
    let mut export_array = Vec::new();
    for profile_name in profile_names {
        for profile in profiles {
            let settings_profile_name = String::from(profile["Name"].as_str().unwrap());
            if profile_name == settings_profile_name {
                export_array.push(profile);
            }
        }
    }

    let export_json = json!({"Profiles": export_array});

    let export_path =   FileDialog::new()
        .set_location("~")
        .add_filter("JSON File", &["json"])
        .show_save_single_file()
        .unwrap();
    let export_path = match export_path {
        Some(path) => path,
        None => {
            error!("Path to export selected profiles to does not exist.");
            return;
        },
    };

    let export_file = File::create(export_path.clone()).unwrap();
    match serde_json::to_writer_pretty(export_file, &export_json) {
        Ok(_) => info!("Successfully exported selected profiles to new file at {:?}", export_path),
        Err(e) => error!("Failed to export selected profiles to new file at {:?}: {}", export_path, e),
    }
}

/// Fetch the progress of the mod that is currently being downloaded.
#[tauri::command]
async fn fetch_current_download_progress() -> u8 {
    CURRENT_DOWNLOAD_PROGRESS.read().await.to_string().parse::<u8>().unwrap()
}

/// Fetch the active profile.
#[tauri::command]
async fn fetch_current_profile() -> String {
    let settings_json = SETTINGS_JSON.read().await;
    String::from(settings_json["CurrentProfile"].as_str().unwrap())
}

/// Fetch a list of enabled mods
#[tauri::command]
async fn fetch_enabled_mods() -> Vec<Value> {
    let mods_json: Value = serde_json::from_str(&MODS_JSON.read().await).unwrap();
    let manifests = mods_json["Manifest"].as_array().unwrap();
    let mod_count = manifests.len();
    let mut enabled_mods = Vec::new();
    let mods_path = MODS_PATH.read().await.to_string();
    let disabled_path: PathBuf = [mods_path.as_str(), "Disabled"].iter().collect();
    for i in 0..mod_count {
        let mod_name = manifests[i]["Name"].as_str().unwrap();
        let mod_version = manifests[i]["Version"].as_str().unwrap();
        let mod_path: PathBuf = [mods_path.clone().as_str(), mod_name].iter().collect();
        let disabled_mod_path: PathBuf = [
            disabled_path.clone().into_os_string().to_str().unwrap(),
            mod_name,
        ]
        .iter()
        .collect();
        if mod_path.exists() && !disabled_mod_path.exists() {
            enabled_mods.push(json!({"Name": mod_name, "Version": mod_version}));
        }
    }

    enabled_mods
}

/// Fetch a list of installed mods
#[tauri::command]
async fn fetch_installed_mods() -> Vec<Value> {
    let mods_json: Value = serde_json::from_str(&MODS_JSON.read().await.to_string()).unwrap();
    let manifests = mods_json["Manifest"].as_array().unwrap();
    let mod_count = manifests.len();

    let mut installed_mods = Vec::new();
    let mods_path = MODS_PATH.read().await.to_string();
    let disabled_path: PathBuf = [mods_path.as_str(), "Disabled"].iter().collect();
    for i in 0..mod_count {
        let mod_name = manifests[i]["Name"].as_str().unwrap();
        let mod_version = manifests[i]["Version"].as_str().unwrap();
        let mod_path: PathBuf = [mods_path.clone().as_str(), mod_name].iter().collect();
        let disabled_mod_path: PathBuf = [
            disabled_path.clone().into_os_string().to_str().unwrap(),
            mod_name,
        ]
        .iter()
        .collect();
        if mod_path.exists() || disabled_mod_path.exists() {
            installed_mods.push(json!({"Name": mod_name, "Version": mod_version}));
        }
    }

    let mut settings_json = SETTINGS_JSON.write().await;

    let current_profile = String::from(settings_json["CurrentProfile"].as_str().unwrap());
    let language = String::from(settings_json["Language"].as_str().unwrap());
    let mods_path = String::from(settings_json["ModsPath"].as_str().unwrap());
    let profiles = settings_json["Profiles"].as_array().unwrap().to_vec();
    let theme = String::from(settings_json["Theme"].as_str().unwrap());
    let theme_path = String::from(settings_json["ThemePath"].as_str().unwrap());

    *settings_json = json!({
        "CurrentProfile": current_profile,
        "InstalledMods": installed_mods,
        "Language": language,
        "ModsPath": mods_path,
        "Profiles": profiles,
        "Theme": theme,
        "ThemePath": theme_path,
    });

    let settings_path = SETTINGS_PATH.read().await;
    if PathBuf::from_str(settings_path.as_str()).unwrap().exists() {
        let settings_file = File::options().write(true).open(settings_path.as_str()).unwrap();
        match serde_json::to_writer_pretty(settings_file, &*settings_json) {
            Ok(_) => info!("Successfully wrote installed mod to settings file."),
            Err(e) => error!("Failed to write installed mod to settings file: {}", e),
        }
    }

    installed_mods
}

#[tauri::command]
async fn fetch_language() -> String {
    let settings_json = SETTINGS_JSON.read().await;
    let language = String::from(settings_json["Language"].as_str().unwrap());
    language
}

/// Fetch a stringified JSON containing data on mods installed that are not on ModLinks.xml
#[tauri::command]
async fn fetch_manually_installed_mods() -> String {
    let mut manually_installed_mods = Vec::new();
    let mods_json: ModLinks = serde_json::from_str(&MODS_JSON.read().await).unwrap();
    let mods_path = MODS_PATH.read().await;
    let mut path_bufs = Vec::new();
    let path_buf = PathBuf::from_str(mods_path.as_str()).unwrap();
    path_bufs.push(path_buf);
    let disabled_path_buf: PathBuf = [mods_path.as_str(), "Disabled"].iter().collect();
    if disabled_path_buf.exists() {
        path_bufs.push(disabled_path_buf);
    }

    for path in path_bufs {
        'folder_loop: for mod_folder in fs::read_dir(path).unwrap() {
            match mod_folder.as_ref().unwrap().path().extension() {
                Some(_) => continue,
                None => (),
            }
            
            for i in 0..mods_json.manifests.len() {
                let mod_path = mod_folder.as_ref().unwrap().path();
                let mod_name = mod_path.file_name().unwrap().to_str().unwrap();
                let manifest_name = mods_json.manifests[i].name.as_str();                
                if mod_name == manifest_name {
                    continue 'folder_loop;
                }
            }

            for mod_file in fs::read_dir(mod_folder.as_ref().unwrap().path()).unwrap() {
                let file_path = mod_file.unwrap().path();
                match file_path.extension() {
                    Some(ext) => {
                        if ext.to_str().unwrap() == "dll" {
                            let mod_path = mod_folder.as_ref().unwrap().path();
                            let mod_name = mod_path.file_name().unwrap().to_str().unwrap();
                            let enabled = !String::from(mod_path.to_str().unwrap()).contains("Disabled");
                            let mod_json = json!({"name": mod_name, "enabled": enabled});
                            manually_installed_mods.push(mod_json);
                            break;
                        }
                    },
                    None => warn!("File {:?} has no extension, may be a directory.", file_path.to_str().unwrap()),
                }
           }
        }
    }

    let manually_installed_json = json!(manually_installed_mods);
    let manual_json = manually_installed_json.to_string();
    info!("Manual JSON: {}", manual_json);
    manual_json
}

/// Fetch the stringified JSON containing mod link data
#[tauri::command]
async fn fetch_mod_list() -> String {
    MODS_JSON.read().await.to_string()
}

/// Fetch all mod profiles
#[tauri::command]
async fn fetch_profiles() -> (String, String) {
    info!("Fetching profiles...");
    let settings_json = SETTINGS_JSON.read().await;
    let profiles = serde_json::to_string(&settings_json["Profiles"]).unwrap();
    let current_profile = String::from(settings_json["CurrentProfile"].as_str().unwrap());
    (profiles, current_profile)
}

/// Fetch theme data
#[tauri::command]
async fn fetch_theme_data() -> (String, String, String) {
    let settings_json = SETTINGS_JSON.read().await;
    let theme_path = String::from(settings_json["ThemePath"].as_str().unwrap());
    let theme = String::from(settings_json["Theme"].as_str().unwrap());
    let mut css = String::from("");
    if theme_path.as_str() != "" {
        let mut css_file = File::options().read(true).open(theme_path.clone()).unwrap();
        match css_file.read_to_string(&mut css) {
            Ok(css) => info!("Successfully read in from CSS file: {}", css),
            Err(e) => error!("Failed to read in from CSS File: {}", e),
        }
    }
    
    (theme, theme_path, css.to_string())
}

/// Import a set of profiles from a JSON file
#[tauri::command]
async fn import_profiles() {
    let import_path = FileDialog::new()
        .set_location("~")
        .add_filter("JSON File", &["json"])
        .show_open_single_file()
        .unwrap();
    let import_path = match import_path {
        Some(path) => path,
        None => {
            error!("Path to imported profiles JSON does not exist.");
            return;
        },
    };

    let imported_json_string = fs::read_to_string(import_path).unwrap();
    let mut imported_json: Value = serde_json::from_str(imported_json_string.as_str()).unwrap();
    let imported_profiles = imported_json["Profiles"].as_array_mut().unwrap();
    
    let current_profile: String;
    let installed_mods: Vec<Value>;
    let language: String;
    let mods_path: String;
    let theme: String;
    let theme_path: String;
    {
        let settings_json = SETTINGS_JSON.read().await;
        current_profile = String::from(settings_json["CurrentProfile"].as_str().unwrap());
        installed_mods = settings_json["InstalledMods"].as_array().unwrap().to_vec();
        language = String::from(settings_json["Language"].as_str().unwrap());
        mods_path = String::from(settings_json["ModsPath"].as_str().unwrap());
        theme = String::from(settings_json["Theme"].as_str().unwrap());
        theme_path = String::from(settings_json["ThemePath"].as_str().unwrap());
    }
    
    let mut settings_json = SETTINGS_JSON.write().await;

    let profiles_array = settings_json["Profiles"].as_array_mut().unwrap();
    let mut profiles_vector = profiles_array.to_vec();
    for profile in imported_profiles {
        profiles_vector.push(profile.clone());
    }
    
    *settings_json = json!({
       "CurrentProfile": current_profile,
       "InstalledMods": installed_mods,
       "Language": language,
       "ModsPath": mods_path,
       "Profiles": profiles_vector,
       "Theme": theme,
       "ThemePath": theme_path,
    });

    let settings_path = SETTINGS_PATH.read().await;
    if PathBuf::from_str(settings_path.as_str()).unwrap().exists() {
        let settings_file = File::options().write(true).open(settings_path.as_str()).unwrap();
        match serde_json::to_writer_pretty(settings_file, &*settings_json) {
            Ok(_) => info!("Successfully updated profiles in settings file."),
            Err(e) => error!("Failed to update profiles in settings file: {}", e),
        }
    }
}

/// Download a mod to disk from a provided link
/// # Arguments
/// * `mod_name` - The name of the mod folder to be created
/// * `mod_version` - The downloaded mod's version
/// * `mod_link` - The download link of the mod
#[tauri::command]
#[async_recursion]
async fn install_mod(mod_name: String, mod_version: String, mod_hash: String, mod_link: String) -> Result<(), String> {
    info!("Installing mod {:?}", mod_name);
    {
        let mut current_download_progress = CURRENT_DOWNLOAD_PROGRESS.write().await;
        *current_download_progress = 0;
    }

    let mods_path = MODS_PATH.read().await;
    let mod_path: PathBuf = [mods_path.as_str(), mod_name.as_str()].iter().collect();
    let disabled_mod_path: PathBuf = [mods_path.as_str(), "Disabled", mod_name.as_str()].iter().collect();
    if mod_path.exists() {
        let out_of_date = check_for_update(mod_name.clone(), mod_version.clone()).await;
        if !out_of_date {
            warn!("Mod {:?} already installed", mod_name);
            let mut current_download_progress = CURRENT_DOWNLOAD_PROGRESS.write().await;
            *current_download_progress = 100;
            return Ok(());
        }
    } else if disabled_mod_path.exists() {
        let out_of_date = check_for_update(mod_name.clone(), mod_version.clone()).await;
        if !out_of_date {
            warn!("Mod {:?} already installed but is disabled, enabling it instead.", mod_name);
            enable_mod(mod_name).await;
            let mut current_download_progress = CURRENT_DOWNLOAD_PROGRESS.write().await;
            *current_download_progress = 100;
            return Ok(());
        } else {
            uninstall_mod(mod_name.clone()).await;
        }
    }

    let client = reqwest::Client::new();
    let result = client.get(mod_link.clone()).send().await.or(Err(""))?;
    let total_size = result.content_length().ok_or(format!("Failed to get content length from {}", mod_link))?;
    let mod_path = format!("{}/{}", MODS_PATH.read().await.to_string(), mod_name);

    if !PathBuf::from_str(mod_path.as_str()).unwrap().exists() {
        match fs::create_dir(mod_path.clone()) {
            Ok(_) => info!("Successfully created mod folder for {:?}.", mod_name),
            Err(e) => error!("Failed to create mod folder for {:?}: {}", mod_name, e),
        }
    }

    let extension = mod_link.split(".").last().unwrap();
    let mut download_path = String::from("");
    if extension == "zip" {
        download_path = format!("{}/temp.zip", mod_path.clone());
    } else {
        download_path = format!("{}/{}", mod_path.clone(), mod_link.split("/").last().unwrap());
    }

    {
        let mut file = File::create(download_path.clone()).unwrap();
        let mut downloaded: u64 = 0;
        let mut stream = result.bytes_stream();
        while let Some(item) = stream.next().await {
            let chunk = item.unwrap();
            file.write_all(&chunk).unwrap();
            let new = min(downloaded + (chunk.len() as u64), total_size);
            downloaded = new;
            let mut current_download_progress = CURRENT_DOWNLOAD_PROGRESS.write().await;
            *current_download_progress = (((new as f64) / (total_size as f64)) * 100.0).floor() as u8;
        }
    }

    let file_hash = digest_file(download_path.clone()).unwrap();
    if file_hash.to_lowercase() != mod_hash.to_lowercase() {
        error!("Failed to verify SHA256 of downloaded file for mod {:?}, re-downloading...", mod_name);
        return install_mod(mod_name, mod_version, mod_hash, mod_link).await;
    } else {
        info!("Downloaded hash of {:?} matches with that on modlinks.", mod_name);
    }

    if extension == "zip" {
        let file = File::open(download_path.clone()).unwrap();
        let unzipper = Unzipper::new(file, mod_path);
        match unzipper.unzip() {
            Ok(_) => info!("Successfully unzipped contents of {}", download_path),
            Err(e) => error!("Failed to unzip contents of {}: {}", download_path, e),
        }

        fs::remove_file(download_path).unwrap();
    }

    let current_profile: String;
    let language: String;
    let mods_path: String;
    let profiles: Vec<Value>;
    let theme: String;
    let theme_path: String;
    {
        let settings_json = SETTINGS_JSON.read().await;
        let installed_mods = settings_json["InstalledMods"].as_array().unwrap();
        for install in installed_mods {
            let install_name = String::from(install["Name"].as_str().unwrap());
            let install_version = String::from(install["Version"].as_str().unwrap());
            if install_name == mod_name &&
               install_version == mod_version {
                return Ok(());
            }
        }

        current_profile = String::from(settings_json["CurrentProfile"].as_str().unwrap());
        language = String::from(settings_json["Language"].as_str().unwrap());
        mods_path = String::from(settings_json["ModsPath"].as_str().unwrap());
        profiles = settings_json["Profiles"].as_array().unwrap().to_vec();
        theme = String::from(settings_json["Theme"].as_str().unwrap());
        theme_path = String::from(settings_json["ThemePath"].as_str().unwrap());
    }

    let mut settings_json = SETTINGS_JSON.write().await;
    let installed_mods = settings_json["InstalledMods"].as_array_mut().unwrap();
    let mut exists = false;
    for i in 0..installed_mods.len() {
        let install_name = String::from(installed_mods[i]["Name"].as_str().unwrap());
        if install_name == mod_name {
            exists = true;
            installed_mods[i]["Version"] = json!(mod_version);
        }
    }

    if !exists {
        installed_mods.push(json!({"Name": mod_name, "Version": mod_version}));
    }
        
    *settings_json = json!({
        "CurrentProfile": current_profile,
        "InstalledMods": installed_mods,
        "Language": language,
        "ModsPath": mods_path,
        "Profiles": profiles,
        "Theme": theme,
        "ThemePath": theme_path,
    });

    let settings_path = SETTINGS_PATH.read().await;
    if PathBuf::from_str(settings_path.as_str()).unwrap().exists() {
        let settings_file = File::options().write(true).open(settings_path.as_str()).unwrap();
        match serde_json::to_writer_pretty(settings_file, &*settings_json) {
            Ok(_) => info!("Successfully wrote installed mod to settings file."),
            Err(e) => error!("Failed to write installed mod to settings file: {}", e),
        }
    }


    Ok(())
}

/// Manually install a mod from disk.
#[tauri::command]
async fn manually_install_mod() -> String {
    let selected_path = FileDialog::new()
        .set_location("~")
        .add_filter("Dynamic Link Library", &["dll"])
        .add_filter("ZIP Archive", &["zip"])
        .show_open_single_file()
        .unwrap();
    let selected_path = match selected_path {
        Some(path) => path,
        None => {
            error!("Selected path is not valid.");
            return String::from("");
        }
    };

    let path = Path::new(&selected_path);
    let mods_path = MODS_PATH.read().await;
    let extension = path.extension().unwrap().to_str().unwrap();
    let mod_name = String::from(path.file_name().unwrap().to_str().unwrap()).replace(format!(".{}", extension).as_str(), "");
    let mod_path = format!("{}/{}", mods_path, mod_name);
    let dll_path = format!("{}/{}.dll", mod_path, mod_name);
    match fs::create_dir(Path::new(&mod_path)) {
        Ok(_) => info!("Successfully created directory for manually installed mod {}", mod_name),
        Err(e) => error!("Failed to create directory for manually installed mod {}: {}", mod_name, e),
    }

    if extension == "dll" {
        match fs::copy(selected_path.clone(), dll_path) {
            Ok(_) => info!("Successfully copied DLL from selected path to mod path for manually installed mod {}", mod_name),
            Err(e) => error!("Failed to copy DLL from selected path to mod path for manually installed mod {}: {}", mod_name, e),
        }
    } else if extension == "zip" {
        let file = File::options().read(true).write(true).open(selected_path.clone()).unwrap();
        let unzipper = Unzipper::new(file, mod_path);
        match unzipper.unzip() {
            Ok(_) => info!("Successfully unzipped contents of manually installed mod at {}", selected_path.display()),
            Err(e) => error!("Failed to unzip contents of manually installed mod at {}: {}", selected_path.display(), e),
        }
    }

    let mut settings_json = SETTINGS_JSON.write().await;
    let current_profile = String::from(settings_json["CurrentProfile"].as_str().unwrap());
    let language = String::from(settings_json["Language"].as_str().unwrap());
    let mods_path = String::from(settings_json["ModsPath"].as_str().unwrap());
    let profiles = settings_json["Profiles"].as_array().unwrap().to_vec();
    let theme = String::from(settings_json["Theme"].as_str().unwrap());
    let theme_path = String::from(settings_json["ThemePath"].as_str().unwrap());
    let installed_mods = settings_json["InstalledMods"].as_array_mut().unwrap();
    let mut exists = false;
    for i in 0..installed_mods.len() {
        let install_name = String::from(installed_mods[i]["Name"].as_str().unwrap());
        if install_name == mod_name {
            exists = true;
            installed_mods[i]["Version"] = json!("Unknown");
        }
    }

    if exists {
       return String::from(""); 
    }

    installed_mods.push(json!({"Name": mod_name, "Version": "Unknown"}));
        
    *settings_json = json!({
        "CurrentProfile": current_profile,
        "InstalledMods": installed_mods,
        "Language": language,
        "ModsPath": mods_path,
        "Profiles": profiles,
        "Theme": theme,
        "ThemePath": theme_path,
    });

    let settings_path = SETTINGS_PATH.read().await;
    if PathBuf::from_str(settings_path.as_str()).unwrap().exists() {
        let settings_file = File::options().write(true).open(settings_path.as_str()).unwrap();
        match serde_json::to_writer_pretty(settings_file, &*settings_json) {
            Ok(_) => info!("Successfully wrote manually installed mod to settings file."),
            Err(e) => error!("Failed to write manually installed mod to settings file: {}", e),
        }
    }

    mod_name
}

/// Open the local folder on the file system containing all installed mods
#[tauri::command]
async fn open_mods_folder() {
    let mods_path = MODS_PATH.read().await;
    info!("Mods path: {:?}", &mods_path.as_str());
    match env::consts::OS {
        "linux" => {
            match Command::new("xdg-open").arg(&mods_path.as_str()).spawn() {
                Ok(_) => info!("Successfully opened mods folder."),
                Err(e) => error!("Failed to open mods folder: {}", e),
            }
        },
        "mac" => {
            match Command::new("open").arg(&mods_path.as_str()).spawn() {
                Ok(_) => info!("Successfully opened mods folder."),
                Err(e) => error!("Failed to open mods folder: {}", e),
            }
        },
        "windows" => {
            match Command::new("explorer").arg(str::replace(&mods_path.as_str(), "/", "\\")).spawn() {
                Ok(_) => info!("Successfully opened mods folder."),
                Err(e) => error!("Failed to open mods folder: {}", e),
            }
        },
        _ => panic!("OS not supported"),
    };
}

/// Resets a mod's global settings
/// # Arguments
/// * `mod_name` - The name of the mod whose global settings will be reset
#[tauri::command]
async fn reset_settings(mod_name: String) {
    let mods_path = MODS_PATH.read().await;
    let mod_path = format!("{}/{}", mods_path, mod_name);
    let file_paths = fs::read_dir(mod_path).unwrap();
    let base_dir = BaseDirs::new().unwrap();
    for file_path in file_paths {
        let path_buf = file_path.unwrap().path();
        let path = Path::new(&path_buf);
        let file_name = String::from(path.file_name().unwrap().to_str().unwrap());
        let file_extension = path.extension().unwrap().to_str().unwrap();
        let mut saves_path = String::from("");
        let mut dll_name = String::from("");
        if file_extension == "dll" {
            dll_name = file_name.replace(format!(".{}", file_extension).as_str(), "");
            match env::consts::OS {
                "linux" => {
                    saves_path = format!("{}/unity3d/Team Cherry/Hollow Knight", base_dir.config_dir().display());
                },
                "mac" => {
                    saves_path = format!("{}/unity.Team Cherry.Hollow Knight", base_dir.data_dir().display());
                },
                "windows" => {
                    saves_path = format!("{}/../LocalLow/Team Cherry/Hollow Knight", base_dir.data_dir().display());
                },
                _ => panic!("OS not supported."),
            }
        } else {
            continue;
        }

        info!("DLL name: {:?}", dll_name);

        let data_paths = fs::read_dir(saves_path).unwrap();
        for data_path in data_paths {
            let path = data_path.unwrap().path();
            let data_name = String::from(Path::new(&path).file_name().unwrap().to_str().unwrap());
            info!("Data name: {:?}", data_name);
            if data_name == format!("{}.GlobalSettings.json", dll_name) {
                match fs::remove_file(Path::new(&path)) {
                    Ok(_) => info!("Successfully deleted global settings for {}", mod_name),
                    Err(e) => error!("Failed to delete global settings for {}: {}", mod_name, e),
                }
            }
        }
    }
}

/// Set the application's default language
#[tauri::command]
async fn set_language(language: String) {
    let mut settings_json = SETTINGS_JSON.write().await;
    let value = json!(language.as_str());
    settings_json["Language"] = value;

    let settings_path = SETTINGS_PATH.read().await;
    if PathBuf::from_str(settings_path.as_str()).unwrap().exists() {
        let settings_file = File::options().write(true).open(settings_path.as_str()).unwrap();
        match serde_json::to_writer_pretty(settings_file, &*settings_json) {
            Ok(_) => info!("Successfully set language to {:?} in settings file.", language),
            Err(e) => error!("Failed to set language to {:?} in settings file: {}", language, e),
        }
    }
}

/// Sets the current mod profile in settings
/// # Arguments
/// * `profile_name` - The name of the profile to be set to
#[tauri::command]
async fn set_profile(profile_name: String) {
    let mut settings_json = SETTINGS_JSON.write().await;
    let value = json!(profile_name.as_str());
    settings_json["CurrentProfile"] = value;

    let settings_path = SETTINGS_PATH.read().await;
    if PathBuf::from_str(settings_path.as_str()).unwrap().exists() {
        let settings_file = File::options().write(true).open(settings_path.as_str()).unwrap();
        match serde_json::to_writer_pretty(settings_file, &*settings_json) {
            Ok(_) => info!("Successfully set current profile to {:?} in settings file.", profile_name),
            Err(e) => error!("Failed to set profile to {:?} in settings file: {}", profile_name, e),
        }
    }
}

/// Set the global theme
/// # Arguments
/// * `theme_name` - The name of theme to be set to
#[tauri::command]
async fn set_theme(theme_name: String) {
    let mut settings_json = SETTINGS_JSON.write().await;
    let value = json!(theme_name.as_str());
    settings_json["Theme"] = value;

    let settings_path = SETTINGS_PATH.read().await;
    if PathBuf::from_str(settings_path.as_str()).unwrap().exists() {
        let settings_file = File::options().write(true).open(settings_path.as_str()).unwrap();
        match serde_json::to_writer_pretty(settings_file, &*settings_json) {
            Ok(_) => info!("Successfully set current theme to {:?} in settings file.", theme_name),
            Err(e) => error!("Failed to set theme to {:?} in settings file: {}", theme_name, e),
        }
    }
}

/// Toggles the Modding API and returns whether it has been toggled on or off
#[tauri::command]
async fn toggle_api() -> bool {
    let mods_path = MODS_PATH.read().await;
    let managed_path: PathBuf = [mods_path.as_str(), ".."].iter().collect();
    let assembly: PathBuf = [managed_path.to_str().unwrap(), "Assembly-CSharp.dll"].iter().collect();
    let vanilla_assembly: PathBuf = [managed_path.to_str().unwrap(), "Assembly-CSharp.dll.vanilla"].iter().collect();
    let modded_assembly: PathBuf = [managed_path.to_str().unwrap(), "Assembly-CSharp.dll.modded"].iter().collect();
    if vanilla_assembly.exists() && !modded_assembly.exists() {
        // Disable the Modding API
        match fs::rename(assembly.clone(), modded_assembly) {
            Ok(_) => info!("Successfully renamed Assembly-CSharp to modded assembly backup."),
            Err(e) => error!("Failed to rename Assembly-CSharp to modded assembly backup: {}", e),
        }

        match fs::rename(vanilla_assembly, assembly) {
            Ok(_) => info!("Successfully replaced modded Assembly-CSharp with vanilla assembly."),
            Err(e) => error!("Failed to replace modded Assembly-CSharp with vanilla assembly: {}", e),
        }

        return false;
    } else if modded_assembly.exists() && !vanilla_assembly.exists() {
        // Enable the Modding API
        match fs::rename(assembly.clone(), vanilla_assembly) {
            Ok(_) => info!("Successfully renamed Assembly-CSharp to modded assembly backup."),
            Err(e) => error!("Failed to rename Assembly-CSharp to modded assembly backup: {}", e),
        }

        match fs::rename(modded_assembly, assembly) {
            Ok(_) => info!("Successfully replaced vanilla Assembly-CSharp with modded assembly."),
            Err(e) => error!("Failed to replace vanilla Assembly-CSharp with modded assembly: {}", e),
        }

        return true;
    } else if !modded_assembly.exists() && !vanilla_assembly.exists() {
        warn!("Neither the modded or vanilla assembly backups exists, downloading API.");
        install_api().await;
        return true;
    } else if modded_assembly.exists() && vanilla_assembly.exists() {
        panic!("Somehow, both assembly backups exist.");
    }
    
    panic!("Some other error has occurred.");
}

/// Removes a mod folder from disk
/// # Arguments
/// * `mod_name` - The name of the mod folder
#[tauri::command]
async fn uninstall_mod(mod_name: String) {
    info!("Uninstalling mod {:?}", mod_name);
    let mods_path = MODS_PATH.read().await;
    let mod_path: PathBuf = [mods_path.to_string(), mod_name.clone()].iter().collect();
    let disabled_mod_path: PathBuf = [
        mods_path.to_string(),
        String::from("Disabled"),
        mod_name.clone(),
    ]
    .iter()
    .collect();
    if mod_path.exists() {
        match fs::remove_dir_all(mod_path.as_path()) {
            Ok(_) => info!("Successfully removed all contents for {}", mod_name),
            Err(e) => error!("Failed to remove mod directory {:?}: {}", mod_path.to_str().unwrap(), e),
        }
    } else if disabled_mod_path.exists() {
        match fs::remove_dir_all(disabled_mod_path.as_path()) {
            Ok(_) => info!("Successfully removed all contents for {}", mod_name),
            Err(e) => error!("Failed to remove mod directory {:?}: {}", disabled_mod_path.to_str().unwrap(), e),
        }
    } else {
        warn!(
            "Path {:?} does not exist.",
            mod_path.to_str().unwrap()
        );
    }

    let mut settings_json = SETTINGS_JSON.write().await;
    let mut installed_mods = settings_json["InstalledMods"].as_array_mut().unwrap().to_vec();
    for i in 0..installed_mods.len() {
        let install_name = String::from(installed_mods[i]["Name"].as_str().unwrap());
        if install_name == mod_name {
            installed_mods.remove(i);
            break;
        }
    }

    let current_profile = String::from(settings_json["CurrentProfile"].as_str().unwrap());
    let language = String::from(settings_json["Language"].as_str().unwrap());
    let mods_path = String::from(settings_json["ModsPath"].as_str().unwrap());
    let profiles_value = &mut settings_json.clone()["Profiles"];
    let profiles = profiles_value.as_array().unwrap();
    let theme = String::from(settings_json["Theme"].as_str().unwrap());
    let theme_path = String::from(settings_json["ThemePath"].as_str().unwrap());
    
    *settings_json = json!({
       "CurrentProfile": current_profile,
       "InstalledMods": installed_mods,
       "Language": language,
       "ModsPath": mods_path,
       "Profiles": profiles,
       "Theme": theme,
       "ThemePath": theme_path,
    });
    
    let settings_path = SETTINGS_PATH.read().await;
    if PathBuf::from_str(settings_path.as_str()).unwrap().exists() {
        let settings_file = File::options().write(true).open(settings_path.as_str()).unwrap();
        match serde_json::to_writer_pretty(settings_file, &*settings_json) {
            Ok(_) => info!("Successfully removed installed mod from settings file."),
            Err(e) => error!("Failed to remove installed mod from settings file: {}", e),
        }
    }
}

/// Automatically detect the path to Hollow Knight executable, else prompt the user to select its path.
async fn auto_detect() {
    let mut settings_json = SETTINGS_JSON.write().await;
    if !settings_json.is_null() {
        return;
    }

    match env::consts::OS {
        "linux" | "mac" => {
            match STATIC_PATHS.into_iter().find(|path| {
                let base_dir = BaseDirs::new().unwrap();
                let path_buf: PathBuf = [base_dir.data_dir().to_str().unwrap(), path].iter().collect();
                path_buf.exists()
            }) {
                Some(game_path) => {
                    let confirm = MessageDialog::new()
                        .set_type(MessageType::Info)
                        .set_title("Is this your game path?")
                        .set_text(&format!(
                            "Game path detected at: {}\nIs this correct?",
                            game_path
                        ))
                        .show_confirm()
                        .unwrap();
                    if confirm {
                        match SUFFIXES.into_iter().find(|suffix| {
                            let path_buf: PathBuf = [game_path, suffix].iter().collect();
                            path_buf.exists()
                        }) {
                            Some(suffix) => {
                                let mut mods_path = MODS_PATH.write().await;
                                let base_dir = BaseDirs::new().unwrap();
                                *mods_path = format!(
                                    "{}/{}/{}/Mods",
                                    base_dir.data_dir().to_str().unwrap(),
                                    game_path,
                                    suffix
                                )
                                .to_string();
                            }
                            None => {
                                error!("No managed path exists.");
                            }
                        }
                    } else {
                        select_game_path().await;
                    }
                },
                None => {
                    MessageDialog::new()
                        .set_type(MessageType::Info)
                        .set_title("Could not find Hollow Knight")
                        .set_text("Butterfly could not detect your Hollow Knight installation.\n
                            Please select the folder that contains your Hollow Knight executable."
                        )
                        .show_alert()
                        .unwrap();
                    select_game_path().await
                },
            }
        }
        "windows" => {
            let mut drive_letter: String = String::from("C:/");
            for i in 65u8..=90 {
                if PathBuf::from_str(format!("{}:/", i).as_str())
                    .unwrap()
                    .exists()
                {
                    drive_letter = format!("{}:/", i);
                }
            }
            match STATIC_PATHS.into_iter().find(|path| {
                let path_buf: PathBuf = [drive_letter.to_string(), path.to_string()]
                    .iter()
                    .collect();
                info!(
                    "Checking if path {} exists",
                    path_buf.clone().into_os_string().into_string().unwrap()
                );
                path_buf.exists()
            }) {
                Some(game_path) => {
                    let confirm = MessageDialog::new()
                        .set_type(MessageType::Info)
                        .set_title("Is this your game path?")
                        .set_text(&format!(
                            "Game path detected at: {}{}\nIs this correct?",
                            drive_letter.as_str(),
                            game_path
                        ))
                        .show_confirm()
                        .unwrap();
                    if confirm {
                        match SUFFIXES.into_iter().find(|suffix| {
                            let path_buf: PathBuf =
                                [drive_letter.as_str(), game_path, suffix]
                                    .iter()
                                    .collect();
                            info!(
                                "Checking managed path: {}",
                                path_buf.clone().into_os_string().into_string().unwrap()
                            );
                            path_buf.exists()
                        }) {
                            Some(suffix) => {
                                let mut mods_path = MODS_PATH.write().await;
                                *mods_path = format!(
                                    "{}{}/{}/Mods",
                                    drive_letter.as_str(),
                                    game_path,
                                    suffix
                                )
                                .to_string();
                            }
                            None => error!("No managed path exists."),
                        }
                    } else {
                        select_game_path().await;
                    }
                }
                None => select_game_path().await,
            }
        }
        _ => panic!("OS not supported"),
    }

    let mods_path = MODS_PATH.read().await;
    if !PathBuf::from_str(mods_path.as_str()).unwrap().exists() {
        match fs::create_dir(mods_path.as_str()) {
            Ok(_) => info!("Successfully created mods directory."),
            Err(e) => error!("Error creating mods folder: {}", e),
        }
    }

    *settings_json = json!({
        "CurrentProfile": "",
        "InstalledMods": [],
        "Language": "English",
        "ModsPath" : String::from(mods_path.as_str()),
        "Profiles": [],
        "Theme": "Dark",
        "ThemePath": "",
    });
    info!("Settings JSON: {}", settings_json.to_string());
    let settings_path = SETTINGS_PATH.read().await;
    if !PathBuf::from_str(settings_path.as_str()).unwrap().exists() {
        let settings_file = File::create(settings_path.as_str()).unwrap();
        match serde_json::to_writer_pretty(settings_file, &*settings_json) {
            Ok(_) => info!("Successfully created settings file."),
            Err(e) => error!("Failed to create settings file: {}", e),
        }
    }
}

/// Load the list of mods from https://raw.githubusercontent.com/hk-modding/modlinks/main/ModLinks.xml
async fn load_mod_list() {
    info!("Loading mod list...");
    let content = reqwest::blocking::get(
        "https://raw.githubusercontent.com/hk-modding/modlinks/main/ModLinks.xml",
    )
    .unwrap()
    .text()
    .unwrap();
    let mut mod_links = ModLinks::new();
    match quick_xml::de::from_str(content.as_str()) {
        Ok(value) => {
            info!("Successfully parsed ModLinks XML");
            mod_links = value;
        }
        Err(e) => error!("Failed to parse ModLinks XML: {}", e),
    }
    
    let mut mods_json = MODS_JSON.write().await;
    *mods_json = serde_json::to_string_pretty(&mod_links).unwrap();
    info!("Mods JSON\n{}", mods_json);
}

/// Download a copy of the Modding API and replace local files with its contents if
/// their hashes do not match; Also backs up the vanilla Assembly-CSharp.dll file.
async fn install_api() {
    let content = reqwest::blocking::get(
        "https://raw.githubusercontent.com/hk-modding/modlinks/main/ApiLinks.xml",
    )
    .unwrap()
    .text()
    .unwrap();
    let mut api_links = ApiLinks::new();
    match quick_xml::de::from_str(content.as_str()) {
        Ok(value) => {
            info!("Successfully parsed API XML.");
            api_links = value;
            info!("API XML\n{}", serde_json::to_string_pretty(&api_links).unwrap());
        }
        Err(e) => error!("Failed to parse API XML: {}", e),
    }

    let mods_path = MODS_PATH.read().await;
    let managed_path: PathBuf = [mods_path.as_str(), ".."].iter().collect();
    let settings_path = SETTINGS_PATH.read().await;
    let temp_path: PathBuf = [settings_path.as_str(), "..", "Temp"].iter().collect();
    let api_url: String;
    match env::consts::OS {
        "linux" => api_url = String::from("https://github.com/hk-modding/api/releases/latest/download/ModdingApiLinux.zip"),
        "mac" => api_url = String::from("https://github.com/hk-modding/api/releases/latest/download/ModdingApiMac.zip"),
        "windows" => api_url = String::from("https://github.com/hk-modding/api/releases/latest/download/ModdingApiWin.zip"),
        _ => panic!("OS not supported."),
    }

    match reqwest::blocking::get(api_url) {
        Ok(response) => {
            let content = response.bytes().unwrap();
            let reader = Cursor::new(content);
            let unzipper = Unzipper::new(reader, temp_path.clone());
            match unzipper.unzip() {
                Ok(_) => info!("Successfully unzipped API to Temp folder."),
                Err(e) => error!("Failed to unzip API to Temp folder: {}", e),
            }
        },
        Err(e) => error!("Failed to get response: {}", e),
    }

    for file in api_links.manifest.files.files {
        let temp_file: PathBuf = [temp_path.to_str().unwrap(), file.as_str()].iter().collect();
        let local_file: PathBuf = [managed_path.to_str().unwrap(), file.as_str()].iter().collect();
        if !local_file.exists() {
            match fs::rename(temp_file, local_file) {
                Ok(_) => info!("Successfully moved temp file for {:?} to Managed folder.", file),
                Err(e) => error!("Failed to move temp file for {:?} to Managed folder: {}", file, e),
            }
        } else if digest_file(temp_file.clone()).unwrap() != digest_file(local_file.clone()).unwrap() {
            if file == "Assembly-CSharp.dll" {
                let vanilla_backup: PathBuf = [managed_path.to_str().unwrap(), "Assembly-CSharp.dll.vanilla"].iter().collect();
                match fs::rename(local_file.clone(), vanilla_backup) {
                    Ok(_) => info!("Successfully backed up vanilla Assembly-CSharp."),
                    Err(e) => error!("Failed to backup vanilla Assembly-Csharp: {}", e),
                }
            }
            match fs::rename(temp_file, local_file) {
                Ok(_) => info!("Successfully replaced old local file for {:?} with new API file.", file),
                Err(e) => error!("Failed to replace old local file for {:?} with new API file: {}", file, e),
            }
        }
    }

    match fs::remove_dir_all(temp_path) {
        Ok(_) => info!("Successfully deleted Temp folder."),
        Err(e) => error!("Failed to delete Temp folder: {}", e),
    }
}

/// Load the settings JSON file into the settings object, or create the file if it does not exist
/// and open the log file
async fn load_or_create_files() {
    const SETTINGS_FOLDER: &str = "Butterfly";
    let base_dir = BaseDirs::new().unwrap();
    let settings_dir: PathBuf = [base_dir.data_dir().to_str().unwrap(), SETTINGS_FOLDER].iter().collect();
    if !settings_dir.exists() {
        match fs::create_dir(settings_dir.as_path()) {
            Ok(_) => info!("Created settings and log directory"),
            Err(e) => error!("Failed to create settings folder: {}", e),
        }
    }

    let settings_string = settings_dir.to_str().unwrap();
    let mut log_path = LOG_PATH.write().await;
    *log_path = format!("{}/Log.txt", settings_string);
    match simple_logging::log_to_file(log_path.as_str(), LevelFilter::Info) {
        Ok(_) => info!("Opened logger at: {}", log_path.as_str()),
        Err(e) => {
            println!("Failed to open logger: {}", e);
            return;
        }
    }

    let mut settings_path = SETTINGS_PATH.write().await;
    *settings_path = format!("{}/Settings.json", settings_string);
    info!(
        "Checking if settings JSON exists at {}",
        settings_path.as_str()
    );

    if PathBuf::from_str(settings_path.as_str()).unwrap().exists() {
        let mut settings_json = SETTINGS_JSON.write().await;
        
        let mut settings_string = fs::read_to_string(Path::new(settings_path.as_str())).unwrap();
        loop {
            match serde_json::from_str(&settings_string) {
                Ok(value) => {
                    *settings_json = value;
                    break;
                },
                Err(_) => settings_string = settings_string[..settings_string.len() - 1].to_string(),
            }
        }

        info!("Settings JSON value is now: {}", settings_json.to_string());

        let mut mods_path = MODS_PATH.write().await;
        if settings_json["ModsPath"].is_string() {
            *mods_path = String::from(settings_json["ModsPath"].as_str().unwrap());
        }
    }
}

/// Manually select the path of the game's executable
async fn select_game_path() {
    warn!("Selecting game path manually.");
    let mut mods_path = MODS_PATH.write().await;

    let selected_path = FileDialog::new()
        .set_location("~")
        .show_open_single_dir()
        .unwrap();
    let selected_path = match selected_path {
        Some(path) => path,
        None => {
            error!("Selected path is not valid.");
            return;
        }
    };

    match SUFFIXES.into_iter().find(|suffix| {
        let path_buf: PathBuf = [selected_path.clone(), PathBuf::from_str(suffix).unwrap()]
            .iter()
            .collect();
        info!("Checking selected path: {}", path_buf.clone().to_str().unwrap());
        path_buf.exists()
    }) {
        Some(suffix) => {
            *mods_path = format!(
                "{}/{}/Mods",
                selected_path.to_str().unwrap(),
                suffix
            );
        }
        None => error!("No managed path found."),
    }
    info!("Selected mod path as: {}", mods_path.as_str());
}
