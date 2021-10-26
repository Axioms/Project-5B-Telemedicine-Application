use std::io::Cursor;
use std::env;

use rocket::{
    fairing::{Fairing, Info, Kind},
    http::{ContentType, Header, HeaderMap, Method, Status},
    Request, Response,
};  

pub struct Cors();

impl Cors {
    fn get_header(headers: &HeaderMap, name: &str) -> String {
        match headers.get_one(name) {
            Some(h) => h.to_string(),
            _ => "".to_string(),
        }
    }

    // Check a request's `Origin` header against the list of allowed origins.
    // If a match exists, return it. Otherwise, return None.
    fn get_allowed_origin(headers: &HeaderMap) -> Option<String> {
        let origin = Cors::get_header(headers, "Origin");
        let domain_origin = env::var("DOMAIN_ORIGIN").expect("DOMAIN_ORIGIN must be set");
        if origin == domain_origin {
            Some(origin)
        } else {
            None
        }
    }
}

impl Fairing for Cors {
    fn info(&self) -> Info {
        Info {
            name: "Cors",
            kind: Kind::Response,
        }
    }

    fn on_response(&self, request: &Request, response: &mut Response) {
        let req_headers = request.headers();

        if let Some(origin) = Cors::get_allowed_origin(req_headers) {
            response.set_header(Header::new("Access-Control-Allow-Origin", origin));
        }

        // Preflight request
        if request.method() == Method::Options {
            let req_allow_headers = Cors::get_header(req_headers, "Access-Control-Request-Headers");
            let req_allow_method = Cors::get_header(req_headers, "Access-Control-Request-Method");

            response.set_header(Header::new("Access-Control-Allow-Methods", req_allow_method));
            response.set_header(Header::new("Access-Control-Allow-Headers", req_allow_headers));
            response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
            response.set_status(Status::Ok);
            response.set_header(ContentType::Plain);
            response.set_sized_body(Cursor::new(""));
        }
    }
}