#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod app;
mod mod_links;

use app::app::App;
use app::profile::Profile;
use app::settings::Settings;
use directories::BaseDirs;
use futures_util::StreamExt;
use log::{error, info, warn, LevelFilter};
use mod_links::mod_links::*;
use native_dialog::{FileDialog, MessageDialog, MessageType};
use open;
use reqwest;
use serde_json;
use serde_json::{json, Value};
use sha256::digest_file;
use simple_logging;
use std::cmp::min;
use std::convert::Into;
use std::env;
use std::fs;
use std::fs::{File, ReadDir};
use std::io::{Cursor, Read, Write};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::str::FromStr;
use std::sync::{mpsc, Mutex, MutexGuard};
use sysinfo::{ProcessExt, System, SystemExt};
use tauri::{async_runtime, Manager, State};
use unzip::Unzipper;

struct AppState(Mutex<App>);

const SETTINGS_FOLDER: &str = "Butterfly";

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

fn setup_app() {
    exit_game();
    let app_state = AppState(Default::default());
    load_or_create_files();
    auto_detect(&app_state);
    let app = tauri::Builder::default()
        .manage(app_state)
        .invoke_handler(tauri::generate_handler![
            check_api_installed,
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
            import_save,
            install_mod,
            manually_install_mod,
            open_mods_folder,
            open_mod_read_me,
            reset_settings,
            set_language,
            set_profile,
            set_theme,
            toggle_api,
            uninstall_mod,
        ])
        .build(tauri::generate_context!())
        .expect("Failed to build tauri application.");

    app.run(move |app_handle, event| match event {
        tauri::RunEvent::ExitRequested { api, .. } => {
            api.prevent_exit();

            let app_state = app_handle.state::<AppState>();
            let state = app_state.0.lock().unwrap();
            let settings = state.settings.clone();
            let base_dir = BaseDirs::new().unwrap();
            let settings_dir: PathBuf = [base_dir.data_dir().to_str().unwrap(), SETTINGS_FOLDER]
                .iter()
                .collect();
            if !settings_dir.exists() {
                match fs::create_dir(settings_dir.as_path()) {
                    Ok(_) => info!("Succesfully created settings folder."),
                    Err(e) => error!("Failed to create settings folder: {}", e),
                }
            }
            let settings_path: PathBuf = [settings_dir.to_str().unwrap(), "Settings.json"]
                .iter()
                .collect();
            if settings_path.exists() {
                let settings_file = File::options()
                    .write(true)
                    .open(settings_path.as_path())
                    .unwrap();
                match serde_json::to_writer_pretty(settings_file, &settings) {
                    Ok(_) => info!("Successfully saved settings."),
                    Err(e) => error!("Failed to save settings: {}", e),
                }
            } else {
                let mut settings_file = File::create(settings_path.as_path()).unwrap();
                let settings_string = serde_json::to_string(&state.settings).unwrap();
                match settings_file.write_all(settings_string.as_bytes()) {
                    Ok(_) => info!("Successfully created new settings file."),
                    Err(e) => error!("Failed to create new settings file: {}", e),
                }
            }

            app_handle.exit(0);
        }
        _ => {}
    });
}

fn main() {
    setup_app();
}

/// Check and return whether the Modding API has been installed
#[tauri::command]
fn check_api_installed(state: State<AppState>) -> bool {
    let app_state = state.0.lock().unwrap();
    let mods_path = &app_state.settings.mods_path;
    let managed_path: PathBuf = [mods_path.as_str(), ".."].iter().collect();
    let vanilla_assembly: PathBuf = [
        managed_path.to_str().unwrap(),
        "Assembly-CSharp.dll.vanilla",
    ]
    .iter()
    .collect();
    let modded_assembly: PathBuf = [managed_path.to_str().unwrap(), "Assembly-CSharp.dll.modded"]
        .iter()
        .collect();
    vanilla_assembly.exists() && !modded_assembly.exists()
}

/// Create a new profile and save it to settings
/// # Arguments
/// * `profile_name` - The name of the new profile
/// * `mod_names` - The name of the mods that will be included in the profile
/// * `state` - The state of the application
#[tauri::command]
fn create_profile(profile_name: String, mod_names: Vec<String>, state: State<AppState>) {
    let mut app_state = state.0.lock().unwrap();
    app_state.settings.profiles.push(Profile {
        name: profile_name,
        mods: mod_names,
    });
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
fn delete_profile(profile_name: String, state: State<AppState>) {
    let mut app_state = state.0.lock().unwrap();
    (*app_state)
        .settings
        .profiles
        .retain(|p| p.name != profile_name);
}

/// Move a mod folder into the Disabled folder if it is located in the Mods folder
/// # Argumentz`
/// `mod_name` - The name of the mod folder to be moved into the Disabled folder
#[tauri::command]
fn disable_mod(mod_name: String, state: State<AppState>) {
    info!("Disabling mod {:?}", mod_name);
    let mut app_state = state.0.lock().unwrap();
    let mods_path = &app_state.settings.mods_path;
    let mod_path: PathBuf = [mods_path.clone(), mod_name.clone()].iter().collect();
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
        warn!("Path {:?} does not exist.", mod_path.to_str().unwrap());
    }

    let manifests = &app_state.settings.mod_links.manifests;
    for i in 0..manifests.len() {
        if app_state.settings.mod_links.manifests[i].name == mod_name {
            app_state.settings.mod_links.manifests[i].enabled = false;
        }
    }
}

