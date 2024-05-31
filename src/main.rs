use axum::extract::State;
use axum::{routing::get, Router};
use axum_server::{tls_rustls::RustlsConfig, Handle};
use casbin::function_map::key_match2;
use sea_orm::*;
use tokio::signal;
use std::path::PathBuf;
use std::{net::SocketAddr, time::Duration};
use casbin::{CoreApi, DefaultModel, Enforcer};
use casbin_adapter::SeaOrmAdapter;

#[macro_use]
extern crate tracing;

mod config;
mod log;
mod context;
mod middleware;
mod constants;
pub mod error;
use crate::middleware::casbin::CasbinAxumLayer;
use crate::config::CFG;
use crate::context::AppState;
use crate::context::db_init;
use crate::config::CASBIN_MODEL;

#[tokio::main]
async fn main() {
    let _guard = log::init_log();
    info!("Starting config is {:?}", &CFG.app_name);

    // configure certificate and private key used by https
    let config = RustlsConfig::from_pem_file(&CFG.server.pem_cert_path, &CFG.server.pem_key_path)
        .await
        .unwrap();

    // db
    let conn =  db_init().await;

    // casbin load
    let m = DefaultModel::from_str(CASBIN_MODEL).await.unwrap();
    let a = SeaOrmAdapter::new(conn.clone()).await.unwrap();
    let casbin_middleware = CasbinAxumLayer::new(m, a).await.unwrap();
    casbin_middleware
        .write()
        .await
        .get_role_manager()
        .write()
        .matching_fn(Some(key_match2), None);

    let state = AppState { conn };

    let app = Router::new()
    .route("/", get(handler))
    .with_state(state)
    .layer(casbin_middleware);

    //Create a handle for our TLS server so the shutdown signal can all shutdown
    let handle = Handle::new();
    // Spawn a task to shutdown server.
    tokio::spawn(shutdown_signal(handle.clone()));


    let addr = SocketAddr::new(CFG.server.address.parse().unwrap(), CFG.server.port);
    info!("listening on {addr}");
    // run https server
    match CFG.server.tls {
        true => axum_server::bind_rustls(addr, config)
            .handle(handle)
            .serve(app.into_make_service())
            .await
            .unwrap(),
        false => axum_server::bind(addr)
            .handle(handle)
            .serve(app.into_make_service())
            .await
            .unwrap(),
    }
}

async fn handler() -> &'static str {
    info!("Hello, World!");
    "Hello, World!"
}

async fn shutdown_signal(handle: axum_server::Handle) {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    info!("Received termination signal shutting down");
    handle.graceful_shutdown(Some(Duration::from_secs(10))); // 10 secs is how long docker will wait
                                                             // to force shutdown
}
