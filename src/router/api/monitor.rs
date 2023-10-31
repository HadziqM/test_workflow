use rocket::{get, post};
use rocket::http::CookieJar;
use crate::database::Logs;
use crate::database::log::ShipLogs;
use crate::database::status::Status;
use crate::database::users::UserShip;
use rocket::serde::json::Json;
use crate::database::sensors::{GenericSensorData,Sensors};
use crate::cookies::Session;
use rocket::State;
use rocket::response::stream::{EventStream,Event};

async fn generic_stream<'a>(jar:&CookieJar<'_>,db:&'a Logs,id:i32,state:&State<Session>,table:&str)->EventStream![Event + 'a] {
    let sens = Sensors::new(table,id);
    EventStream! {
        loop {
            match db.get_sensor_data::<GenericSensorData>(&sens,10).await {
                Ok(x)=> {
                    yield Event::json(&x);
                }
                Err(y)=>println!("{y:?}")
            }
        }
    }
}

#[get("/vibration/<id>")]
pub async fn vibration<'a>(jar:&CookieJar<'_>,db:&'a Logs,id:i32,state:&State<Session>)->EventStream![Event + 'a] {
    generic_stream(jar, db, id, state, "vibration_sens").await
}
#[get("/temp/<id>")]
pub async fn temp<'a>(jar:&CookieJar<'_>,db:&'a Logs,id:i32,state:&State<Session>)->EventStream![Event + 'a] {
    generic_stream(jar, db, id, state, "temp_sens").await
}
#[get("/flow/<id>")]
pub async fn flow<'a>(jar:&CookieJar<'_>,db:&'a Logs,id:i32,state:&State<Session>)->EventStream![Event + 'a] {
    generic_stream(jar, db, id, state, "flow_sens").await
}


#[get("/messages/<id>")]
pub async fn message(jar:&CookieJar<'_>,state:&State<Session>,db:&Logs,id:i32)-> Option<Json<Vec<ShipLogs>>> {
    if state.in_session(jar).await.is_some() {
        return Some(Json(db.get_messages(id).await.ok()?));
    }
    None
}
#[post("/status", format="json", data="<ship>" )]
pub async fn status(jar:&CookieJar<'_>,db:&Logs,state:&State<Session>,ship:Json<UserShip>)->Option<Json<Vec<Status>>> {
    if state.in_session(jar).await.is_some() {
        return Some(Json(db.get_statuses(&ship).await.ok()?));
    }
    None
}