/// Move a mod folder out of the Disabled folder if it is there
/// # Arguments
/// * `mod_name` - The name of the mod folder to move out of the Disabled folder
#[tauri::command]
fn enable_mod(mod_name: String, state: State<AppState>) {
    info!("Enabling mod {:?}", mod_name);
    let mut app_state = state.0.lock().unwrap();
    let mods_path = &app_state.settings.mods_path;
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
        warn!("Path {:?} does not exist.", mod_path.to_str().unwrap());
    }

    (*app_state)
        .settings
        .mod_links
        .manifests
        .iter_mut()
        .for_each(|m| {
            if m.name == mod_name {
                m.enabled = true;
            }
        });
}

/// Export a selected set of profiles to a JSON file
/// # Arguments
/// * `profile_names` - The names of the profiles to be exported
#[tauri::command]
fn export_profiles(profile_names: Vec<String>, state: State<AppState>) -> bool {
    let app_state = state.0.lock().unwrap();
    let profiles = &app_state.settings.profiles;
    let mut export_array = vec![];
    for profile_name in profile_names {
        for profile in profiles {
            if profile.name == profile_name {
                export_array.push(profile);
            }
        }
    }

    let export_json = json!({ "Profiles": export_array });

    let export_path = FileDialog::new()
        .set_location("~")
        .add_filter("JSON File", &["json"])
        .show_save_single_file()
        .unwrap();
    let export_path = match export_path {
        Some(path) => path,
        None => {
            error!("Path to export selected profiles to does not exist.");
            return false;
        }
    };

    let export_file = File::create(export_path.clone()).unwrap();
    match serde_json::to_writer_pretty(export_file, &export_json) {
        Ok(_) => info!(
            "Successfully exported selected profiles to new file at {:?}",
            export_path
        ),
        Err(e) => error!(
            "Failed to export selected profiles to new file at {:?}: {}",
            export_path, e
        ),
    }

    true
}

/// Fetch the progress of the mod that is currently being downloaded.
#[tauri::command]
fn fetch_current_download_progress(state: State<AppState>) -> u8 {
    let app_state = state.0.lock().unwrap();
    app_state.current_download_progress
}

/// Fetch the active profile.
#[tauri::command]
fn fetch_current_profile(state: State<AppState>) -> String {
    let app_state = state.0.lock().unwrap();
    app_state.settings.current_profile.clone()
}

/// Fetch a list of enabled mods
#[tauri::command]
fn fetch_enabled_mods(state: State<AppState>) -> Vec<Value> {
    let app_state = state.0.lock().unwrap();
    let manifests = &app_state.settings.mod_links.manifests;
    let mod_count = manifests.len();
    let mut enabled_mods = vec![];
    let mods_path = &app_state.settings.mods_path;
    let disabled_path: PathBuf = [mods_path.as_str(), "Disabled"].iter().collect();
    for i in 0..mod_count {
        let mod_name = manifests[i].name.as_str();
        let mod_version = manifests[i].name.as_str();
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
fn fetch_installed_mods(state: State<AppState>) -> Vec<Value> {
    let app_state = state.0.lock().unwrap();
    let manifests = &app_state.settings.mod_links.manifests;
    let mod_count = manifests.len();

    let mut installed_mods = vec![];
    let mods_path = &app_state.settings.mods_path;
    let disabled_path: PathBuf = [mods_path.as_str(), "Disabled"].iter().collect();
    for i in 0..mod_count {
        let mod_name = manifests[i].name.as_str();
        let mod_version = manifests[i].name.as_str();
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

    installed_mods
}

#[tauri::command]
fn fetch_language(state: State<AppState>) -> String {
    let app_state = state.0.lock().unwrap();
    app_state.settings.language.clone()
}

/// Fetch a stringified JSON containing data on mods installed that are not on ModLinks.xml
#[tauri::command]
fn fetch_manually_installed_mods(state: State<AppState>) -> String {
    let app_state = state.0.lock().unwrap();
    let mut manually_installed_mods = vec![];
    let mods_path = &app_state.settings.mods_path;
    let manifests = &app_state.settings.mod_links.manifests;
    let mut path_bufs = vec![];
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

            for i in 0..manifests.len() {
                let mod_path = mod_folder.as_ref().unwrap().path();
                let mod_name = mod_path.file_name().unwrap().to_str().unwrap();
                let manifest_name = manifests[i].name.as_str();
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
                            let enabled =
                                !String::from(mod_path.to_str().unwrap()).contains("Disabled");
                            let mod_json = json!({"name": mod_name, "enabled": enabled});
                            manually_installed_mods.push(mod_json);
                            break;
                        }
                    }
                    None => warn!(
                        "File {:?} has no extension, may be a directory.",
                        file_path.to_str().unwrap()
                    ),
                }
            }
        }
    }

    let manually_installed_json = json!(manually_installed_mods);
    let manual_json = manually_installed_json.to_string();
    info!("Manual JSON: {}", manual_json);
    manual_json
}

