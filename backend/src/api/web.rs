use std::path::{Path, PathBuf};
use std::env;

use rocket::{http::ContentType, response::content::Content, response::NamedFile, Route};
use rocket_contrib::json::Json;
use serde_json::Value;

use crate::{
    utils::cache::Cached,
};

pub fn routes() -> Vec<Route> {

routes![web_index, javascript, css, fonts, images, favicon]
}

#[get("/")]
fn web_index() -> Cached<Option<NamedFile>> {
    let web_root = env::var("WEB_ROOT").expect("WEB_ROOT must be set");
    Cached::short(NamedFile::open(Path::new(&web_root).join("index.html")).ok())
}

#[get("/<p..>", rank = 10)] // Only match this if the other routes don't match
fn web_files(p: PathBuf) -> Cached<Option<NamedFile>> {
    let web_root = env::var("WEB_ROOT").expect("WEB_ROOT must be set");
    Cached::long(NamedFile::open(Path::new(&web_root).join(p)).ok())
}

#[get("/js/<filename>")]
fn javascript(filename: String) -> Option<NamedFile> {
    let web_root = env::var("WEB_ROOT").expect("WEB_ROOT must be set");
    NamedFile::open(Path::new(&web_root).join("js").join(filename)).ok()
}

#[get("/css/<filename>")]
fn css(filename: String) -> Option<NamedFile> {
    let web_root = env::var("WEB_ROOT").expect("WEB_ROOT must be set");
    NamedFile::open(Path::new(&web_root).join("css").join(filename)).ok()
}

#[get("/fonts/<filename>")]
fn fonts(filename: String) -> Option<NamedFile> {
    let web_root = env::var("WEB_ROOT").expect("WEB_ROOT must be set");
    NamedFile::open(Path::new(&web_root).join("fonts").join(filename)).ok()
}

#[get("/img/<filename>")]
fn images(filename: String) -> Option<NamedFile> {
    let web_root = env::var("WEB_ROOT").expect("WEB_ROOT must be set");
    NamedFile::open(Path::new(&web_root).join("img").join(filename)).ok()
}

#[get("/favicon.ico")]
fn favicon() -> Option<NamedFile> {
    let web_root = env::var("WEB_ROOT").expect("WEB_ROOT must be set");
    NamedFile::open(Path::new(&web_root).join("favicon.ico")).ok()
}