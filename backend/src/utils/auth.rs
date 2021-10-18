use chrono::{Duration, Utc};
use once_cell::sync::Lazy;
use std::env;

use jsonwebtoken::{self, Algorithm, DecodingKey, EncodingKey, Header};
use serde::de::DeserializeOwned;
use serde::ser::Serialize;

use crate::{
    utils::files::{read_file, },
    db::models::User
};

const JWT_ALGORITHM: Algorithm = Algorithm::RS256;

// Tokens are valid for 2 hours
pub static DEFAULT_VALIDITY: Lazy<Duration> = Lazy::new(|| Duration::hours(2));
static JWT_HEADER: Lazy<Header> = Lazy::new(|| Header::new(JWT_ALGORITHM));

pub static JWT_LOGIN_ISSUER: Lazy<String> = Lazy::new(|| format!("{}|login", env::var("DOMAIN_ORIGIN").expect("DOMAIN_ORIGIN must be set")));
static JWT_DELETE_ISSUER: Lazy<String> = Lazy::new(|| format!("{}|delete", env::var("DOMAIN_ORIGIN").expect("DOMAIN_ORIGIN must be set")));

static PRIVATE_RSA_KEY_VEC: Lazy<Vec<u8>> = Lazy::new(|| {
    read_file(&env::var("PRIVATE_RSA_KEY").expect("PRIVATE_RSA_KEY must be set")).unwrap_or_else(|e| panic!("Error loading private RSA Key.\n{}", e))
});
static PRIVATE_RSA_KEY: Lazy<EncodingKey> = Lazy::new(|| {
    EncodingKey::from_rsa_pem(&PRIVATE_RSA_KEY_VEC).unwrap_or_else(|e| panic!("Error decoding private RSA Key.\n{}", e))
});
static PUBLIC_RSA_KEY_VEC: Lazy<Vec<u8>> = Lazy::new(|| {
    read_file(&env::var("PUBLIC_RSA_KEY").expect("PUBLIC_RSA_KEY must be set")).unwrap_or_else(|e| panic!("Error loading public RSA Key.\n{}", e))
});
static PUBLIC_RSA_KEY: Lazy<DecodingKey> = Lazy::new(|| {
    DecodingKey::from_rsa_pem(&PUBLIC_RSA_KEY_VEC).unwrap_or_else(|e| panic!("Error decoding public RSA Key.\n{}", e))
});

pub fn load_keys() {
    Lazy::force(&PRIVATE_RSA_KEY);
    Lazy::force(&PUBLIC_RSA_KEY);
}

pub fn encode_jwt<T: Serialize>(claims: &T) -> String {
    match jsonwebtoken::encode(&JWT_HEADER, claims, &PRIVATE_RSA_KEY) {
        Ok(token) => token,
        Err(e) => panic!("Error encoding jwt {}", e),
    }
}

fn decode_jwt<T: DeserializeOwned>(token: &str, issuer: String) -> Result<T, jsonwebtoken::errors::Error> {
    let validation = jsonwebtoken::Validation {
        leeway: 30, // 30 seconds
        validate_exp: true,
        validate_nbf: true,
        aud: None,
        iss: Some(issuer),
        sub: None,
        algorithms: vec![JWT_ALGORITHM],
    };

    let token = token.replace(char::is_whitespace, "");
    let token_data = match jsonwebtoken::decode(&token, &PUBLIC_RSA_KEY, &validation) {
        Ok(c) => Ok(c.claims),
        Err(err) => match *err.kind() {
            jsonwebtoken::errors::ErrorKind::InvalidToken => panic!("Token is invalid"), // Example on how to handle a specific error
            jsonwebtoken::errors::ErrorKind::InvalidIssuer => panic!("Issuer is invalid"), // Example on how to handle a specific error
            _ => panic!("Some other errors"),
        }
    };

    token_data
}

pub fn decode_login(token: &str) -> Result<LoginJwtClaims, jsonwebtoken::errors::Error> {
    decode_jwt(token, JWT_LOGIN_ISSUER.to_string())
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginJwtClaims {
    // Not before
    pub nbf: i64,
    // Expiration time
    pub exp: i64,
    // Issuer
    pub iss: String,
    // Subject
    pub sub: String,

    pub name: String,
    pub email: String,

    // user security_stamp
    pub sstamp: String,
    // [ "api", "offline_access" ]
    pub scope: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BasicJwtClaims {
    // Not before
    pub nbf: i64,
    // Expiration time
    pub exp: i64,
    // Issuer
    pub iss: String,
    // Subject
    pub sub: String,
}

pub fn generate_login_claims(user: User) -> LoginJwtClaims {
    let time_now = Utc::now().naive_utc();
    let issuer = JWT_LOGIN_ISSUER.clone();
    let scope: Vec<String> = user.scope.split(" ").map(|s| s.to_string()).collect();
    LoginJwtClaims {
        nbf: time_now.timestamp(),
        exp: ((time_now + *DEFAULT_VALIDITY).timestamp()),
        iss: issuer,
        sub: user.uuid,
        name: user.username,
        // @TODO fix
        email: user.public_key,
        sstamp: user.security_stamp,
        scope: scope,
    }
}

pub fn generate_delete_claims(uuid: String) -> BasicJwtClaims {
    let time_now = Utc::now().naive_utc();
    BasicJwtClaims {
        nbf: time_now.timestamp(),
        exp: (time_now + Duration::days(5)).timestamp(),
        iss: JWT_DELETE_ISSUER.to_string(),
        sub: uuid,
    }
}

//
// Bearer token authentication
//
use rocket::request::{FromRequest, Outcome, Request};