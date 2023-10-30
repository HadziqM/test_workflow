use rocket::{async_trait, fairing::{Fairing, Info, Kind}, http::Header};
use rocket::{Request,Response};
use crate::setting::CONF;

pub struct CORS;

#[async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        let allowed_origins = CONF.cors.to_vec().join(", ");
        response.set_header(Header::new("Access-Control-Allow-Origin".to_owned(), allowed_origins));
        response.set_header(Header::new("Access-Control-Allow-Methods", "POST, GET, PATCH, OPTIONS"));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}
