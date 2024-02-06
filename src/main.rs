use axum::{routing::get, Router};
use axum_server::{tls_rustls::RustlsConfig, Handle};
use tokio::signal;
use std::{net::SocketAddr, time::Duration};



#[macro_use]
extern crate tracing;
use rbatis::rbdc::datetime::DateTime;

#[macro_use]
extern crate rbatis;

mod config;
mod log;
mod context;
mod domain;
mod middleware;
use config::CFG;
use context::RB;

#[tokio::main]
async fn main() {
    let _guard = log::init_log();
    info!("Startingconfig is {:?}", &CFG.app_name);

    context::db_init().await;
    domain::tables::sync_tables(&RB).await;

    // configure certificate and private key used by https
    let config = RustlsConfig::from_pem_file(&CFG.server.pem_cert_path, &CFG.server.pem_key_path)
        .await
        .unwrap();

    let app = Router::new().route("/", get(handler));

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