#![allow(unused)]

use super::Logs;
use serde::{Serialize,Deserialize};
use sqlx::FromRow;
use sqlx::Result;

//global struct for users
#[derive(Serialize,Deserialize,Clone)]
pub struct Users{
    name:String,
    pub id:i32,
    ships:Vec<UserShip>
}
#[derive(Serialize,Deserialize,Clone)]
pub struct UserShip{
    name:String,
    pub id:i32,
    pub sensors:Vec<ShipSensors>
}
#[derive(Serialize,Deserialize,FromRow,Clone)]
pub struct ShipSensors{
    pub id:i32,
    name:String,
    identifier:i32,
    pub warning_value:f32,
    min_value:f32,
    max_value:f32
}

//struct for database interfacing only
#[derive(FromRow)]
pub struct PgUsers{
    pub username:String,
    pub password:String,
    pub id:i32,
}
#[derive(FromRow)]
struct PgShips {
    name:String,
    id:i32
}

impl PgShips {
    fn into_global_format(&self,sensors:Vec<ShipSensors>) -> UserShip {
        UserShip{sensors,name:self.name.clone(),id:self.id}
    }
}
impl PgUsers {
    fn into_global_format(&self,ships:Vec<UserShip>)->Users {
        Users{ships,name:self.username.clone(),id:self.id}
    }
}

impl Logs {
    pub async fn get_user(&self,username:&str)->Result<PgUsers>  {
        sqlx::query_as::<_,PgUsers>("select id,username,password from users where username=$1")
            .bind(username)
            .fetch_one(&**self)
        .await
    }
    pub async fn get_user_ui_data(&self,user:&PgUsers)-> Result<Users> {
        let mut ship_container = vec![];
        let ships = sqlx::query_as::<_,PgShips>("select ships.id as id,ships.name as name from user_ship 
                                left outer join ships on ships.id=user_ship.ship_id where user_ship.user_id=$1 
                                order by user_ship.created_at asc")
            .bind(user.id)
            .fetch_all(&**self).await?;
        for ship in &ships {
            let sensors = sqlx::query_as::<_,ShipSensors>("select sensors.name as name, sensors.id as id, identifier, 
                                min_value, max_value from sensors where ship_id=$1")
                .bind(ship.id)
                .fetch_all(&**self).await?;
            ship_container.push(ship.into_global_format(sensors))
        }
        Ok(user.into_global_format(ship_container))
    }
}