/// Load and return the list of mods from https://raw.githubusercontent.com/hk-modding/modlinks/main/ModLinks.xml
#[tauri::command]
fn fetch_mod_list(state: State<AppState>) -> (String, Vec<String>, Vec<String>) {
    let mut app_state = state.0.lock().unwrap();
    let client = reqwest::blocking::Client::new();
    let mut new_mods = vec![];
    let mut mods_json = "".to_string();
    let mut outdated_mods = vec![];
    match client
        .get("https://raw.githubusercontent.com/hk-modding/modlinks/main/ModLinks.xml")
        .send()
    {
        Ok(response) => {
            let content = response.text().expect("Failed to get content of mod list.");
            let mut remote_mod_links = RemoteModLinks::new();
            match quick_xml::de::from_str(content.as_str()) {
                Ok(value) => {
                    info!("Successfully parsed ModLinks XML");
                    remote_mod_links = value;
                }
                Err(e) => error!("Failed to parse ModLinks XML: {}", e),
            }

            let saved_manifests: Vec<LocalModManifest> = vec![];

            // If save mod links are empty, then this is a first run of the app.
            if saved_manifests.len() > 0 {
                for manifest in remote_mod_links.clone().manifests {
                    if !saved_manifests
                        .clone()
                        .into_iter()
                        .map(|m| serde_json::to_string(&m.name).unwrap())
                        .collect::<Vec<String>>()
                        .contains(&manifest.name)
                    {
                        new_mods.push(manifest.name.clone());
                    }

                    if saved_manifests
                        .clone()
                        .into_iter()
                        .map(|m| serde_json::to_string(&m.name).unwrap())
                        .collect::<Vec<String>>()
                        .contains(&manifest.name)
                        && !saved_manifests
                            .clone()
                            .into_iter()
                            .map(|m| serde_json::to_string(&m.version).unwrap())
                            .collect::<Vec<String>>()
                            .contains(&manifest.version)
                    {
                        outdated_mods.push(manifest.name);
                    }
                }
            }

            let mod_count = remote_mod_links.manifests.len();

            let mods_path = &app_state.settings.mods_path;
            let disabled_path: PathBuf = [mods_path.as_str(), "Disabled"].iter().collect();
            for i in 0..mod_count {
                let mod_name = &remote_mod_links.manifests[i].name;
                let mod_path: PathBuf = [mods_path.clone(), mod_name.clone()].iter().collect();
                let disabled_mod_path: PathBuf = [
                    disabled_path.clone().into_os_string().to_str().unwrap(),
                    mod_name.as_str(),
                ]
                .iter()
                .collect();
                if mod_path.exists() || disabled_mod_path.exists() {
                    remote_mod_links.manifests[i].installed = true;
                }
                if mod_path.exists() && !disabled_mod_path.exists() {
                    remote_mod_links.manifests[i].enabled = true;
                }
            }

            mods_json = serde_json::to_string_pretty(&remote_mod_links).unwrap();
            app_state.settings.mod_links = serde_json::from_str(mods_json.as_str()).unwrap();
        }
        Err(e) => error!("Failed to fetch mod links: {}", e),
    }

    (mods_json.to_string(), new_mods, outdated_mods)
}

/// Fetch all mod profiles
#[tauri::command]
fn fetch_profiles(state: State<AppState>) -> (String, String) {
    let app_state = state.0.lock().unwrap();
    let profiles = serde_json::to_string_pretty(&app_state.settings.profiles).unwrap();
    let current_profile = &app_state.settings.current_profile;
    (profiles, current_profile.to_string())
}
/// Fetch theme data
#[tauri::command]
fn fetch_theme_data(state: State<AppState>) -> (String, String, String) {
    let app_state = state.0.lock().unwrap();
    let theme_path = &app_state.settings.theme_path;
    let theme = &app_state.settings.theme;
    let mut css = "".to_string();
    if theme_path.as_str() != "" {
        let mut css_file = File::options().read(true).open(theme_path.clone()).unwrap();
        match css_file.read_to_string(&mut css) {
            Ok(css) => info!("Successfully read in from CSS file: {}", css),
            Err(e) => error!("Failed to read in from CSS File: {}", e),
        }
    }
    (theme.to_string(), theme_path.to_string(), css.to_string())
}

/// Import a set of profiles from a JSON file
#[tauri::command]
fn import_profiles(state: State<AppState>) {
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
        }
    };

    let imported_json_string = fs::read_to_string(import_path).unwrap();
    let mut imported_json: Value = serde_json::from_str(imported_json_string.as_str()).unwrap();
    let imported_profiles = imported_json["Profiles"].as_array_mut().unwrap();

    let mut app_state = state.0.lock().unwrap();
    for profile in imported_profiles {
        app_state.settings.profiles.push(Profile {
            name: profile["Name"].to_string(),
            mods: profile["Mods"]
                .as_array()
                .unwrap()
                .to_vec()
                .iter()
                .map(|value| value.to_string())
                .collect(),
        });
    }
}

