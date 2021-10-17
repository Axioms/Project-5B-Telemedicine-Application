use chrono::Utc;
use rocket_contrib::json::Json;

use crate::{
    db::{models::*, DbConn},
    api::EmptyResult,
    api::StringResult,
};

pub fn routes() -> Vec<rocket::Route> {
    routes![
        register,
        login
    ]
}

#[derive(Deserialize, Debug)]
#[allow(non_snake_case)]
struct RegisterData {
    Email: String,
    Kdf: Option<i32>,
    KdfIterations: Option<i32>,
    Key: String,
    Keys: Option<KeysData>,
    PasswordHash: String,
    MasterPasswordHint: Option<String>,
    Name: Option<String>,
    Token: Option<String>,
    OrganizationUserId: Option<String>,
}

#[derive(Deserialize, Debug)]
#[allow(non_snake_case)]
struct LoginData {
    Email: String,
    PasswordHash: String,
}

#[derive(Deserialize, Debug)]
#[allow(non_snake_case)]
struct KeysData {
    EncryptedPrivateKey: String,
    PublicKey: String,
}

#[post("/accounts/register", data = "<data>")]
fn register(data: Json<RegisterData>, conn: DbConn) -> EmptyResult {
    let email = data.Email.to_lowercase();
    let mut user = User::new(email, 100);
    user.set_password(&data.PasswordHash);
    user.save(&conn)
}

#[post("/accounts/login", data = "<data>")]
fn login(data: Json<LoginData>, conn: DbConn) -> StringResult {
    let email = data.Email.to_lowercase();

    let usr = User::find_by_email(&email, &conn).unwrap();
    
    let validated = usr.validate_password(&data.PasswordHash);

    if validated {
        return Ok(String::from("User varified"));
    }
    else {
        return Ok(String::from("unvarified user"));
    }
}