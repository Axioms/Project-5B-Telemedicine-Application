#![feature(decl_macro)]
#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

fn main() {
    //rocket::build().mount("/", routes![index])

    rocket::ignite().mount("/", routes![index])
        .launch();
}