/// Import a save into the game's saves folder.
/// # Arguments
/// * `save_slot` - The number of the save slot to replace
#[tauri::command]
fn import_save(save_slot: i32) {
    let import_path = FileDialog::new()
        .set_location("~")
        .add_filter("Save file", &["dat"])
        .show_open_single_file()
        .unwrap();
    let import_path = match import_path {
        Some(path) => path,
        None => {
            error!("Import path is not valid.");
            return;
        }
    };

    let base_dir = BaseDirs::new().unwrap();
    let save_path: String;
    match env::consts::OS {
        "linux" => {
            save_path = format!(
                "{}/unity3d/Team Cherry/Hollow Knight/user{}.dat",
                base_dir.config_dir().display(),
                save_slot
            );
        }
        "mac" => {
            save_path = format!(
                "{}/unity.Team Cherry.Hollow Knight/user{}.dat",
                base_dir.data_dir().display(),
                save_slot
            );
        }
        "windows" => {
            save_path = format!(
                "{}/../LocalLow/Team Cherry/Hollow Knight/user{}.dat",
                base_dir.data_dir().display(),
                save_slot
            );
        }
        _ => panic!("OS not supported."),
    }

    match fs::copy(import_path, save_path) {
        Ok(_) => info!(
            "Successfully copied save file to saves folder for slot {}.",
            save_slot
        ),
        Err(e) => error!(
            "Failed to copy save file to saves folder for slot {}: {}.",
            save_slot, e
        ),
    }
}

/// Download a mod to disk from a provided link
/// # Arguments
/// * `mod_name` - The name of the mod folder to be created
/// * `mod_version` - The downloaded mod's version
/// * `mod_link` - The download link of the mod
#[tauri::command]
fn install_mod(
    mod_name: String,
    mod_version: String,
    mod_hash: String,
    mod_link: String,
    state: State<AppState>,
) {
    info!("Installing mod {:?}", mod_name);
    let mut app_state = state.0.lock().unwrap();
    (*app_state).current_download_progress = 0;
    let mods_path = app_state.settings.mods_path.clone();
    let mod_path: PathBuf = [mods_path.as_str(), mod_name.as_str()].iter().collect();
    let disabled_mod_path: PathBuf = [mods_path.as_str(), "Disabled", mod_name.as_str()]
        .iter()
        .collect();
    if mod_path.exists() {
        // let out_of_date = check_for_update(mod_name.clone(), mod_version.clone()).await;
        // if !out_of_date {
        //     warn!("Mod {:?} already installed", mod_name);
        //     let mut current_download_progress = CURRENT_DOWNLOAD_PROGRESS.write().await;
        //     *current_download_progress = 100;
        //     return Ok(());
        // }
        enable_mod(mod_name.clone(), state.clone());
    } else if disabled_mod_path.exists() {
        // let out_of_date = check_for_update(mod_name.clone(), mod_version.clone()).await;
        // if !out_of_date {
        //     warn!("Mod {:?} already installed but is disabled, enabling it instead.", mod_name);
        //     enable_mod(mod_name).await;
        //     let mut current_download_progress = CURRENT_DOWNLOAD_PROGRESS.write().await;
        //     *current_download_progress = 100;
        //     return Ok(());
        // } else {
        //     uninstall_mod(mod_name.clone()).await;
        // }
    }

    let (tx, rx) = mpsc::channel();
    let tx = tx.clone();

    let mod_name_param = mod_name.clone();
    app_state
        .pool
        .execute(move || async_runtime::block_on(download_mod(tx, mod_name_param, mod_link, mods_path)));

    while app_state.current_download_progress < 100 {
        (*app_state).current_download_progress = rx.recv().unwrap();
    }

    {
        let mut app_state = state.0.lock().unwrap();
        for i in 0..app_state.settings.mod_links.manifests.len() {
            if app_state.settings.mod_links.manifests[i].name == mod_name {
                app_state.settings.mod_links.manifests[i].installed = true;
                app_state.settings.mod_links.manifests[i].enabled = true;
            }
        }
    }
}

