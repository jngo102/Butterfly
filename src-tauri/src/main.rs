#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use lazy_static::lazy_static;
use reqwest;
use serde;
use serde::{Deserialize, Serialize};
use serde_json;
use serde_json::Value;
use std::env;
use std::fs;
use std::io::Cursor;
use std::path::PathBuf;
use std::str::FromStr;
use tokio;
use tokio::sync::RwLock;
use unzip::Unzipper;

static STATIC_PATHS: [&str; 6] = [
  "Program Files/Steam/steamapps/common/Hollow Knight",
  "Program Files (x86)/Steam/steamapps/common/Hollow Knight",
  "Program Files/GOG Galaxy/Games/Hollow Knight",
  "Program Files (x86)/GOG Galaxy/Games/Hollow Knight",
  "Steam/steamapps/common/Hollow Knight",
  "GOG Galaxy/Games/Hollow Knight"
];

static SUFFIXES: [&str; 3] = [
  // GOG
  "Hollow Knight_Data/Managed",
  // Steam
  "hollow_knight_Data/Managed",
  // Mac
  "Contents/Resources/Data/Managed"
];

#[derive(Debug, Deserialize, PartialEq, Serialize)]
struct Dependencies {
  #[serde(rename = "Dependency", default)]
  dependencies: Vec<String>,
}

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
  static ref MODS_PATH: RwLock<String> = RwLock::new(String::new());
  static ref MODS_JSON: RwLock<String> = RwLock::new(String::new());
  static ref ENABLED_MODS: RwLock<Vec<bool>> = RwLock::new(Vec::new());
  static ref INSTALLED_MODS: RwLock<Vec<bool>> = RwLock::new(Vec::new());
}

#[tokio::main]
async fn main() {
  println!("Initializing app..");
  auto_detect().await;
  println!("Detected HK mods path: {}", MODS_PATH.read().await.to_string());
  load_mod_list().await;
  println!("Loaded mods list.");
  get_installed_mods().await;
  println!("Got installed mods.");
  get_enabled_mods().await;
  println!("Got enabled mods.");
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![
      debug,
      disable_mod,
      enable_mod,
      fetch_enabled_mods,
      fetch_installed_mods,
      fetch_mod_list,
      install_mod,
      uninstall_mod])
    .run(tauri::generate_context!())
    .expect("Failed to run tauri application.");
}

#[tauri::command]
fn debug(msg: String) {
  println!("Debug message: {}", msg);
}

#[tauri::command]
async fn disable_mod(mod_name: String) {
  let mods_path = MODS_PATH.read().await;
  let mod_path: PathBuf = [mods_path.to_string(), mod_name.clone()].iter().collect();
  let disabled_mods_path: PathBuf = [mods_path.to_string(), String::from("Disabled")].iter().collect();
  let disabled_mod_path: PathBuf = [mods_path.to_string(), String::from("Disabled"), mod_name].iter().collect();
  if !disabled_mods_path.exists() {
    match fs::create_dir(disabled_mods_path.as_path()) {
      Ok(_) => (),
      Err(e) => println!("Failed to create Disabled folder: {}", e),
    }
  }
  if mod_path.exists() {
    match fs::rename(mod_path.as_path(), disabled_mod_path) {
      Ok(_) => (),
      Err(e) => println!("Failed to move mod directory {:?} to Disabled: {}", mod_path.into_os_string().into_string(), e),
    }
  } else {
    println!("Path {:?} does not exist.", mod_path.into_os_string().into_string());
  }
}

#[tauri::command]
async fn enable_mod(mod_name: String) {
  let mods_path = MODS_PATH.read().await;
  let mod_path: PathBuf = [mods_path.to_string(), mod_name.clone()].iter().collect();
  let disabled_mod_path: PathBuf = [mods_path.to_string(), String::from("Disabled"), mod_name].iter().collect();
  if disabled_mod_path.exists() {
    match fs::rename(disabled_mod_path.as_path(), mod_path.as_path()) {
      Ok(_) => (),
      Err(e) => println!("Failed to move mod directory {:?} from Disabled: {}", mod_path.into_os_string().into_string(), e),
    }
  } else {
    println!("Path {:?} does not exist.", mod_path.into_os_string().into_string());
  }
}

#[tauri::command]
async fn fetch_enabled_mods() -> Vec<bool> {
  ENABLED_MODS.read().await.to_vec()
}

