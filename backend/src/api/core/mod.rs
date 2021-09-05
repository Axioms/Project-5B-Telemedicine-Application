mod accounts;
use rocket::Route;
pub fn routes() -> Vec<Route> {

    let mut routes = Vec::new();
    routes.append(&mut accounts::routes());
    routes
}