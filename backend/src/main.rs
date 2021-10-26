#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

extern crate openssl;

#[macro_use]
mod db;
mod utils;
mod api;
use dotenv::dotenv;
use rocket::State;
use std::sync::{Arc, Mutex};
use std::thread;

fn init_settings(settings: utils::SettingsMutex) {
    thread::spawn(move || {
        utils::settings::detect_change(settings);
    });
}

fn check_rsa_keys() -> Result<(), ()> {
    dotenv().ok();
    // If the RSA keys don't exist, try to create them
    let priv_path = std::env::var("PRIVATE_RSA_KEY").expect("PRIVATE_RSA_KEY must be set");
    let pub_path = std::env::var("PUBLIC_RSA_KEY").expect("PUBLIC_RSA_KEY must be set");

    if !utils::files::file_exists(&priv_path) {
        let rsa_key = match openssl::rsa::Rsa::generate(2048) {
            Ok(rsa_key) => rsa_key,
            Err(e) => panic!("Error creating RSA keys: {}", e),
        };

        let priv_key = match rsa_key.private_key_to_pem() {
            Ok(priv_key) => priv_key,
            Err(e) => panic!("Error creating RSA keys: {}", e),
        };

        utils::files::write_file(&priv_path, &priv_key).unwrap();
    }

    if !utils::files::file_exists(&pub_path) {
        let rsa_key = match openssl::rsa::Rsa::private_key_from_pem(match &utils::files::read_file(&priv_path) {
            Ok(file) => file,
            Err(e) => panic!("Error loading keys: {}", e),
        }) {
            Ok(rsa_key) => rsa_key,
            Err(e) => panic!("Error creating RSA keys: {}", e),
        };

        let pub_key = match rsa_key.public_key_to_pem() {
            Ok(rsa_key) => rsa_key,
            Err(e) => panic!("Error creating RSA keys: {}", e),
        };
        match utils::files::write_file(&pub_path, &pub_key) {
            Ok(rsa_key) => rsa_key,
            Err(e) => panic!("Error saving RSA keys: {}", e),
        };
    }

    utils::auth::load_keys();
    Ok(())
}


fn main() {
    //rocket::build().mount("/", routes![index])
    check_rsa_keys().unwrap();
    let settings = Arc::new(Mutex::new(utils::settings::SettingsWrapper::defualt()));
    init_settings(settings.clone());

    rocket::ignite()
        .manage(db::init_pool())
        .mount(&["", "/api"].concat(), api::core_routes())
        .mount(&["", "/"].concat(), api::web_routes())
        .manage(settings)
        .attach(utils::headers::AppHeaders())
        .attach(utils::cors::Cors())
        .launch();
}
