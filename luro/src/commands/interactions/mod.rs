use twilight_interactions::command::{CommandOption, CreateOption};

use crate::{
    models::{interaction::InteractionContext, CreateCommand},
    responses::StandardResponse,
};

#[cfg(feature = "command-about")]
mod about;
#[cfg(feature = "command-ban")]
mod ban;
#[cfg(feature = "command-base64")]
mod base64;
#[cfg(feature = "command-boop")]
mod boop;
#[cfg(feature = "command-dice")]
mod dice;
#[cfg(feature = "command-hello")]
mod hello;
#[cfg(feature = "command-owner")]
mod owner;
#[cfg(feature = "command-ping")]
mod ping;
mod test;
#[cfg(feature = "command-uwu")]
mod uwu;

mod kick;
mod unban;
mod warn;

pub fn default_commands() -> Vec<twilight_model::application::command::Command> {
    vec![
        #[cfg(feature = "command-about")]
        about::About::setup_command(),
        #[cfg(feature = "command-ban")]
        ban::Ban::setup_command(),
        #[cfg(feature = "command-base64")]
        base64::Base64::setup_command(),
        #[cfg(feature = "command-boop")]
        boop::Boop::setup_command(),
        #[cfg(feature = "command-dice")]
        dice::Dice::setup_command(),
        #[cfg(feature = "command-uwu")]
        uwu::UwU::setup_command(),
        #[cfg(feature = "command-ping")]
        ping::Ping::setup_command(),
        #[cfg(feature = "command-owner")]
        owner::Owner::setup_command(),
        kick::Command::setup_command(),
        warn::Command::setup_command(),
        unban::Command::setup_command(),
        // test::test_command_v2().twilight_command(),
    ]
}

/// Handle a command spawned from an interaction context
pub async fn interaction_handler(mut framework: InteractionContext) {
    tracing::info!("Handling interaction `{}`", framework.command_name());

    let response = match framework.command_name() {
        "about" => about::About::interaction_handler(&mut framework).await,
        "ban" => ban::Ban::interaction_handler(&mut framework).await,
        "base64" | "base64-encode" | "base64-decode" => base64::Base64::interaction_handler(&mut framework).await,
        "boop" => boop::Boop::interaction_handler(&mut framework).await,
        "dice" => dice::Dice::interaction_handler(&mut framework).await,
        "uwu" => uwu::UwU::interaction_handler(&mut framework).await,
        "ping" => ping::Ping::interaction_handler(&mut framework).await,
        "owner" => owner::Owner::interaction_handler(&mut framework).await,
        "unban" => unban::Command::interaction_handler(&mut framework).await,
        "kick" => kick::Command::interaction_handler(&mut framework).await,
        "warn" => warn::Command::interaction_handler(&mut framework).await,

        name => framework.standard_response(StandardResponse::UnknownCommand(name)).await,
    };

    // Exit early if no error happened
    let error = match response {
        Ok(_) => return,
        Err(why) => why,
    };

    if let Err(why) = error.handle(&framework).await {
        // The error handler... Had an error. Might want to log that.
        tracing::warn!(?why, "The error handler had an error itself");
    }
}

// =====
// Shared Structs Below
// =====

#[derive(CommandOption, CreateOption)]
pub enum PunishmentReason {
    #[option(name = "[Artist Scam] - Offers fake commissions", value = "[Artist Scam]")]
    ArtistScam,
    #[option(
        name = "[Compromised Account] - An account that has been compromised",
        value = "[Compromised Account]"
    )]
    CompormisedAccount,
    #[option(name = "[Custom] - Write your own reason", value = "")]
    Custom,
    #[option(name = "[Raider] - Someone who joined just to cause trouble", value = "[Raider]")]
    Raider,
    #[option(name = "[Vile] - Gross misconduct and other more extreme infractions", value = "[Vile]")]
    Vile,
    #[option(name = "[Inactive] - User is inactive / never messaged in the server", value = "[Inactive]")]
    Inactive,
}

impl PunishmentReason {
    pub fn fmt(&self, details: Option<String>) -> String {
        let mut reason = self.value().to_string();

        if let Some(details) = details {
            match details.contains('`') {
                true => reason = details,
                false => match reason.is_empty() {
                    true => reason.push_str(&details.to_string()),
                    false => reason.push_str(&format!(" - {details}")),
                },
            }
        }

        reason
    }
}

#[derive(CommandOption, CreateOption)]
pub enum PunishmentPurgeAmount {
    #[option(name = "Don't Delete Any", value = 0)]
    None,
    #[option(name = "Previous Hour", value = 3_600)]
    Hour,
    #[option(name = "Previous 6 Hours", value = 21_600)]
    SixHours,
    #[option(name = "Previous 12 Hours", value = 43_200)]
    TwelveHours,
    #[option(name = "Previous 24 Hours", value = 86_400)]
    TwentyFourHours,
    #[option(name = "Previous 3 Days", value = 259_200)]
    ThreeDays,
    #[option(name = "Previous 7 Days", value = 604_800)]
    SevenDays,
}
