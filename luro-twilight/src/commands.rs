use luro_framework::CreateLuroCommand;
use luro_framework::responses::Response;
use luro_framework::InteractionContext;
use tracing::info;
use twilight_interactions::command::CreateCommand;
use twilight_model::application::command::Command;

#[cfg(feature = "command-about")]
mod about;
#[cfg(feature = "command-base64")]
mod base64;
#[cfg(feature = "command-boop")]
mod boop;
#[cfg(feature = "command-character")]
mod character;
// #[cfg(feature = "command-count")]
// mod count;
#[cfg(feature = "command-dice")]
mod dice;
// #[cfg(feature = "command-heck")]
// mod heck;
// #[cfg(feature = "command-hello")]
// mod hello;
#[cfg(feature = "command-info")]
mod info;
// #[cfg(feature = "command-lewd")]
// mod lewd;
// #[cfg(feature = "command-luro")]
// mod luro;
#[cfg(feature = "command-marry")]
mod marry;
#[cfg(feature = "command-muzzle")]
mod muzzle;
// #[cfg(feature = "command-moderator")]
// mod moderator;
#[cfg(feature = "command-music")]
mod music;
#[cfg(feature = "command-owner")]
mod owner;
// #[cfg(feature = "command-ping")]
// mod ping;
// #[cfg(feature = "command-quote")]
// mod quote;
// #[cfg(feature = "command-roles")]
// mod roles;
#[cfg(feature = "command-say")]
mod say;
// #[cfg(feature = "command-story")]
// mod story;
#[cfg(feature = "command-uwu")]
mod uwu;
// #[cfg(feature = "command-wordcount")]
// mod wordcount;

pub fn default_commands() -> Vec<Command> {
    vec![
        #[cfg(feature = "command-about")]
        about::About::create_command().into(),
        #[cfg(feature = "command-base64")]
        base64::Base64::create_command().into(),
        #[cfg(feature = "command-boop")]
        boop::Boop::create_command().into(),
        #[cfg(feature = "command-character")]
        character::Character::create_command().into(),
        #[cfg(feature = "command-dice")]
        dice::Dice::create_command().into(),
        #[cfg(feature = "command-info")]
        info::Info::create_command().into(),
        #[cfg(feature = "command-marry")]
        marry::Marry::create_command().into(),
        #[cfg(feature = "command-muzzle")]
        muzzle::Muzzle::create_command().into(),
        #[cfg(feature = "command-music")]
        music::Music::create_command().into(),
        #[cfg(feature = "command-owner")]
        owner::Owner::create_command().into(),
        #[cfg(feature = "command-say")]
        say::Say::create_command().into(),
        #[cfg(feature = "command-uwu")]
        uwu::UwU::create_command().into(),
    ]
}

