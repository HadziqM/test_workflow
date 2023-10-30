use rocket::figment::Figment;
use serde::Deserialize;
use std::path::Path;

#[derive(Deserialize,Clone)]
pub struct Config {
    postgres: ConfigDB,
    pub server:Domain,
    pub cors:Vec<String>
}
#[derive(Deserialize,Clone)]
pub struct Domain{
    pub domain:String
}
#[derive(Deserialize,Clone)]
struct ConfigDB {
    host:String,
    port:u16,
    user:String,
    database:String,
    password:String
}

lazy_static::lazy_static!{
    pub static ref CONF:Config = Config::default();
}

impl Default for Config {
    fn default() -> Self {
        let path = Path::new(".").join("config.json");
        let data = std::fs::read(path).unwrap();
        return serde_json::from_slice::<Config>(&data).unwrap();
    }
}



pub fn db_url() -> String{
    let conf = CONF.clone();
    format!("postgres://{}:{}@{}:{}/{}",
                conf.postgres.user,
                conf.postgres.password,
                conf.postgres.host,
                conf.postgres.port,
                conf.postgres.database)
}

pub fn db_conf() -> Figment {
    rocket::Config::figment()
        .merge(("databases.name", rocket_db_pools::Config {
            url:db_url(),
            min_connections: None,
            max_connections: 100,
            connect_timeout: 3,
            idle_timeout: None,
        }))
}

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn load_config() {
        let _ = Config::default();
    }
}
