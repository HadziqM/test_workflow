use std::collections::HashMap;
use std::ops::Deref;
use crate::database::users::PgUsers;
use rocket::http::{CookieJar,Cookie};
use async_mutex::Mutex;
use uuid::Uuid;


pub struct Session(Mutex<HashMap<String,PgUsers>>);

impl Default for Session {
    fn default() -> Self {
        Session(Mutex::new(HashMap::new()))
    }
}

impl Deref for Session {
    type Target = Mutex<HashMap<String,PgUsers>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Session {
    pub async fn add_session(&self,user:PgUsers,jar:&CookieJar<'_>) {
        let new_id = Uuid::new_v4().to_string();
        self.lock().await.insert(new_id.clone(),user);
        jar.add(Cookie::new("SESSION_TOKEN".to_owned(), new_id.to_owned()))
    }
    pub async fn in_session(&self,jar:&CookieJar<'_>)->Option<PgUsers>{
        if let Some(token) = jar.get("SESSION_TOKEN"){
            if let Some(user) = self.lock().await.get(token.value()) {
                return Some(user.to_owned());
            }
        }
        None
    }
}
