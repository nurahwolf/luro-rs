use async_trait::async_trait;
use tracing_subscriber::filter;

use twilight_interactions::command::{CommandModel, CommandOption, CreateCommand, CreateOption};

use crate::models::LuroSlash;

use crate::traits::luro_command::LuroCommand;
#[derive(CommandModel, CreateCommand, Debug, PartialEq, Eq)]
#[command(name = "log", desc = "Set Luro's global log level, useful for debugging")]
pub struct LogCommand {
    /// The level to set
    pub level: LogLevel
}

#[derive(CommandOption, CreateOption, Clone, Debug, PartialEq, Eq)]
pub enum LogLevel {
    #[option(name = "TRACE - Holy shit, if you are using this something is FUCKED", value = "trace")]
    Trace,
    #[option(name = "DEBUG - Extra information to know when it broke", value = "debug")]
    Debug,
    #[option(name = "INFO - Useful information, the default", value = "info")]
    Info,
    #[option(name = "WARN - Include recoverable errors, useful for production", value = "warn")]
    Warn,
    #[option(name = "ERROR - Only interested in errors that break Luro in some way", value = "error")]
    Error,
    #[option(name = "OFF - You have balls if you use this.", value = "off")]
    Off
}

#[async_trait]
impl LuroCommand for LogCommand {
    async fn run_command(self, ctx: LuroSlash) -> anyhow::Result<()> {
        let (_, level) = match self.level {
            LogLevel::Trace => (
                ctx.luro
                    .tracing_subscriber
                    .modify(|filter| *filter = filter::LevelFilter::TRACE)?,
                "TRACE"
            ),
            LogLevel::Debug => (
                ctx.luro
                    .tracing_subscriber
                    .modify(|filter| *filter = filter::LevelFilter::DEBUG)?,
                "DEBUG"
            ),
            LogLevel::Info => (
                ctx.luro
                    .tracing_subscriber
                    .modify(|filter| *filter = filter::LevelFilter::INFO)?,
                "INFO"
            ),
            LogLevel::Warn => (
                ctx.luro
                    .tracing_subscriber
                    .modify(|filter| *filter = filter::LevelFilter::WARN)?,
                "WARN"
            ),
            LogLevel::Error => (
                ctx.luro
                    .tracing_subscriber
                    .modify(|filter| *filter = filter::LevelFilter::ERROR)?,
                "ERROR"
            ),
            LogLevel::Off => (
                ctx.luro
                    .tracing_subscriber
                    .modify(|filter| *filter = filter::LevelFilter::OFF)?,
                "OFF"
            )
        };

        ctx.content(format!("Luro's log level is now set to {}!", level))
            .respond()
            .await
    }
}
