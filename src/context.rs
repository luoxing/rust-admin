use std::{str::FromStr, time::Duration};

use log::{self, LevelFilter};

use sea_orm::{ConnectOptions, Database, DatabaseConnection};

use crate::config::CFG;

#[derive(Clone)]
pub struct AppState {
   pub conn: DatabaseConnection,
}

pub async fn db_init() -> DatabaseConnection {
    let mut opt = ConnectOptions::new(CFG.db.url.as_str());
    opt.max_connections(100)
    .min_connections(5)
    .connect_timeout(Duration::from_secs(8))
    .acquire_timeout(Duration::from_secs(8))
    .idle_timeout(Duration::from_secs(8))
    .max_lifetime(Duration::from_secs(8))
    .sqlx_logging(CFG.db.log)
    .sqlx_logging_level(LevelFilter::from_str(&CFG.db.log_level).unwrap())
    .set_schema_search_path("my_schema"); // Setting default PostgreSQL schema

    // Database::connect(opt).await.unwrap()
    let db = Database::connect(opt).await.unwrap();
    db
}