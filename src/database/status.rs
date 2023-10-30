#![allow(unused)]

use super::Logs;
use super::users::UserShip;
use serde::Serialize;
use sqlx::{FromRow,Result};
use rocket::tokio::task::JoinSet;

#[derive(Debug,Serialize,FromRow)]
pub struct Status {
    id: i32,
    status:bool
}

impl Logs {
    async fn for_spawn(self,id:i32)->Result<Status>{
        sqlx::query_as::<_,Status>("select id,status from sensors where id=$1").bind(id).fetch_one(&*self).await
    }
    pub async fn get_statuses(&self,ship:&UserShip)->Result<Vec<Status>> {
        let mut multi_thread = JoinSet::new();
        let mut status = vec![];
        for sensor in &ship.sensors {
            let rc_clone = self.clone();
            let id = sensor.id;
            multi_thread.spawn(async move{
                rc_clone.for_spawn(id).await
            });
        }
        while let Some(pat) = multi_thread.join_next().await {
            status.push(pat.unwrap()?)
        }
        Ok(status)
    }
}
