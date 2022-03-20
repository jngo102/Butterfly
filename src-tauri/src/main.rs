#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use directories::BaseDirs;
use lazy_static::lazy_static;
use log::{error, info, trace, warn, LevelFilter};
use native_dialog::{FileDialog, MessageDialog, MessageType};
use reqwest;
use serde;
use serde::{Deserialize, Serialize};
use serde_json;
use serde_json::{json, Map, Value};
use simple_logging;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::fs::{File, OpenOptions};
use std::io::{Cursor, Write};
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::sync::RwLock;
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
struct Dependencies {
    #[serde(rename = "Dependency", default)]
    dependencies: Vec<String>,
}

/// The manifest object containing data about an individual mod
#[derive(Debug, Deserialize, PartialEq, Serialize)]
struct Manifest {
    #[serde(rename = "Name", default)]
    name: String,
    #[serde(rename = "Description", default)]
    description: String,
    #[serde(rename = "Version", default)]
    version: String,
    #[serde(rename = "Link", default)]
    link: String,
    #[serde(rename = "SHA256", default)]
    sha256: String,
    #[serde(rename = "Dependencies")]
    dependencies: Dependencies,
}

/// The main mod links object
#[derive(Debug, Deserialize, PartialEq, Serialize)]
struct ModLinks {
    #[serde(rename = "xmlns", default)]
    xmlns: String,
    #[serde(rename = "xmlns:xsd", default)]
    xsd: String,
    #[serde(rename = "xmlns:xsi", default)]
    xsi: String,
    #[serde(rename = "xsi:schemaLocation", default)]
    schema_location: String,
    #[serde(rename = "Manifest", default)]
    manifests: Vec<Manifest>,
}

impl ModLinks {
    /// Create a new instance of a mod links object
    fn new() -> ModLinks {
        ModLinks {
            xmlns: String::new(),
            xsd: String::new(),
            xsi: String::new(),
            schema_location: String::new(),
            manifests: Vec::new(),
        }
    }
}

lazy_static! {
    /// A list of enabled mods
    static ref ENABLED_MODS: RwLock<Vec<bool>> = RwLock::new(Vec::new());
    /// A list of installed mods
    static ref INSTALLED_MODS: RwLock<Vec<bool>> = RwLock::new(Vec::new());
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
}

fn main() {
    load_or_create_files();
    auto_detect();
    load_mod_list();
    get_installed_mods();
    get_enabled_mods();
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            create_profile,
            debug,
            disable_mod,
            enable_mod,
            fetch_enabled_mods,
            fetch_installed_mods,
            fetch_mod_list,
            fetch_profiles,
            install_mod,
            set_profile,
            uninstall_mod
        ])
        .run(tauri::generate_context!())
        .expect("Failed to run tauri application.");
}

#[tauri::command]
fn create_profile(profile_name: String, mod_names: Vec<String>) {
    {
        let mut settings_json = SETTINGS_JSON.write().unwrap();
        let profiles = settings_json["Profiles"].as_array_mut().unwrap();
        profiles.push(json!({"Name": profile_name, "Mods": mod_names}));
        settings_json["Profiles"] = json!(profiles);

        let settings_path = SETTINGS_PATH.read().unwrap();
        if PathBuf::from_str(settings_path.as_str()).unwrap().exists() {
            let mut settings_file = OpenOptions::new().write(true).open(settings_path.as_str()).unwrap();
            match settings_file.write_all(settings_json.to_string().as_bytes()) {
                Ok(_) => info!("Successfully added new profile to settings file."),
                Err(e) => error!("Failed to write new profile to settings file: {}", e),
            }
            match settings_file.sync_all() {
                Ok(_) => info!("Successfully synced settings file with file system."),
                Err(e) => error!("Failed to sync with file system: {}", e),
            }
        }
    }
}

/// A tauri command that may be invoked from TypeScript for debugging purposes
/// # Arguments
/// * `msg` - The message to send from TypeScript
#[tauri::command]
fn debug(msg: String) {
    trace!("Debug message: {}", msg);
}

/// Move a mod folder into the Disabled folder if it is located in the Mods folder
/// # Arguments
/// `mod_name` - The name of the mod folder to be moved into the Disabled folder
#[tauri::command]
fn disable_mod(mod_name: String) {
    {
        let mods_path = MODS_PATH.read().unwrap();
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
                    mod_path.into_os_string().into_string(),
                    e
                ),
            }
        } else {
            warn!(
                "Path {:?} does not exist.",
                mod_path.into_os_string().into_string()
            );
        }
    }
}

