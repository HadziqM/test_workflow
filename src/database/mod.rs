pub mod users;
pub mod sensors;
pub mod status;
pub mod log;


use rocket_db_pools::{sqlx,Database};

//main database struct
#[derive(Database,Clone)]
#[database("name")]
pub struct Logs(sqlx::PgPool);




#[cfg(test)]
mod test{
    use super::*;
    use sqlx::Result;

    impl Logs {
        pub async fn new()->Result<Self>{
            let x = sqlx::postgres::PgPoolOptions::new()
                .max_connections(5).connect(&crate::setting::db_url()).await?;
            Ok(Self(x.into()))
        }
    }

    #[rocket::async_test]
    async fn connect(){
        let _ = Logs::new().await.unwrap();
    }
}