async fn download_mod(tx: mpsc::Sender<u8>, name: String, url: String, mods_path: String) {
    let client = reqwest::Client::new();
    let result = client
        .get(url.clone())
        .send()
        .await
        .expect("Failed to download mod.");
    let total_size = result
        .content_length()
        .ok_or(format!("Failed to get content length from {}", url))
        .unwrap();
    let mod_path = format!("{}/{}", mods_path, name);

    if !PathBuf::from_str(mod_path.as_str()).unwrap().exists() {
        match fs::create_dir(mod_path.clone()) {
            Ok(_) => info!("Successfully created mod folder for {:?}.", name),
            Err(e) => error!("Failed to create mod folder for {:?}: {}", name, e),
        }
    }

    let extension = url.split(".").last().unwrap();
    let download_path: String;
    if extension == "zip" {
        download_path = format!("{}/temp.zip", mod_path.clone());
    } else {
        download_path = format!(
            "{}/{}",
            mod_path.clone(),
            url.clone().split("/").last().unwrap()
        );
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
            tx.send((((new as f64) / (total_size as f64)) * 100.0).floor() as u8).expect("Failed to send download progress.");
        }
    }

    /*let file_hash = digest_file(download_path.clone()).unwrap();
    if file_hash.to_lowercase() != mod_hash.to_lowercase() {
        error!("Failed to verify SHA256 of downloaded file for mod {:?}, re-downloading...", mod_name);
        install_mod(mod_name.clone(), mod_version, mod_hash, mod_link.clone()).await;
    } else {
        info!("Downloaded hash of {:?} matches with that on modlinks.", mod_name);
    }*/

    if extension == "zip" {
        let file = File::open(download_path.clone()).unwrap();
        let unzipper = Unzipper::new(file, mod_path);
        match unzipper.unzip() {
            Ok(_) => info!("Successfully unzipped contents of {}", download_path),
            Err(e) => error!("Failed to unzip contents of {}: {}", download_path, e),
        }

        fs::remove_file(download_path).unwrap();
    }
}

/// Manually install a mod from disk.
#[tauri::command]
fn manually_install_mod(state: State<AppState>) -> String {
    let mut app_state = state.0.lock().unwrap();
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
            return "".to_string();
        }
    };

    let path = Path::new(&selected_path);
    let mods_path = &app_state.settings.mods_path;
    let extension = path.extension().unwrap().to_str().unwrap();
    let mod_name = String::from(path.file_name().unwrap().to_str().unwrap())
        .replace(format!(".{}", extension).as_str(), "");
    let mod_path = format!("{}/{}", mods_path, mod_name);
    let dll_path = format!("{}/{}.dll", mod_path, mod_name);
    match fs::create_dir(Path::new(&mod_path)) {
        Ok(_) => info!(
            "Successfully created directory for manually installed mod {}",
            mod_name
        ),
        Err(e) => error!(
            "Failed to create directory for manually installed mod {}: {}",
            mod_name, e
        ),
    }

    if extension == "dll" {
        match fs::copy(selected_path.clone(), dll_path) {
            Ok(_) => info!("Successfully copied DLL from selected path to mod path for manually installed mod {}", mod_name),
            Err(e) => error!("Failed to copy DLL from selected path to mod path for manually installed mod {}: {}", mod_name, e),
        }
    } else if extension == "zip" {
        let file = File::options()
            .read(true)
            .write(true)
            .open(selected_path.clone())
            .unwrap();
        let unzipper = Unzipper::new(file, mod_path);
        match unzipper.unzip() {
            Ok(_) => info!(
                "Successfully unzipped contents of manually installed mod at {}",
                selected_path.display()
            ),
            Err(e) => error!(
                "Failed to unzip contents of manually installed mod at {}: {}",
                selected_path.display(),
                e
            ),
        }
    }

    (*app_state)
        .settings
        .mod_links
        .manifests
        .push(LocalModManifest {
            name: mod_name.clone(),
            description: String::from("No description available."),
            version: String::from("Unknown"),
            link: ModLink {
                sha256: "".to_string(),
                link: "".to_string(),
            },
            dependencies: ModDependencies {
                dependencies: vec![],
            },
            repository: "".to_string(),
            tags: Some(ModTags { tags: vec![] }),
            enabled: true,
            installed: true,
        });

    let manifests = &app_state.settings.mod_links.manifests;
    let mut exists = false;
    for manifest in manifests {
        if manifest.name == mod_name {
            exists = true;
        }
    }

    if exists {
        return "".to_string();
    }

    mod_name
}

/// Open the local folder on the file system containing all installed mods
#[tauri::command]
fn open_mods_folder(state: State<AppState>) {
    let app_state = state.0.lock().unwrap();
    let mods_path = &app_state.settings.mods_path;
    info!("Mods path: {:?}", &mods_path.as_str());
    match env::consts::OS {
        "linux" => match Command::new("xdg-open").arg(&mods_path.as_str()).spawn() {
            Ok(_) => info!("Successfully opened mods folder."),
            Err(e) => error!("Failed to open mods folder: {}", e),
        },
        "mac" => match Command::new("open").arg(&mods_path.as_str()).spawn() {
            Ok(_) => info!("Successfully opened mods folder."),
            Err(e) => error!("Failed to open mods folder: {}", e),
        },
        "windows" => {
            match Command::new("explorer")
                .arg(str::replace(&mods_path.as_str(), "/", "\\"))
                .spawn()
            {
                Ok(_) => info!("Successfully opened mods folder."),
                Err(e) => error!("Failed to open mods folder: {}", e),
            }
        }
        _ => panic!("OS not supported"),
    };
}