/// Move a mod folder out of the Disabled folder if it is there
/// # Arguments
/// * `mod_name` - The name of the mod folder to move out of the Disabled folder
#[tauri::command]
fn enable_mod(mod_name: String) {
    {
        let mods_path = MODS_PATH.read().unwrap();
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
                    mod_path.into_os_string().into_string(),
                    e
                ),
            }
        } else {
            warn!(
                "Path {:?} does not exist.",
                mod_path.into_os_string().into_string()
            );
        }
    }
}

/// Fetch a list of enabled mods
#[tauri::command]
fn fetch_enabled_mods() -> Vec<bool> {
    ENABLED_MODS.read().unwrap().to_vec()
}

/// Fetch a list of installed mods
#[tauri::command]
fn fetch_installed_mods() -> Vec<bool> {
    INSTALLED_MODS.read().unwrap().to_vec()
}

/// Fetch the stringified JSON containing mod link data
#[tauri::command]
fn fetch_mod_list() -> String {
    MODS_JSON.read().unwrap().to_string()
}

/// Fetch all mod profiles
#[tauri::command]
fn fetch_profiles() -> String {
    {
        info!("Fetching profiles...");
        let settings_json = SETTINGS_JSON.read().unwrap();
        serde_json::to_string(&settings_json["Profiles"]).unwrap()
    }
}

/// Download a mod to disk from a provided link
/// # Arguments
/// * `mod_name` - The name of the mod folder to be created
/// * `mod_link` - The download link of the mod
#[tauri::command]
fn install_mod(mod_name: String, mod_link: String) {
    match reqwest::blocking::get(mod_link) {
        Ok(response) => {
            let content = response.bytes().unwrap();
            let reader = Cursor::new(content);
            let mod_path = format!("{}/{}", MODS_PATH.read().unwrap().to_string(), mod_name);
            if !PathBuf::from_str(mod_path.as_str()).unwrap().exists() {
                match fs::create_dir(mod_path.clone()) {
                    Ok(_) => info!("Successfully created mod folder."),
                    Err(e) => error!("Failed to create mod folder {}: {}", mod_name, e),
                }
            }
            let zip = Unzipper::new(reader, mod_path.clone());
            match zip.unzip() {
                Ok(_) => info!("Successfully unzipped to mod folder."),
                Err(e) => error!("Failed to unzip: {}", e),
            }
        }
        Err(e) => error!("Failed to get response: {}", e),
    }
}

#[tauri::command]
fn set_profile(profile_name: String) {
    {
        info!("Set current profile to {:?}", profile_name);
        let mut settings_json = SETTINGS_JSON.write().unwrap();
        *settings_json = json!({
            "ModsPath": settings_json["ModsPath"],
            "CurrentProfile": profile_name,
            "Profiles": settings_json["Profiles"]
        });

        let settings_path = SETTINGS_PATH.read().unwrap();
        if PathBuf::from_str(settings_path.as_str()).unwrap().exists() {
            let mut settings_file = OpenOptions::new().write(true).open(settings_path.as_str()).unwrap();
            match settings_file.write_all(settings_json.to_string().as_bytes()) {
                Ok(_) => info!("Successfully set profile in settings file."),
                Err(e) => error!("Failed to set profile in settings file: {}", e),
            }
            match settings_file.sync_all() {
                Ok(_) => info!("Successfully synced settings file with file system."),
                Err(e) => error!("Failed to sync with file system: {}", e),
            }
        }
    }
}

/// Removes a mod folder from disk
/// # Arguments
/// * `mod_name` - The name of the mod folder
#[tauri::command]
fn uninstall_mod(mod_name: String) {
    {
        let mods_path = MODS_PATH.read().unwrap();
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
                Ok(_) => println!("Successfully removed all contents for {}", mod_name),
                Err(e) => error!(
                    "Failed to remove mod directory {:?}: {}",
                    mod_path.into_os_string().into_string(),
                    e
                ),
            }
        } else if disabled_mod_path.exists() {
            match fs::remove_dir_all(disabled_mod_path.as_path()) {
                Ok(_) => println!("Successfully removed all contents for {}", mod_name),
                Err(e) => error!(
                    "Failed to remove mod directory {:?}: {}",
                    disabled_mod_path.into_os_string().into_string(),
                    e
                ),
            }
        } else {
            warn!(
                "Path {:?} does not exist.",
                mod_path.into_os_string().into_string()
            );
        }
    }
}

