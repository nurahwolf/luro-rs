use tracing_appender::non_blocking::{NonBlocking, WorkerGuard};
use tracing_subscriber::{
    fmt::{
        format::{DefaultFields, Format},
        writer::{MakeWriterExt, WithMaxLevel},
    },
    layer::SubscriberExt,
    util::SubscriberInitExt,
};

pub type ConsoleHandler = tracing_subscriber::reload::Handle<
    tracing_subscriber::fmt::Layer<tracing_subscriber::Registry, DefaultFields, Format, WithMaxLevel<NonBlocking>>,
    tracing_subscriber::Registry,
>;

#[derive(Debug)]
pub struct Logging {
    pub guards: [WorkerGuard; 2],
    pub console_handler: ConsoleHandler,
}

/// [tracing_subscriber] default filter level
pub const DEFAULT_FILTER: tracing_subscriber::filter::LevelFilter = tracing_subscriber::filter::LevelFilter::INFO;
const DEFAULT_TRACING_LEVEL: tracing::Level = tracing::Level::INFO;
/// Path for storing hourly logs on disk.
pub const LOG_PATH: &str = "log/";

/// Setup logging. The passed parameter is used for the name of the log, such as `luro.log`.
pub fn init(file_name: &str) -> Logging {
    // let console_layer = console_subscriber::spawn();
    let file_appender = tracing_appender::rolling::hourly(LOG_PATH, format!("{file_name}.log"));
    let (file_appender_layer, file_appender_guard) = tracing_appender::non_blocking(file_appender);
    let (console_logging_layer, console_logging_guard) = tracing_appender::non_blocking(std::io::stdout());
    let (console_logging_layer, console_logging_handle) = tracing_subscriber::reload::Layer::new(
        tracing_subscriber::fmt::Layer::new().with_writer(console_logging_layer.with_max_level(DEFAULT_TRACING_LEVEL)),
    );

    tracing_subscriber::registry()
        .with(console_logging_layer)
        // .with(console_layer)
        .with(tracing_subscriber::fmt::Layer::new().with_writer(file_appender_layer.with_max_level(DEFAULT_TRACING_LEVEL)))
        .init();

    Logging {
        guards: [file_appender_guard, console_logging_guard],
        console_handler: console_logging_handle,
    }
}