/// Open a mod's read me if it has one
/// # Arguments
/// * `mod_name` - The name of the mod whose readme is to be opened
#[tauri::command]
fn open_mod_read_me(mod_name: String, state: State<AppState>) {
    let app_state = state.0.lock().unwrap();
    let mods_path = &app_state.settings.mods_path;
    let mod_path = format!("{}/{}", mods_path, mod_name);
    let disabled_mod_path = format!("{}/Disabled/{}", mods_path, mod_name);
    let mut file_paths: Option<ReadDir> = None;
    if PathBuf::from_str(mod_path.as_str()).unwrap().exists() {
        file_paths = Some(fs::read_dir(mod_path).unwrap());
    } else if PathBuf::from_str(disabled_mod_path.as_str())
        .unwrap()
        .exists()
    {
        file_paths = Some(fs::read_dir(disabled_mod_path).unwrap());
    }

    let file_paths = match file_paths {
        Some(value) => value,
        None => {
            error!("The mod {:?} is not installed.", mod_name);
            return;
        }
    };

    for file_path in file_paths {
        let path_buf = file_path.unwrap().path();
        let path = Path::new(&path_buf);
        let file_name = String::from(path.file_name().unwrap().to_str().unwrap());
        let file_extension = path.extension().unwrap().to_str().unwrap();
        let file_name_no_ext = file_name.replace(format!(".{}", file_extension).as_str(), "");
        if file_name_no_ext.to_lowercase() == "readme"
            && (file_extension == "txt" || file_extension == "md")
        {
            match open::that(path) {
                Ok(_) => info!("Successfully opened read me file at {}", path.display()),
                Err(e) => error!("Failed to open read me file at {}: {}", path.display(), e),
            }
            return;
        }
    }
}

