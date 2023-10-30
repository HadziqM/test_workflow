pub mod api;

use rocket::{http::CookieJar, fs::NamedFile, get, State, Route, routes};
use crate::cookies::Session;

impl Session {
    async fn dashboard(&self,jar:&CookieJar<_>)->NamedFile {
        if self.in_session(jar).await.is_some() {
            let path = Path::new(".").join("astro").join("dist").join("dashboard.html");
            return  NamedFile::open(path).await.unwrap();
        }
        let path = Path::new(".").join("astro").join("dist").join("login.html");
        return  NamedFile::open(path).await.unwrap();
    }
}

#[get("/")]
async fn index(jar:&CookieJar<_>,state:&State<Session>)->NamedFile {
    state.dashboard(jar).await
}

#[get("/dashboard")]
async fn dashboard(jar:&CookieJar<_>,state:&State<Session>){
    state.dashboard(jar).await
}

pub fn reg()->Vec<Route> {
    routes![index,dashboard]
}