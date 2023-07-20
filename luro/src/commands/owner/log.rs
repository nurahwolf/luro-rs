use tracing_subscriber::filter;
use twilight_interactions::command::{CommandModel, CommandOption, CreateCommand, CreateOption};
use twilight_model::application::interaction::Interaction;

use crate::{functions::interaction_context, interactions::InteractionResponse, LuroContext, SlashResponse};

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

impl LogCommand {
    pub async fn run(self, interaction: &Interaction, ctx: &LuroContext) -> SlashResponse {
        let ephemeral = ctx.defer_interaction(interaction, true).await?;
        let (_, _, _) = interaction_context(interaction, "owner log")?;

        let (_, level) = match self.level {
            LogLevel::Trace => (
                ctx.tracing_subscriber.modify(|filter| *filter = filter::LevelFilter::TRACE)?,
                "TRACE"
            ),
            LogLevel::Debug => (
                ctx.tracing_subscriber.modify(|filter| *filter = filter::LevelFilter::DEBUG)?,
                "DEBUG"
            ),
            LogLevel::Info => (
                ctx.tracing_subscriber.modify(|filter| *filter = filter::LevelFilter::INFO)?,
                "INFO"
            ),
            LogLevel::Warn => (
                ctx.tracing_subscriber.modify(|filter| *filter = filter::LevelFilter::WARN)?,
                "WARN"
            ),
            LogLevel::Error => (
                ctx.tracing_subscriber.modify(|filter| *filter = filter::LevelFilter::ERROR)?,
                "ERROR"
            ),
            LogLevel::Off => (
                ctx.tracing_subscriber.modify(|filter| *filter = filter::LevelFilter::OFF)?,
                "OFF"
            )
        };

        Ok(InteractionResponse::Content {
            content: format!("Luro's log level is now set to {}!", level),
            ephemeral
        })
    }
}
