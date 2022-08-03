use rocket::async_trait;
use rocket::fairing::{Fairing, Kind, Info};
use rocket::http::Header;
use rocket::request::Request;
use rocket::response::Response;

pub struct CorsFairing;

#[async_trait]
impl Fairing for CorsFairing {
    fn info(&self) -> Info {
        Info {
            name: "cors config",
            kind: Kind::Response
        }
    }

    async fn on_response<'r>(&self, _req: &'r Request<'_>, resp: &mut Response<'r>) {
        resp.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        resp.set_header(Header::new("Access-Control-Allow-Methods", "POST, GET, PUT, OPTIONS"));
        resp.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        resp.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

pub fn stage() -> CorsFairing {
    CorsFairing{}
}
