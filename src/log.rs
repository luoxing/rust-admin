use crate::config::CFG;
use std::io;
use tracing::Level;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::{
    fmt::{self, time::OffsetTime},
    layer::SubscriberExt,
    EnvFilter,
};
use time::{macros::format_description, UtcOffset};

pub fn init_log() -> WorkerGuard {
    // 文件输出
    let file_appender = tracing_appender::rolling::hourly(&CFG.log.path, &CFG.log.name);
    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);

    let local_time = OffsetTime::new(
        UtcOffset::from_hms(8, 0, 0).unwrap(),
        format_description!("[year]-[month]-[day] [hour]:[minute]:[second].[subsecond digits:3]"),
    );
    let log_fmt = fmt::format()
        .with_level(true)
        .with_target(true)
        .with_thread_ids(true)
        .with_timer(local_time)
        .compact();

    let collector = tracing_subscriber::registry()
        .with(EnvFilter::from_default_env().add_directive(get_log_level().into()))
        .with(
            fmt::Layer::default()
                .with_writer(io::stdout)
                .event_format(log_fmt.clone()),
        )
        .with(
            fmt::Layer::default()
                .with_writer(non_blocking)
                .event_format(log_fmt),
        );
    tracing::subscriber::set_global_default(collector).expect("Unable to set a global collector");
    guard
}


pub fn get_log_level() -> Level {
    match CFG.log.level.as_str() {
        "TRACE" => tracing::Level::TRACE,
        "DEBUG" => tracing::Level::DEBUG,
        "INFO" => tracing::Level::INFO,
        "WARN" => tracing::Level::WARN,
        "ERROR" => tracing::Level::ERROR,
        _ => tracing::Level::INFO,
    }
}