/// Merges the settings JSON with a new field
/// # Arguments
/// * `fields` - The fields that will be merged with the settings JSON
fn add_to_settings(fields: HashMap<String, String>) -> () {
    {
        let mut settings_json = SETTINGS_JSON.write().unwrap();
        let mut map = Map::new();
        for (key, value) in fields {
            map.insert(key, Value::String(value));
        }

        *settings_json = Value::Object(map);
        let settings_path = SETTINGS_PATH.read().unwrap();
        if PathBuf::from_str(settings_path.as_str()).unwrap().exists() {
            let mut settings_file = File::open(settings_path.as_str()).unwrap();
            match settings_file.write_all(settings_json.to_string().as_bytes()) {
                Ok(_) => info!("Successfully added additional field to settings JSON."),
                Err(e) => error!("Failed to write to settings file: {}", e),
            }
            match settings_file.sync_all() {
                Ok(_) => info!("Successfully synced settings JSON with file system."),
                Err(e) => error!("Failed to sync with file system: {}", e),
            }
        }
    }
}

/// Automatically detect the path to Hollow Knight executable, else prompt the user to select its path.
fn auto_detect() -> () {
    {
        let mut settings_json = SETTINGS_JSON.write().unwrap();
        if !settings_json.is_null() {
            return;
        }

        match env::consts::OS {
            "linux" => {
                match STATIC_PATHS.into_iter().find(|path| {
                    let base_dir = BaseDirs::new().unwrap();
                    let path_buf: PathBuf = [
                        base_dir.data_dir().to_str().unwrap(),
                        ".local",
                        "share",
                        path,
                    ]
                    .iter()
                    .collect();
                    path_buf.exists()
                }) {
                    Some(static_path) => {
                        let confirm = MessageDialog::new()
                            .set_type(MessageType::Info)
                            .set_title("Is this your game path?")
                            .set_text(&format!(
                                "Game path detected at: {}\nIs this correct?",
                                static_path
                            ))
                            .show_confirm()
                            .unwrap();
                        if confirm {
                            match SUFFIXES.into_iter().find(|suffix| {
                                let path_buf: PathBuf = [static_path, suffix].iter().collect();
                                path_buf.exists()
                            }) {
                                Some(suffix) => {
                                    let mut mods_path = MODS_PATH.write().unwrap();
                                    let base_dir = BaseDirs::new().unwrap();
                                    *mods_path = format!(
                                        "{}/.local/share/{}/{}/Mods",
                                        base_dir.data_dir().to_str().unwrap(),
                                        static_path,
                                        suffix
                                    )
                                    .to_string();
                                }
                                None => {
                                    error!("No managed path exists.");
                                }
                            }
                        } else {
                            select_game_path();
                        }
                    }
                    None => select_game_path(),
                }
            }
            "macos" => {
                match STATIC_PATHS.into_iter().find(|path| {
                    let base_dir = BaseDirs::new().unwrap();
                    let path_buf: PathBuf = [
                        base_dir.data_dir().to_str().unwrap(),
                        "Library",
                        "Application Support",
                        path,
                    ]
                    .iter()
                    .collect();
                    path_buf.exists()
                }) {
                    Some(static_path) => {
                        let confirm = MessageDialog::new()
                            .set_type(MessageType::Info)
                            .set_title("Is this your game path?")
                            .set_text(&format!(
                                "Game path detected at: {}\nIs this correct?",
                                static_path
                            ))
                            .show_confirm()
                            .unwrap();
                        if confirm {
                            match SUFFIXES.into_iter().find(|suffix| {
                                let path_buf: PathBuf = [static_path, suffix].iter().collect();
                                path_buf.exists()
                            }) {
                                Some(suffix) => {
                                    let mut mods_path = MODS_PATH.write().unwrap();
                                    let base_dir = BaseDirs::new().unwrap();
                                    *mods_path = format!(
                                        "{}/Library/Application Support/{}/{}/Mods",
                                        base_dir.data_dir().to_str().unwrap(),
                                        static_path,
                                        suffix
                                    )
                                    .to_string();
                                }
                                None => {
                                    error!("No managed path exists.");
                                }
                            }
                        } else {
                            select_game_path();
                        }
                    }
                    None => select_game_path(),
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
                    Some(static_path) => {
                        let confirm = MessageDialog::new()
                            .set_type(MessageType::Info)
                            .set_title("Is this your game path?")
                            .set_text(&format!(
                                "Game path detected at: {}{}\nIs this correct?",
                                drive_letter.as_str(),
                                static_path
                            ))
                            .show_confirm()
                            .unwrap();
                        if confirm {
                            match SUFFIXES.into_iter().find(|suffix| {
                                let path_buf: PathBuf =
                                    [drive_letter.as_str(), static_path, suffix]
                                        .iter()
                                        .collect();
                                info!(
                                    "Checking managed path: {}",
                                    path_buf.clone().into_os_string().into_string().unwrap()
                                );
                                path_buf.exists()
                            }) {
                                Some(suffix) => {
                                    let mut mods_path = MODS_PATH.write().unwrap();
                                    *mods_path = format!(
                                        "{}{}/{}/Mods",
                                        drive_letter.as_str(),
                                        static_path,
                                        suffix
                                    )
                                    .to_string();
                                }
                                None => error!("No managed path exists."),
                            }
                        } else {
                            select_game_path();
                        }
                    }
                    None => select_game_path(),
                }
            }
            _ => panic!("OS not supported."),
        }

        info!("Getting mods path");
        let mods_path = MODS_PATH.read().unwrap();
        info!("Mods path: {}", mods_path.as_str());
        if !PathBuf::from_str(mods_path.as_str()).unwrap().exists() {
            match fs::create_dir(mods_path.as_str()) {
                Ok(_) => info!("Successfully created mods directory."),
                Err(e) => error!("Error creating mods folder: {}", e),
            }
        }

        *settings_json = json!({
            "ModsPath" : mods_path.as_str(),
            "Profiles": [],
            "CurrentProfile": "",
        });
        info!("Settings JSON: {}", settings_json.to_string());
        let settings_path = SETTINGS_PATH.read().unwrap();
        if !PathBuf::from_str(settings_path.as_str()).unwrap().exists() {
            let mut settings_file = File::create(settings_path.as_str()).unwrap();
            match settings_file.write_all(settings_json.to_string().as_bytes()) {
                Ok(_) => info!("Successfully wrote path of mods to settings JSON."),
                Err(e) => error!("Failed to write to settings file: {}", e),
            }
            match settings_file.sync_all() {
                Ok(_) => info!("Successfully synced settings JSON with file system."),
                Err(e) => error!("Failed to sync with file system: {}", e),
            }
        }
    }
}

