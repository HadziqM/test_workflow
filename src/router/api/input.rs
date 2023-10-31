use rocket::{post, serde::json::Json, http::Status};
use crate::database::{Logs, sensors::SensorInput};


#[post("/input/general", format="json", data="<sensor>")]
pub async fn general(sensor:Json<SensorInput>,db:&Logs,)->Status {
    if let Err(e) =  db.write_sensor_data(&sensor).await {
        println!("{e:?}");
    }
    Status::Ok
}