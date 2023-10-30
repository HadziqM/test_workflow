#![allow(unused)]

use super::Logs;
use sqlx::{FromRow,Result};
use serde::{Serialize,Deserialize};
use chrono::NaiveDateTime;

#[derive(Debug,Serialize,FromRow)]
pub struct ShipLogs {
    message: String,
    severity: i32,
    created_at:NaiveDateTime
}

#[derive(Deserialize)]
pub struct Message{
    pub message: String,
    pub severity:i32,
    pub global:bool,
    pub ship_id:i32
}

impl Logs {
    pub async fn get_messages(&self,ship_id:i32)->Result<Vec<ShipLogs>> {
        sqlx::query_as::<_,ShipLogs>("select message,severity,created_at from ship_log where ship_id=$1 or global=TRUE")
            .bind(ship_id).fetch_all(&**self).await
    }
    pub async fn write_message(&self,msg:&Message)->Result<()> {
        sqlx::query("insert into ship_log (message,ship_id,severity,global) values ($1,$2,$3,$4)")
            .bind(msg.message.clone())
            .bind(msg.ship_id)
            .bind(msg.severity)
            .bind(msg.global)
            .execute(&**self).await;
        Ok(())
    }
}
