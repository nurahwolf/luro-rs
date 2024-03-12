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
#[cfg(feature = "command-uwu")]
mod uwu;

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
        #[cfg(feature = "command-owner")]
        owner::Owner::setup_command(),
    ]
}

/// Handle a command spawned from an interaction context
pub async fn interaction_handler(mut framework: InteractionContext) {
    tracing::info!("Handling interaction `{}`", framework.command_name());

    let response = match framework.command_name() {
        "about" => about::About::interaction_handler(&mut framework).await,
        "ban" => ban::Ban::interaction_handler(&mut framework).await,
        "base64" | "base64-encode" | "base64-decode" => {
            base64::Base64::interaction_handler(&mut framework).await
        }
        "boop" => boop::Boop::interaction_handler(&mut framework).await,
        "dice" => dice::Dice::interaction_handler(&mut framework).await,
        "uwu" => uwu::UwU::interaction_handler(&mut framework).await,
        "owner" => owner::Owner::interaction_handler(&mut framework).await,

        name => {
            framework
                .standard_response(StandardResponse::UnknownCommand(name))
                .await
        }
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