/// Retrieve a list of mods that are enabled
fn get_enabled_mods() {
    let mods_json: Value = serde_json::from_str(&MODS_JSON.read().unwrap()).unwrap();
    let manifests = mods_json["Manifest"].as_array().unwrap();
    let mod_count = manifests.len();
    let mut enabled_mods = ENABLED_MODS.write().unwrap();
    let mods_path = MODS_PATH.read().unwrap().to_string();
    let disabled_path: PathBuf = [mods_path.as_str(), "Disabled"].iter().collect();
    let installed_mods = INSTALLED_MODS.read().unwrap();
    for i in 0..mod_count {
        if !installed_mods[i] {
            enabled_mods.push(false);
            continue;
        }
        let mod_name = manifests[i]["Name"].as_str().unwrap();
        let mod_path: PathBuf = [mods_path.clone().as_str(), mod_name].iter().collect();
        let disabled_mod_path: PathBuf = [
            disabled_path.clone().into_os_string().to_str().unwrap(),
            mod_name,
        ]
        .iter()
        .collect();
        enabled_mods.push(mod_path.exists() && !disabled_mod_path.exists());
    }
}

/// Retrieve a list of mods that are installed on disk
fn get_installed_mods() {
    let mods_json: Value = serde_json::from_str(&MODS_JSON.read().unwrap().to_string()).unwrap();
    let manifests = mods_json["Manifest"].as_array().unwrap();
    let mod_count = manifests.len();
    {
        let mut installed_mods = INSTALLED_MODS.write().unwrap();
        let mods_path = MODS_PATH.read().unwrap().to_string();
        let disabled_path: PathBuf = [mods_path.as_str(), "Disabled"].iter().collect();
        for i in 0..mod_count {
            let mod_name = manifests[i]["Name"].as_str().unwrap();
            let mod_path: PathBuf = [mods_path.clone().as_str(), mod_name].iter().collect();
            let disabled_mod_path: PathBuf = [
                disabled_path.clone().into_os_string().to_str().unwrap(),
                mod_name,
            ]
            .iter()
            .collect();
            installed_mods.push(mod_path.exists() || disabled_mod_path.exists());
        }
    }
}

