use tracing::level_filters::LevelFilter;
use tracing_subscriber::fmt::Layer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::prelude::*;
use tracing_subscriber::Registry;

#[cfg(not(any(feature = "logs-stdout", feature = "logs-file")))]
pub fn init_logging() {
    let logging_level = LevelFilter::INFO;

    // Must enable 'tokio_unstable' cfg to use this feature.
    // For example: `RUSTFLAGS="--cfg tokio_unstable" cargo run -F common-telemetry/console -- standalone start`
    #[cfg(feature = "logs-tokio-console")]
    let tokio_console_layer = console_subscriber::ConsoleLayer::builder().with_default_env().spawn();

    let subscriber = Registry::default();
    #[cfg(feature = "logs-tokio-console")]
    let subscriber = subscriber.with(tokio_console_layer.with_filter(logging_level));

    tracing::subscriber::set_global_default(subscriber).expect("error setting global tracing subscriber");

    tracing::info!("LOGGING: Logging has started!");
}

#[cfg(any(feature = "logs-stdout", feature = "logs-file"))]
pub fn init_logging() -> Vec<tracing_appender::non_blocking::WorkerGuard> {
    let logging_level = LevelFilter::INFO;
    let mut logging_guards = vec![];

    // Stdout layer.
    #[cfg(feature = "logs-stdout")]
    let stdout_logging_layer = {
        let (stdout_writer, stdout_guard) = tracing_appender::non_blocking(std::io::stdout());
        let stdout_logging_layer = Layer::new().with_writer(stdout_writer);
        logging_guards.push(stdout_guard);
        stdout_logging_layer
    };

    // JSON log layer.
    #[cfg(feature = "logs-file")]
    let (file_logging_layer, err_file_logging_layer) = {
        let logging_directory = std::env::var("LOG_PATH").unwrap_or("./logs".to_string());
        let error_directory = std::env::var("ERROR_PATH").unwrap_or("./logs".to_string());
        let filename_prefix = std::env::var("BOT_NAME").unwrap_or("sira".to_string());

        let rolling_appender = tracing_appender::rolling::RollingFileAppender::new(
            tracing_appender::rolling::Rotation::HOURLY,
            logging_directory.clone(),
            filename_prefix.clone(),
        );
        let (rolling_writer, rolling_writer_guard) = tracing_appender::non_blocking(rolling_appender);
        let file_logging_layer = Layer::new().with_writer(rolling_writer);
        logging_guards.push(rolling_writer_guard);

        // error JSON log layer.
        let err_rolling_appender = tracing_appender::rolling::RollingFileAppender::new(
            tracing_appender::rolling::Rotation::HOURLY,
            error_directory,
            format!("{filename_prefix}-err"),
        );
        let (err_rolling_writer, err_rolling_writer_guard) = tracing_appender::non_blocking(err_rolling_appender);
        let err_file_logging_layer = Layer::new().with_writer(err_rolling_writer);
        logging_guards.push(err_rolling_writer_guard);

        (file_logging_layer, err_file_logging_layer)
    };

    // Must enable 'tokio_unstable' cfg to use this feature.
    // For example: `RUSTFLAGS="--cfg tokio_unstable" cargo run -F common-telemetry/console -- standalone start`
    #[cfg(feature = "logs-tokio-console")]
    let tokio_console_layer = console_subscriber::ConsoleLayer::builder().with_default_env().spawn();

    let subscriber = Registry::default();
    #[cfg(feature = "logs-tokio-console")]
    let subscriber = subscriber.with(tokio_console_layer);
    #[cfg(feature = "logs-stdout")]
    let subscriber = subscriber.with(stdout_logging_layer.with_filter(logging_level));
    #[cfg(feature = "logs-file")]
    let subscriber = {
        let subscriber = subscriber.with(file_logging_layer.with_filter(logging_level));
        subscriber.with(err_file_logging_layer.with_filter(LevelFilter::ERROR))
    };

    tracing::subscriber::set_global_default(subscriber).expect("error setting global tracing subscriber");

    tracing::info!("Logging has started!");
    logging_guards
}
