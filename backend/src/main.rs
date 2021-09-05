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

#[macro_use]
mod db;
mod utils;
mod api;    

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

fn main() {
    //rocket::build().mount("/", routes![index])

    rocket::ignite().mount("/", routes![index])
        .manage(db::init_pool())
        .mount(&["", "/api"].concat(), api::core_routes())
        .launch();
}
