use luro_framework::{CommandInteraction, LuroCommand};

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

impl LuroCommand for Log {
    async fn interaction_command(self, ctx: CommandInteraction) -> anyhow::Result<luro_model::types::CommandResponse> {
        // let logger = ctx.logging.console_handler.clone();
        // TODO: Implement this
        // let (_, level) = match self.level {
        //     LogLevel::Trace => (logger.modify(|filter| { filter.with_filter(LevelFilter::TRACE); })?, "TRACE"),
        //     LogLevel::Debug => (logger.modify(|filter| { filter.with_filter(LevelFilter::DEBUG); })?, "DEBUG"),
        //     LogLevel::Info => (logger.modify(|filter| { filter.with_filter(LevelFilter::INFO); })?, "INFO"),
        //     LogLevel::Warn => (logger.modify(|filter| { filter.with_filter(LevelFilter::WARN); })?, "WARN"),
        //     LogLevel::Error => (logger.modify(|filter| { filter.with_filter(LevelFilter::ERROR); })?, "ERROR"),
        //     LogLevel::Off => (logger.modify(|filter| { filter.with_filter(LevelFilter::OFF); })?, "OFF"),
        // };

        let level = match self.level {
            LogLevel::Trace => "TRACE",
            LogLevel::Debug => "DEBUG",
            LogLevel::Info => "INFO",
            LogLevel::Warn => "WARN",
            LogLevel::Error => "ERROR",
            LogLevel::Off => "OFF",
        };

        ctx.respond(|r| r.content(format!("Luro's log level is now set to {level}!")).ephemeral())
            .await
    }
}
