use rocket::{post, serde::json::Json, http::CookieJar, State, get};

use crate::{database::{Logs, users::Users}, cookies::Session};
use serde::Deserialize;

#[derive(Deserialize,Clone)]
pub struct FormUser{
    username:String,
    password:String
}


#[post("/login" ,format="json" ,data="<form>")]
pub async fn login(form:Json<FormUser>,db:&Logs,jar:&CookieJar<'_>,state:&State<Session>)->String{
    if let Ok(pg) = db.get_user(&form.username).await{
        if let Ok(verify) = bcrypt::verify(&form.password, &pg.password){
            if verify {
                state.add_session(pg,jar).await;
                return "success".into();
            }
            return  "password wrong".into();
        }
    }
    return "username wrong".to_owned();
}

#[get("/initial")]
pub async fn ships(db:&Logs,jar:&CookieJar<'_>,state:&State<Session>)->Option<Json<Users>> {
    if let Some(pg) = state.in_session(jar).await {
        return Some(Json(db.get_user_ui_data(&pg).await.ok()?));
    }
    None
}