/// Resets a mod's global settings
/// # Arguments
/// * `mod_name` - The name of the mod whose global settings will be reset
#[tauri::command]
fn reset_settings(mod_name: String, state: State<AppState>) {
    let app_state = state.0.lock().unwrap();
    let mods_path = &app_state.settings.mods_path;
    let mod_path = format!("{}/{}", mods_path, mod_name);
    let file_paths = fs::read_dir(mod_path).unwrap();
    let base_dir = BaseDirs::new().unwrap();
    for file_path in file_paths {
        let path_buf = file_path.unwrap().path();
        let path = Path::new(&path_buf);
        let file_name = String::from(path.file_name().unwrap().to_str().unwrap());
        let file_extension = path.extension().unwrap().to_str().unwrap();
        let saves_path: String;
        let dll_name: String;
        if file_extension == "dll" {
            dll_name = file_name.replace(format!(".{}", file_extension).as_str(), "");
            match env::consts::OS {
                "linux" => {
                    saves_path = format!(
                        "{}/unity3d/Team Cherry/Hollow Knight",
                        base_dir.config_dir().display()
                    );
                }
                "mac" => {
                    saves_path = format!(
                        "{}/unity.Team Cherry.Hollow Knight",
                        base_dir.data_dir().display()
                    );
                }
                "windows" => {
                    saves_path = format!(
                        "{}/../LocalLow/Team Cherry/Hollow Knight",
                        base_dir.data_dir().display()
                    );
                }
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
fn set_language(language: String, state: State<AppState>) {
    let mut app_state = state.0.lock().unwrap();
    app_state.settings.language = language;
}

/// Sets the current mod profile in settings
/// # Arguments
/// * `profile_name` - The name of the profile to be set to
#[tauri::command]
fn set_profile(profile_name: String, state: State<AppState>) {
    let mut app_state = state.0.lock().unwrap();
    app_state.settings.current_profile = profile_name;
}

/// Set the global theme
/// # Arguments
/// * `theme_name` - The name of theme to be set to
#[tauri::command]
fn set_theme(theme_name: String, state: State<AppState>) {
    let mut app_state = state.0.lock().unwrap();
    app_state.settings.theme = theme_name;
}

/// Toggles the Modding API and returns whether it has been toggled on or off
#[tauri::command]
fn toggle_api(state: State<AppState>) -> bool {
    let mods_path: String;
    {
        let app_state = state.0.lock().unwrap();
        mods_path = app_state.settings.mods_path.clone();
    }
    let managed_path: PathBuf = [mods_path.as_str(), ".."].iter().collect();
    let assembly: PathBuf = [managed_path.to_str().unwrap(), "Assembly-CSharp.dll"]
        .iter()
        .collect();
    let vanilla_assembly: PathBuf = [
        managed_path.to_str().unwrap(),
        "Assembly-CSharp.dll.vanilla",
    ]
    .iter()
    .collect();
    let modded_assembly: PathBuf = [managed_path.to_str().unwrap(), "Assembly-CSharp.dll.modded"]
        .iter()
        .collect();
    if vanilla_assembly.exists() && !modded_assembly.exists() {
        // Disable the Modding API
        match fs::rename(assembly.clone(), modded_assembly) {
            Ok(_) => info!("Successfully renamed Assembly-CSharp to modded assembly backup."),
            Err(e) => error!(
                "Failed to rename Assembly-CSharp to modded assembly backup: {}",
                e
            ),
        }

        match fs::rename(vanilla_assembly, assembly) {
            Ok(_) => info!("Successfully replaced modded Assembly-CSharp with vanilla assembly."),
            Err(e) => error!(
                "Failed to replace modded Assembly-CSharp with vanilla assembly: {}",
                e
            ),
        }

        return false;
    } else if modded_assembly.exists() && !vanilla_assembly.exists() {
        // Enable the Modding API
        match fs::rename(assembly.clone(), vanilla_assembly) {
            Ok(_) => info!("Successfully renamed Assembly-CSharp to modded assembly backup."),
            Err(e) => error!(
                "Failed to rename Assembly-CSharp to modded assembly backup: {}",
                e
            ),
        }

        match fs::rename(modded_assembly, assembly) {
            Ok(_) => info!("Successfully replaced vanilla Assembly-CSharp with modded assembly."),
            Err(e) => error!(
                "Failed to replace vanilla Assembly-CSharp with modded assembly: {}",
                e
            ),
        }

        return true;
    } else if !modded_assembly.exists() && !vanilla_assembly.exists() {
        warn!("Neither the modded or vanilla assembly backups exists, downloading API.");
        let app_state = state.0.lock().unwrap();
        app_state.pool.execute(move || install_api(mods_path));
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
fn uninstall_mod(mod_name: String, state: State<AppState>) {
    info!("Uninstalling mod {:?}", mod_name);
    {
        let app_state = state.0.lock().unwrap();
        let mods_path = &app_state.settings.mods_path;
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
                Err(e) => error!(
                    "Failed to remove mod directory {:?}: {}",
                    mod_path.to_str().unwrap(),
                    e
                ),
            }
        } else if disabled_mod_path.exists() {
            match fs::remove_dir_all(disabled_mod_path.as_path()) {
                Ok(_) => info!("Successfully removed all contents for {}", mod_name),
                Err(e) => error!(
                    "Failed to remove mod directory {:?}: {}",
                    disabled_mod_path.to_str().unwrap(),
                    e
                ),
            }
        } else {
            warn!("Path {:?} does not exist.", mod_path.to_str().unwrap());
        }
    }

    {
        let manifests: Vec<LocalModManifest>;
        {
            let app_state = state.0.lock().unwrap();
            manifests = app_state.settings.mod_links.manifests.clone();
        }
        let mut app_state = state.0.lock().unwrap();
        for i in 0..manifests.len() {
            if manifests[i].name == mod_name {
                app_state.settings.mod_links.manifests[i].installed = false;
                app_state.settings.mod_links.manifests[i].enabled = false;
            }
        }
    }
}

/// Automatically detect the path to Hollow Knight executable, else prompt the user to select its path.
fn auto_detect(state: &AppState) {
    {
        let app_state = state.0.lock().unwrap();
        if app_state.settings != Settings::default() {
            return;
        }
    }

    match env::consts::OS {
        "linux" | "mac" => {
            let mut app_state = state.0.lock().unwrap();
            match STATIC_PATHS.into_iter().find(|path| {
                let base_dir = BaseDirs::new().unwrap();
                let path_buf: PathBuf = [base_dir.data_dir().to_str().unwrap(), path]
                    .iter()
                    .collect();
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
                                let base_dir = BaseDirs::new().unwrap();
                                app_state.settings.mods_path = format!(
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
                        select_game_path(app_state);
                    }
                }
                None => {
                    MessageDialog::new()
                        .set_type(MessageType::Info)
                        .set_title("Could not find Hollow Knight")
                        .set_text(
                            "Butterfly could not detect your Hollow Knight installation.\n
                            Please select the folder that contains your Hollow Knight executable.",
                        )
                        .show_alert()
                        .unwrap();
                    select_game_path(app_state)
                }
            }
        }
        "windows" => {
            let mut app_state = state.0.lock().unwrap();
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
                                [drive_letter.as_str(), game_path, suffix].iter().collect();
                            info!(
                                "Checking managed path: {}",
                                path_buf.clone().into_os_string().into_string().unwrap()
                            );
                            path_buf.exists()
                        }) {
                            Some(suffix) => {
                                app_state.settings.mods_path = format!(
                                    "{}{}/{}/Mods",
                                    drive_letter.as_str(),
                                    game_path,
                                    suffix
                                );
                            }
                            None => error!("No managed path exists."),
                        }
                    } else {
                        select_game_path(app_state);
                    }
                }
                None => select_game_path(app_state),
            }
        }
        _ => panic!("OS not supported"),
    }

    {
        let app_state = state.0.lock().unwrap();
        let mods_path = &app_state.settings.mods_path;
        if !PathBuf::from_str(mods_path.as_str()).unwrap().exists() {
            match fs::create_dir(mods_path.as_str()) {
                Ok(_) => info!("Successfully created mods directory."),
                Err(e) => error!("Error creating mods folder: {}", e),
            }
        }
    }
}

/// Close Hollow Knight before starting the installer
fn exit_game() {
    let system = System::new_all();
    for process in system.processes_by_name("hollow_knight") {
        match process.kill() {
            true => info!("Successfully killed hollow_knight process."),
            false => error!("Failed to kill hollow_knight process."),
        }
    }

    for process in system.processes_by_name("Hollow Knight") {
        match process.kill() {
            true => info!("Successfully killed Hollow Knight process."),
            false => error!("Failed to kill Hollow Knight process."),
        }
    }
}