#[tauri::command]
async fn fetch_installed_mods() -> Vec<bool> {
  INSTALLED_MODS.read().await.to_vec()
}

#[tauri::command]
async fn fetch_mod_list() -> String {
  MODS_JSON.read().await.to_string()
}

#[tauri::command]
async fn install_mod(mod_name: String, mod_link: String) {
  match reqwest::blocking::get(mod_link) {
    Ok(response) => {
      let content = response.bytes().unwrap();
      let reader = Cursor::new(content);
      let mod_path = format!("{}/{}", MODS_PATH.read().await.to_string(), mod_name);
      if !PathBuf::from_str(mod_path.as_str()).unwrap().exists() {
        match fs::create_dir(mod_path.clone()) {
          Ok(_) => (),
          Err(e) => println!("Failed to create mod folder {}: {}", mod_name, e),
        }
      }
      let zip = Unzipper::new(reader, mod_path.clone());
      match zip.unzip() {
        Ok(_) => (),
        Err(e) => println!("Failed to unzip: {}", e),
      }
    },
    Err(e) => println!("Failed to get response: {}", e),
  }
}

#[tauri::command]
async fn uninstall_mod(mod_name: String) {
  let mods_path = MODS_PATH.read().await;
  let mod_path: PathBuf = [mods_path.to_string(), mod_name.clone()].iter().collect();
  let disabled_mod_path: PathBuf = [mods_path.to_string(), String::from("Disabled"), mod_name].iter().collect();
  if mod_path.exists() {
    match fs::remove_dir_all(mod_path.as_path()) {
      Ok(_) => (),
      Err(e) => println!("Failed to remove mod directory {:?}: {}", mod_path.into_os_string().into_string(), e),
    }
  } else if disabled_mod_path.exists() {
    match fs::remove_dir_all(disabled_mod_path.as_path()) {
      Ok(_) => (),
      Err(e) => println!("Failed to remove mod directory {:?}: {}", disabled_mod_path.into_os_string().into_string(), e),
    }
  } else {
    println!("Path {:?} does not exist.", mod_path.into_os_string().into_string());
  }
}

async fn auto_detect() -> bool {
  if PathBuf::from_str(MODS_PATH.read().await.as_str()).unwrap().exists() {
    return true;
  }

  match env::consts::OS {
    "linux" => {
      match STATIC_PATHS.into_iter().find(|path| {
        let path_buf: PathBuf = [
          &env::var("user.home").unwrap(),
          ".local", 
          "share", 
          path].iter().collect(); 
        path_buf.exists()
      }) {
        Some(static_path) => {
          match SUFFIXES.into_iter().find(|suffix| {
            let path_buf: PathBuf = [
              static_path,
              suffix
            ].iter().collect();
            path_buf.exists()
          }) {
            Some(suffix) => {
              let mut mods_path = MODS_PATH.write().await;
              *mods_path = format!(
                "{}/.local/share/{}/{}/Mods", 
                &env::var("user.home").unwrap(), 
                static_path,
                suffix).to_string();
              if !PathBuf::from_str(mods_path.as_str()).unwrap().exists() {
                match fs::create_dir(mods_path.as_str()) {
                  Ok(_) => (),
                  Err(e) => println!("Error creating mods folder: {}", e),
                }
              }
            },
            None => {
              println!("No managed path exists.");
            }
          }
        },
        None => println!("No game path exists."),
      }
      true
    }
    "macos" => {
      match STATIC_PATHS.into_iter().find(|path| {
        let path_buf: PathBuf = [
          &env::var("user.home").unwrap(),
          "Library", 
          "Application Support", 
          path].iter().collect(); 
        path_buf.exists()
      }) {
        Some(static_path) => {
          match SUFFIXES.into_iter().find(|suffix| {
            let path_buf: PathBuf = [
              static_path,
              suffix
            ].iter().collect();
            path_buf.exists()
          }) {
            Some(suffix) => {
              let mut mods_path = MODS_PATH.write().await;
              *mods_path = format!(
                "{}/Library/Application Support/{}/{}/Mods", 
                &env::var("user.home").unwrap(), 
                static_path,
                suffix).to_string();
              if !PathBuf::from_str(mods_path.as_str()).unwrap().exists() {
                match fs::create_dir(mods_path.as_str()) {
                  Ok(_) => (),
                  Err(e) => println!("Error creating mods folder: {}", e),
                }
              }
            },
            None => {
              println!("No managed path exists.");
            }
          }
        },
        None => println!("No game path exists."),
      }
      true
    }
    "windows" => {
      let mut drive_letter: String = String::from("C:/");
      for i in 65u8..=90 {
        if PathBuf::from_str(format!("{}:/", i).as_str()).unwrap().exists() {
          drive_letter = format!("{}:/", i);
        }
      };
      match STATIC_PATHS.into_iter().find(|path| {
        let path_buf: PathBuf = [drive_letter.to_string(), path.to_string()].iter().collect(); 
        println!("Checking if path {} exists", path_buf.clone().into_os_string().into_string().unwrap());
        path_buf.exists()
      }) {
        Some(static_path) => {
          match SUFFIXES.into_iter().find(|suffix| {
            let path_buf: PathBuf = [
              drive_letter.as_str(),
              static_path,
              suffix
            ].iter().collect();
            println!("Checking managed path: {}", path_buf.clone().into_os_string().into_string().unwrap());
            path_buf.exists()
          }) {
            Some(suffix) => {
              let mut mods_path = MODS_PATH.write().await;
              *mods_path = format!(
                "{}{}/{}/Mods", 
                drive_letter.as_str(), 
                static_path,
                suffix).to_string();
              if !PathBuf::from_str(mods_path.as_str()).unwrap().exists() {
                match fs::create_dir(mods_path.as_str()) {
                  Ok(_) => (),
                  Err(e) => println!("Error creating mods folder: {}", e),
                }
              }
            },
            None => {
              println!("No managed path exists.");
            }
          }
        },
        None => println!("No game path exists."),
      }
      true
    }
    _ => panic!("OS not supported."),
  }
}