/// Load the list of mods from https://raw.githubusercontent.com/hk-modding/modlinks/main/ModLinks.xml
fn load_mod_list() {
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
            info!("Successfully parsed XML.");
            mod_links = value;
        }
        Err(e) => error!("Failed to parse XML: {}", e),
    }
    {
        let mut mods_json = MODS_JSON.write().unwrap();
        *mods_json = serde_json::to_string(&mod_links).unwrap();
    }
}

/// Load the settings JSON file into the settings object, or create the file if it does not exist
/// and open the log file
fn load_or_create_files() {
    let settings_dir: PathBuf;
    const SETTINGS_FOLDER: &str = "Butterfly";
    match env::consts::OS {
        "linux" => {
            let base_dir = BaseDirs::new().unwrap();
            settings_dir = [base_dir.data_dir().to_str().unwrap(), SETTINGS_FOLDER]
                .iter()
                .collect();
        }
        "macos" => {
            let base_dir = BaseDirs::new().unwrap();
            settings_dir = [
                base_dir.data_dir().to_str().unwrap(),
                "Library",
                "Application Support",
                SETTINGS_FOLDER,
            ]
            .iter()
            .collect();
        }
        "windows" => {
            let base_dir = BaseDirs::new().unwrap();
            settings_dir = [base_dir.data_dir().to_str().unwrap(), SETTINGS_FOLDER]
                .iter()
                .collect();
        }
        _ => panic!("OS not supported."),
    }

    if !settings_dir.exists() {
        match fs::create_dir(settings_dir.as_path()) {
            Ok(_) => info!("Created settings and log directory"),
            Err(e) => error!("Failed to create settings folder: {}", e),
        }
    }

    let settings_string = settings_dir.into_os_string().into_string().unwrap();
    {
        let mut log_path = LOG_PATH.write().unwrap();
        *log_path = format!("{}/Log.txt", settings_string);
        match simple_logging::log_to_file(log_path.as_str(), LevelFilter::Trace) {
            Ok(_) => info!("Opened logger at: {}", log_path.as_str()),
            Err(e) => {
                println!("Failed to open logger: {}", e);
                return;
            }
        }
    }

    {
        let mut settings_path = SETTINGS_PATH.write().unwrap();
        *settings_path = format!("{}/Settings.json", settings_string);
        info!(
            "Checking if settings JSON exists at {}",
            settings_path.as_str()
        );
        if PathBuf::from_str(settings_path.as_str()).unwrap().exists() {
            let mut settings_json = SETTINGS_JSON.write().unwrap();
            *settings_json = serde_json::from_str(
                &fs::read_to_string(Path::new(settings_path.as_str())).unwrap(),
            )
            .unwrap();
            info!("Settings JSON value is now: {}", settings_json.to_string());

            let mut mods_path = MODS_PATH.write().unwrap();
            if settings_json["ModsPath"].is_string() {
                *mods_path = String::from_str(settings_json["ModsPath"].as_str().unwrap()).unwrap();
            }
        }
    }
}

/// Manually select the path of the game's executable
fn select_game_path() {
    warn!("Selecting game path manually.");
    {
        let mut mods_path = MODS_PATH.write().unwrap();

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
            info!(
                "Checking selected path: {}",
                path_buf.clone().into_os_string().into_string().unwrap()
            );
            path_buf.exists()
        }) {
            Some(suffix) => {
                *mods_path = format!(
                    "{}/{}/Mods",
                    selected_path.into_os_string().into_string().unwrap(),
                    suffix
                );
            }
            None => error!("No managed path found."),
        }
        info!("Selected mod path as: {}", mods_path.as_str());
    }
}
