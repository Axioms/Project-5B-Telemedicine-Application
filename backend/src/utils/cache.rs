use rocket::{
    response::{self, Responder},
    Request,
};

pub struct Cached<R>(R, String);

impl<R> Cached<R> {
    pub fn long(r: R) -> Cached<R> {
        // 7 days
        Self::ttl(r, 604800)
    }

    pub fn short(r: R) -> Cached<R> {
        // 10 minutes
        Self(r, String::from("public, max-age=600"))
    }

    pub fn ttl(r: R, ttl: u64) -> Cached<R> {
        Self(r, format!("public, immutable, max-age={}", ttl))
    }
}

impl<'r, R: Responder<'r>> Responder<'r> for Cached<R> {
    fn respond_to(self, req: &Request) -> response::Result<'r> {
        match self.0.respond_to(req) {
            Ok(mut res) => {
                res.set_raw_header("Cache-Control", self.1);
                Ok(res)
            }
            e @ Err(_) => e,
        }
    }
}