async fn get_enabled_mods() {
  let mods_json: Value = serde_json::from_str(&MODS_JSON.read().await).unwrap();
  let manifests = mods_json["Manifest"].as_array().unwrap();
  let mod_count = manifests.len();
  let mut enabled_mods = ENABLED_MODS.write().await;
  let mods_path = MODS_PATH.read().await.to_string();
  let disabled_path: PathBuf = [mods_path.as_str(), "Disabled"].iter().collect();
  let installed_mods = INSTALLED_MODS.read().await;
  for i in 0..mod_count {
    if !installed_mods[i] { 
      enabled_mods.push(false);
      continue; 
    }
    let mod_name = manifests[i]["Name"].as_str().unwrap();
    let mod_path: PathBuf = [mods_path.clone().as_str(), mod_name].iter().collect();
    let disabled_mod_path: PathBuf = [
      disabled_path.clone().into_os_string().to_str().unwrap(),
      mod_name
    ].iter().collect();
    enabled_mods.push(mod_path.exists() && !disabled_mod_path.exists());
  }
}

async fn get_installed_mods() {
  let mods_json: Value = serde_json::from_str(&MODS_JSON.read().await.to_string()).unwrap();
  let manifests = mods_json["Manifest"].as_array().unwrap();
  let mod_count = manifests.len();
  let mut installed_mods = INSTALLED_MODS.write().await;
  let mods_path = MODS_PATH.read().await.to_string();
  let disabled_path: PathBuf = [mods_path.as_str(), "Disabled"].iter().collect();
  for i in 0..mod_count {
    let mod_name = manifests[i]["Name"].as_str().unwrap();
    let mod_path: PathBuf = [mods_path.clone().as_str(), mod_name].iter().collect();
    let disabled_mod_path: PathBuf = [
      disabled_path.clone().into_os_string().to_str().unwrap(),
      mod_name
    ].iter().collect();
    installed_mods.push(mod_path.exists() || disabled_mod_path.exists());
  }
}

async fn load_mod_list() {
  println!("Loading mod list...");
  let content = reqwest::blocking::get(
    "https://raw.githubusercontent.com/hk-modding/modlinks/main/ModLinks.xml"
  )
  .unwrap()
  .text()
  .unwrap();
  let mut mod_links = ModLinks::new();
  match quick_xml::de::from_str(content.as_str()) {
    Ok(value) => {
      println!("Successfully parsed XML.");
      mod_links = value;
    },
    Err(e) => println!("Failed to parse XML: {}", e),
  }
  let mut mods_json = MODS_JSON.write().await;
  *mods_json = serde_json::to_string(&mod_links).unwrap();
}