/// Handle incoming interaction
pub async fn handle_interaction(ctx: InteractionContext) -> anyhow::Result<()> {
    info!("{}: Handling interaction '{}'", ctx.command_type(), ctx.command_name());

    let response_handler = ctx.clone();
    let response = match ctx.command_name() {
        #[cfg(feature = "command-about")]
        "about" => match ctx {
            InteractionContext::Command(command) => about::About::run_interaction_command(command).await,
            InteractionContext::CommandAutocomplete(command) => about::About::run_interaction_autocomplete(command).await,
            InteractionContext::Component(command) => about::About::run_interaction_component(command).await,
            InteractionContext::Modal(command) => about::About::run_interaction_modal(command).await,
        },
        #[cfg(feature = "command-base64")]
        "base64" => match ctx {
            InteractionContext::Command(command) => base64::Base64::run_interaction_command(command).await,
            InteractionContext::CommandAutocomplete(command) => base64::Base64::run_interaction_autocomplete(command).await,
            InteractionContext::Component(command) => base64::Base64::run_interaction_component(command).await,
            InteractionContext::Modal(command) => base64::Base64::run_interaction_modal(command).await,
        },
        #[cfg(feature = "command-boop")]
        "boop" => match ctx {
            InteractionContext::Command(command) => boop::Boop::run_interaction_command(command).await,
            InteractionContext::CommandAutocomplete(command) => boop::Boop::run_interaction_autocomplete(command).await,
            InteractionContext::Component(command) => boop::Boop::run_interaction_component(command).await,
            InteractionContext::Modal(command) => boop::Boop::run_interaction_modal(command).await,
        },
        #[cfg(feature = "command-character")]
        "character" => match ctx {
            InteractionContext::Command(command) => character::Character::run_interaction_command(command).await,
            InteractionContext::CommandAutocomplete(command) => {
                character::Character::run_interaction_autocomplete(command).await
            }
            InteractionContext::Component(command) => character::Character::run_interaction_component(command).await,
            InteractionContext::Modal(command) => character::Character::run_interaction_modal(command).await,
        },
        #[cfg(feature = "command-dice")]
        "dice" => match ctx {
            InteractionContext::Command(command) => dice::Dice::run_interaction_command(command).await,
            InteractionContext::CommandAutocomplete(command) => dice::Dice::run_interaction_autocomplete(command).await,
            InteractionContext::Component(command) => dice::Dice::run_interaction_component(command).await,
            InteractionContext::Modal(command) => dice::Dice::run_interaction_modal(command).await,
        },
        #[cfg(feature = "command-say")]
        "say" => match ctx {
            InteractionContext::Command(command) => say::Say::run_interaction_command(command).await,
            InteractionContext::CommandAutocomplete(command) => say::Say::run_interaction_autocomplete(command).await,
            InteractionContext::Component(command) => say::Say::run_interaction_component(command).await,
            InteractionContext::Modal(command) => say::Say::run_interaction_modal(command).await,
        },
        #[cfg(feature = "command-info")]
        "info" => match ctx {
            InteractionContext::Command(command) => info::Info::run_interaction_command(command).await,
            InteractionContext::CommandAutocomplete(command) => info::Info::run_interaction_autocomplete(command).await,
            InteractionContext::Component(command) => info::Info::run_interaction_component(command).await,
            InteractionContext::Modal(command) => info::Info::run_interaction_modal(command).await,
        },
        #[cfg(feature = "command-owner")]
        "owner" => match ctx {
            InteractionContext::Command(command) => owner::Owner::run_interaction_command(command).await,
            InteractionContext::CommandAutocomplete(command) => owner::Owner::run_interaction_autocomplete(command).await,
            InteractionContext::Component(command) => owner::Owner::run_interaction_component(command).await,
            InteractionContext::Modal(command) => owner::Owner::run_interaction_modal(command).await,
        },
        #[cfg(feature = "command-marry")]
        "marry" | "marry-accept" | "marry-deny" => match ctx {
            InteractionContext::Command(command) => marry::Marry::run_interaction_command(command).await,
            InteractionContext::CommandAutocomplete(command) => marry::Marry::run_interaction_autocomplete(command).await,
            InteractionContext::Component(command) => marry::Marry::run_interaction_component(command).await,
            InteractionContext::Modal(command) => marry::Marry::run_interaction_modal(command).await,
        },
        #[cfg(feature = "command-muzzle")]
        "muzzle" => match ctx {
            InteractionContext::Command(command) => muzzle::Muzzle::run_interaction_command(command).await,
            InteractionContext::CommandAutocomplete(command) => muzzle::Muzzle::run_interaction_autocomplete(command).await,
            InteractionContext::Component(command) => muzzle::Muzzle::run_interaction_component(command).await,
            InteractionContext::Modal(command) => muzzle::Muzzle::run_interaction_modal(command).await,
        },
        #[cfg(feature = "command-music")]
        "music" => match ctx {
            InteractionContext::Command(command) => music::Music::run_interaction_command(command).await,
            InteractionContext::CommandAutocomplete(command) => music::Music::run_interaction_autocomplete(command).await,
            InteractionContext::Component(command) => music::Music::run_interaction_component(command).await,
            InteractionContext::Modal(command) => music::Music::run_interaction_modal(command).await,
        },
        #[cfg(feature = "command-uwu")]
        "uwu" => match ctx {
            InteractionContext::Command(command) => uwu::UwU::run_interaction_command(command).await,
            InteractionContext::CommandAutocomplete(command) => uwu::UwU::run_interaction_autocomplete(command).await,
            InteractionContext::Component(command) => uwu::UwU::run_interaction_component(command).await,
            InteractionContext::Modal(command) => uwu::UwU::run_interaction_modal(command).await,
        },
        name => ctx.simple_response(Response::UnknownCommand(name)).await,
    };

    if let Err(why) = response {
        response_handler.simple_response(Response::InternalError(why)).await?;
    }

    Ok(())
}
