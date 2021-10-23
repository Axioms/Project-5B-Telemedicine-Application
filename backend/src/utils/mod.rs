pub mod auth;
pub mod crypto;
pub mod error;
pub mod files;
pub mod settings;
pub mod cache;
pub mod headers;
pub mod cors;

use uuid;

use std::sync::{Arc, Mutex};
pub type SettingsMutex = Arc<Mutex<settings::SettingsWrapper>>;

pub fn create_uuid() -> String {
    uuid::Uuid::new_v4().to_string()
}