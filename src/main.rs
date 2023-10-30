pub mod database;
pub mod setting;
pub mod cookies;
mod router;
mod cors;


use std::path::Path;
use rocket::fs::FileServer;
use rocket_db_pools::Database;


fn server() -> FileServer{
    //serve static file like css,and javascript
    let path = Path::new(".").join("astro").join("dist");
    FileServer::from(path)
}

#[rocket::launch]
async fn rocket() -> _ {
    rocket::custom(setting::db_conf())
        .mount("/",server())
        .mount("/", router::reg())
        .mount("/api/", router::api::reg())
        .attach(database::Logs::init())
        .manage(cookies::Session::default())
}
