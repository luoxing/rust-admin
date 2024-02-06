use std::time::Duration;

use once_cell::sync::Lazy;
use rbatis::RBatis;
use crate::config::CFG;

/// CONTEXT is all of the service struct
pub static RB: Lazy<RBatis> = Lazy::new(|| RBatis::new());

pub async fn db_init() {
    let driver = rbdc_sqlite::SqliteDriver {};
    RB.link(driver, &CFG.db.url)
        .await
        .expect("[abs_admin] rbatis pool init fail!");
    //RB.intercepts.push(Arc::new(SysTrashService::new()));
    RB.get_pool().unwrap().set_max_open_conns(CFG.db.pool_size as u64).await;
    RB.get_pool().unwrap().set_timeout(Some(Duration::from_secs(CFG.db.pool_timeout as u64))).await;
}