#![allow(unused)]

use sqlx::FromRow;
use chrono::NaiveDateTime;
use sqlx::Result;
use super::Logs;
use rocket::async_trait;
use serde::{Deserialize,Serialize};

//own struct to make sensor data fetching easier
pub struct Sensors {
    table:String,
    id:i32
}
#[derive(Deserialize)]
pub struct SensorInput{
    id:i32,
    data:u32,
    table:String
}
#[derive(FromRow)]
struct SensorCalibrate{
    zero_value:f32,
    factor:f32
}

#[derive(FromRow,Serialize,Clone,Debug)]
pub struct GenericSensorData {
    data:f32,
    created_at:NaiveDateTime
}

#[async_trait]
pub trait SensorQuery {
    async fn latest(sens:&Sensors,quantity:u32,log:&Logs)-> Result<Vec<Self>> where Self:Sized;
}

#[async_trait]
impl SensorQuery for GenericSensorData {
    async fn latest(sens:&Sensors,quantity:u32,log:&Logs)-> Result<Vec<Self>>{
        sqlx::query_as::<_,GenericSensorData>(&format!("select data,created_at from {} where sens_id = {} order by created_at desc limit {}"
            ,sens.table,sens.id,quantity)).fetch_all(&**log).await
    }
}

impl Sensors {
    pub fn new(table:&str,id:i32)->Self {
        Self{table:table.to_owned(),id}
    }
}

impl Logs {
    pub async fn get_sensor_data<T:SensorQuery>(&self,sens:&Sensors,quantity:u32)->Result<Vec<T>> {
        T::latest(sens,quantity,&self).await
    }
    pub async fn write_sensor_data(&self,input:&SensorInput)->Result<()>{
        let calibrate = sqlx::query_as::<_,SensorCalibrate>("select zero_value,factor from sensors where id=$1")
            .bind(input.id)
            .fetch_one(&**self).await?;
        let data = (input.data as f32 - calibrate.zero_value) * calibrate.factor;
        sqlx::query(&format!("insert into {} (data) values ({})",input.table,data))
            .execute(&**self).await?;
        Ok(())
    }
}



