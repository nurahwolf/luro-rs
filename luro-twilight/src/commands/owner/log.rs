use async_trait::async_trait;
use luro_framework::{InteractionCommand, command::LuroCommandTrait, Framework, LuroInteraction};
use luro_model::database_driver::LuroDatabaseDriver;
use tracing_subscriber::filter;

use twilight_interactions::command::{CommandModel, CommandOption, CreateCommand, CreateOption};

#[derive(CommandModel, CreateCommand, Debug, PartialEq, Eq)]
#[command(name = "log", desc = "Set Luro's global log level, useful for debugging")]
pub struct Log {
    /// The level to set
    pub level: LogLevel,
}

#[derive(CommandOption, CreateOption, Clone, Debug, PartialEq, Eq)]
pub enum LogLevel {
    #[option(name = "TRACE - If you are using this something is FUCKED", value = "trace")]
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
    Off,
}

#[async_trait]
impl LuroCommandTrait for Log {
    async fn handle_interaction<D: LuroDatabaseDriver>(
        ctx: Framework,
        interaction: InteractionCommand,
    ) -> anyhow::Result<()> {
        let data = Self::new(interaction.data.clone())?;
        let (_, level) = match data.level {
            LogLevel::Trace => (
                ctx
                    .tracing_subscriber
                    .modify(|filter| *filter = filter::LevelFilter::TRACE)?,
                "TRACE",
            ),
            LogLevel::Debug => (
                ctx
                    .tracing_subscriber
                    .modify(|filter| *filter = filter::LevelFilter::DEBUG)?,
                "DEBUG",
            ),
            LogLevel::Info => (
                ctx
                    .tracing_subscriber
                    .modify(|filter| *filter = filter::LevelFilter::INFO)?,
                "INFO",
            ),
            LogLevel::Warn => (
                ctx
                    .tracing_subscriber
                    .modify(|filter| *filter = filter::LevelFilter::WARN)?,
                "WARN",
            ),
            LogLevel::Error => (
                ctx
                    .tracing_subscriber
                    .modify(|filter| *filter = filter::LevelFilter::ERROR)?,
                "ERROR",
            ),
            LogLevel::Off => (
                ctx
                    .tracing_subscriber
                    .modify(|filter| *filter = filter::LevelFilter::OFF)?,
                "OFF",
            ),
        };

        interaction.respond(&ctx, |r| r.content(format!("Luro's log level is now set to {}!", level)).ephemeral())
            .await
    }
}
