use twilight_interactions::command::{CommandModel, CommandOption, CreateCommand, CreateOption};

use crate::interaction::LuroSlash;

use self::{
    ban::Ban, kick::Kick, purge::PurgeCommand, settings::GuildSettingsCommand, warn::ModeratorWarnCommand
};
use crate::luro_command::LuroCommand;

mod assign;
mod ban;
mod kick;
mod purge;
mod settings;
pub mod warn;

#[derive(CommandOption, CreateOption, Clone, Debug, PartialEq, Eq)]
pub enum Reason {
    /// Someone who attempts to steal your money by offering fake commissions
    #[option(
        name = "Art Scam - Someone who attempts to steal your money by offering fake commissions",
        value = "art-scam"
    )]
    ArtScam,

    /// Compromised Account
    #[option(
        name = "Compromised Account - An account that has been token logged, or is spreading malware",
        value = "compromised"
    )]
    Compromised,

    /// Someone who is being a little bitch
    #[option(name = "Troll - Someone who is being a little bitch", value = "troll")]
    Troll,

    /// Someone who joined just to be a little bitch
    #[option(name = "Raider - Someone who joined just to be a little bitch", value = "raider")]
    Raider,

    /// Racist, Sexist and other such things.
    #[option(name = "Vile - Racist, Sexist and other such plesent things.", value = "")]
    Vile,

    /// A completely custom reason if the others do not fit
    #[option(
        name = "Custom Reason - A completely custom reason if the others do not fit",
        value = "custom"
    )]
    Custom
}

#[derive(CommandModel, CreateCommand, Debug, PartialEq, Eq)]
#[command(name = "mod", desc = "Commands that can be used by moderators", dm_permission = false)]
pub enum ModeratorCommands {
    #[command(name = "ban")]
    Ban(Ban),
    #[command(name = "kick")]
    Kick(Kick),
    #[command(name = "purge")]
    Purge(PurgeCommand),
    #[command(name = "settings")]
    Setting(GuildSettingsCommand),
    #[command(name = "warn")]
    Warn(ModeratorWarnCommand)
}

impl LuroCommand for ModeratorCommands {
    async fn run_command(self, ctx: LuroSlash) -> anyhow::Result<()> {
        // Call the appropriate subcommand.
        match self {
            Self::Ban(command) => command.run_command(ctx).await,
            Self::Kick(command) => command.run_command(ctx).await,
            Self::Purge(command) => command.run_command(ctx).await,
            Self::Setting(command) => command.run_command(ctx).await,
            Self::Warn(command) => command.run_command(ctx).await
        }
    }
}

pub fn reason(reason: Reason, details: Option<String>) -> String {
    let mut reason_string = match reason {
        Reason::ArtScam => "[Art Scam]".to_owned(),
        Reason::Compromised => "[Compromised Account]".to_owned(),
        Reason::Custom => String::new(),
        Reason::Raider => "[Raider]".to_owned(),
        Reason::Troll => "[Troll]".to_owned(),
        Reason::Vile => "[Vile]".to_owned()
    };

    if let Some(details) = details {
        match reason == Reason::Custom {
            true => reason_string.push_str(&details.to_string()),
            false => reason_string.push_str(&format!(" - {details}"))
        }
    }
    
    reason_string
}