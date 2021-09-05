use serde::{Deserialize, Serialize};
use dotenv::dotenv;
use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::sync::{Arc, Mutex};
use notify::{RecommendedWatcher, Watcher, RecursiveMode, DebouncedEvent};
use std::sync::mpsc::channel;
use std::time::Duration;


const SETTINGS_FILE: &str = "settings.json";
const RELOAD_CONF_SECS: u64 = 2;

#[derive(Deserialize, Debug)]
pub struct SettingsWrapper {
    pub settings: Settings
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Settings {
    pub user: User,
    pub is_enabled: bool
}
#[derive(Deserialize, Serialize, Debug)]
pub struct User {
    pub server_password_iterations: u32
}

impl SettingsWrapper {
    pub fn defualt() -> Self {
        Self {
            settings: Settings::defualt(),
        }
    }
}

impl Settings {
    pub fn defualt() -> Self {
        Self {
            user: User::defualt(),
            is_enabled: false,
        }
    }
}

impl User {
    pub fn defualt() -> Self {
        Self {
            server_password_iterations: 100000,
        }
    }
}

fn get_file() -> String {
    dotenv().ok();
    let dir = env::var("SETTINGS_DIR").expect("SETTINGS_DIR must be set");
    let file = dir + SETTINGS_FILE;
    file
}

fn get_dir() -> String {
    dotenv().ok();
    let dir = env::var("SETTINGS_DIR").expect("SETTINGS_DIR must be set");
    dir
}

fn get_datetime() -> String {
    use chrono::prelude::*;
    let dt = Local::now();
    format!("{}.{}.{} {}-{}-{}", dt.hour(), dt.minute(), dt.second(), dt.day(), dt.month(), dt.year())
}

fn create_file(settings: &Settings) -> Result<SettingsWrapper, String> {
    let file = File::create(get_file());
    if file.is_ok() {
        let json_string = serde_json::to_string_pretty(&settings);
        if json_string.is_ok() {
            let written_file = file.unwrap().write_all(json_string.unwrap().as_bytes());
            if written_file.is_ok() {
                Ok(SettingsWrapper {settings: Settings::defualt()})
            }
            else {
                Err(String::from("Failed to write json data to file"))
            }
        }
        else {
            Err(String::from("Failed converting json data into a string"))
        }
    }
    else {
        Err(String::from("Failed to create file. Most likely a permission error"))
    }
}

fn init_makefile_if_does_not_exist() -> bool {
    let file = File::open(get_file());
    if file.is_ok() {
        true
    }
        else {
            let file_created = create_file(&Settings::defualt());
            file_created.is_ok()    
        }
}

pub fn readfile() -> Result<SettingsWrapper, String> {
    let file = File::open(get_file());
    if file.is_ok() {
        let reader = BufReader::new(file.unwrap());
        let json = serde_json::from_reader(reader);
        if json.is_ok() {
            Ok(SettingsWrapper{ settings: json.unwrap()})
        }
        else {
            Err(String::from("failed to convert settings file into a json object"))
        }
    }
    else {
        let settings = Settings::defualt();
        create_file(&settings)?;
        Ok(SettingsWrapper {settings: settings})
    }
}

pub fn detect_change(settings_wrapper: Arc<Mutex<SettingsWrapper>>) {
    init_makefile_if_does_not_exist();
    let (tx, rx) = channel();
    let mut watcher: RecommendedWatcher = Watcher::new(tx, Duration::from_secs(RELOAD_CONF_SECS)).unwrap();
    watcher.watch(get_file(), RecursiveMode::NonRecursive).unwrap();

    loop {
        match rx.recv() {
            Ok(event) => match event {
                DebouncedEvent::Create(_b) | DebouncedEvent::Chmod(_b) | DebouncedEvent::Write(_b) | DebouncedEvent::Remove(_b) => {
                    let json = readfile();
                    if json.is_ok() {
                        let mut data = settings_wrapper.lock().unwrap();
                        data.settings = json.unwrap().settings;
                    }
                    else {
                        use std::fs::copy;
                        let new_file = format!("{}{}{}{}", get_dir(), "settings_backup_", get_datetime(), ".json");
                        let copy_result = copy(get_file(), new_file);

                        if copy_result.is_ok() {
                            let settings_wrapper = settings_wrapper.lock().unwrap();
                            let file_created = create_file(&settings_wrapper.settings);
                            
                            if file_created.is_ok() {
                                println!("file created successfully");
                            }
                            else {
                                println!("unable to Create file");
                            }
                        }
                        else {
                            println!("failed to move bugged file");
                        }
                    }
                }
                _ => {}
            }
            Err(e) => println!("watch error: {:?}", e),
        }
    }
}