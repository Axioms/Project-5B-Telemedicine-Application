mod accounts;
mod reports;
use rocket::Route;
pub fn routes() -> Vec<Route> {

    let mut routes = Vec::new();
    routes.append(&mut accounts::routes());
    routes.append(&mut reports::routes());
    routes
}