/// Download a copy of the Modding API and replace local files with its contents if
/// their hashes do not match; Also backs up the vanilla Assembly-CSharp.dll file.
fn install_api(mods_path: String) {
    let client = reqwest::blocking::Client::new();
    let result = client
        .get("https://raw.githubusercontent.com/hk-modding/modlinks/main/ApiLinks.xml")
        .send()
        .expect("Failed to get response for ApiLinks.");
    let content = result.text().expect("Failed to get response string.");
    let mut api_links = ApiLinks::new();
    match quick_xml::de::from_str(content.as_str()) {
        Ok(value) => {
            info!("Successfully parsed API XML.");
            api_links = value;
            info!(
                "API XML\n{}",
                serde_json::to_string_pretty(&api_links).unwrap()
            );
        }
        Err(e) => error!("Failed to parse API XML: {}", e),
    }

    let managed_path: PathBuf = [mods_path.as_str(), ".."].iter().collect();
    let base_dir = BaseDirs::new().unwrap();
    let settings_dir: PathBuf = [base_dir.data_dir().to_str().unwrap(), SETTINGS_FOLDER]
        .iter()
        .collect();
    let temp_path: PathBuf = [
        settings_dir
            .into_os_string()
            .into_string()
            .unwrap()
            .as_str(),
        "..",
        "Temp",
    ]
    .iter()
    .collect();
    let api_url: String;
    match env::consts::OS {
        "linux" => {
            api_url = String::from(
                "https://github.com/hk-modding/api/releases/latest/download/ModdingApiLinux.zip",
            )
        }
        "mac" => {
            api_url = String::from(
                "https://github.com/hk-modding/api/releases/latest/download/ModdingApiMac.zip",
            )
        }
        "windows" => {
            api_url = String::from(
                "https://github.com/hk-modding/api/releases/latest/download/ModdingApiWin.zip",
            )
        }
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
        }
        Err(e) => error!("Failed to get response: {}", e),
    }

    for file in api_links.manifest.files.files {
        let temp_file: PathBuf = [temp_path.to_str().unwrap(), file.as_str()]
            .iter()
            .collect();
        let local_file: PathBuf = [managed_path.to_str().unwrap(), file.as_str()]
            .iter()
            .collect();
        if !local_file.exists() {
            match fs::rename(temp_file, local_file) {
                Ok(_) => info!(
                    "Successfully moved temp file for {:?} to Managed folder.",
                    file
                ),
                Err(e) => error!(
                    "Failed to move temp file for {:?} to Managed folder: {}",
                    file, e
                ),
            }
        } else if digest_file(temp_file.clone()).unwrap()
            != digest_file(local_file.clone()).unwrap()
        {
            if file == "Assembly-CSharp.dll" {
                let vanilla_backup: PathBuf = [
                    managed_path.to_str().unwrap(),
                    "Assembly-CSharp.dll.vanilla",
                ]
                .iter()
                .collect();
                match fs::rename(local_file.clone(), vanilla_backup) {
                    Ok(_) => info!("Successfully backed up vanilla Assembly-CSharp."),
                    Err(e) => error!("Failed to backup vanilla Assembly-Csharp: {}", e),
                }
            }
            match fs::rename(temp_file, local_file) {
                Ok(_) => info!(
                    "Successfully replaced old local file for {:?} with new API file.",
                    file
                ),
                Err(e) => error!(
                    "Failed to replace old local file for {:?} with new API file: {}",
                    file, e
                ),
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
fn load_or_create_files() {
    let base_dir = BaseDirs::new().unwrap();
    let settings_dir: PathBuf = [base_dir.data_dir().to_str().unwrap(), SETTINGS_FOLDER]
        .iter()
        .collect();
    if !settings_dir.exists() {
        match fs::create_dir(settings_dir.as_path()) {
            Ok(_) => info!("Created settings and log directory"),
            Err(e) => error!("Failed to create settings folder: {}", e),
        }
    }

    let settings_string = settings_dir.to_str().unwrap();
    let log_path = format!("{}/Log.txt", settings_string);
    match simple_logging::log_to_file(log_path.as_str(), LevelFilter::Info) {
        Ok(_) => info!("Opened logger at: {}", log_path.as_str()),
        Err(e) => {
            println!("Failed to open logger: {}", e);
            return;
        }
    }
}

/// Manually select the path of the game's executable
fn select_game_path(mut app: MutexGuard<App>) {
    warn!("Selecting game path manually.");
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
            path_buf.clone().to_str().unwrap()
        );
        path_buf.exists()
    }) {
        Some(suffix) => {
            (*app).settings.mods_path =
                format!("{}/{}/Mods", selected_path.to_str().unwrap(), suffix);
        }
        None => error!("No managed path found."),
    }
    info!("Selected mod path as: {}", app.settings.mods_path);
}
