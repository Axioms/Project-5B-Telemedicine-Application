pub mod core;
pub mod web;

use rocket_contrib::json::Json;
use serde_json::Value;

pub use crate::api::{
    core::routes as core_routes,
    web::routes as web_routes,
};

// Type aliases for API methods results
type ApiResult<T> = Result<T, String>;
pub type JsonResult = ApiResult<Json<Value>>;
pub type StringResult = ApiResult<String>;
pub type EmptyResult = ApiResult<()>;