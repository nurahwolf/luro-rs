use luro_framework::command::CreateLuroCommand;
use luro_framework::InteractionContext;
use luro_framework::responses::Response;
use tracing::info;
use twilight_interactions::command::CreateCommand;
use twilight_model::application::command::Command;

#[cfg(feature = "command-about")]
mod about;
// #[cfg(feature = "command-base64")]
// mod base64;
// #[cfg(feature = "command-boop")]
// mod boop;
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
// #[cfg(feature = "command-moderator")]
// mod moderator;
// #[cfg(feature = "command-music")]
// mod music;
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
// #[cfg(feature = "command-uwu")]
// mod uwu;
// #[cfg(feature = "command-wordcount")]
// mod wordcount;

pub fn default_commands() -> Vec<Command> {
    vec![
        #[cfg(feature = "command-about")]
        about::About::create_command().into(),
        #[cfg(feature = "command-character")]
        character::Character::create_command().into(),
        #[cfg(feature = "command-dice")]
        dice::Dice::create_command().into(),
        #[cfg(feature = "command-say")]
        say::Say::create_command().into(),
        #[cfg(feature = "command-info")]
        info::Info::create_command().into(),
        #[cfg(feature = "command-owner")]
        owner::Owner::create_command().into(),
        #[cfg(feature = "command-marry")]
        marry::Marry::create_command().into(),
    ]
}

/// Handle incoming interaction
pub async fn handle_interaction(ctx: InteractionContext) -> anyhow::Result<()> {
    info!("{}: Handling interaction '{}'", ctx.command_type(), ctx.command_name());

    let response_handler = ctx.clone();
    let response = match ctx.command_name() {
        #[cfg(feature = "command-about")]
        "about" => match ctx {
            InteractionContext::CommandInteraction(command) => about::About::run_interaction_command(command).await,
            InteractionContext::CommandAutocompleteInteraction(command) => about::About::run_interaction_autocomplete(command).await,
            InteractionContext::ComponentInteraction(command) => about::About::run_interaction_component(command).await,
            InteractionContext::ModalInteraction(command) => about::About::run_interaction_modal(command).await,
        },
        #[cfg(feature = "command-character")]
        "character" => match ctx {
            InteractionContext::CommandInteraction(command) => character::Character::run_interaction_command(command).await,
            InteractionContext::CommandAutocompleteInteraction(command) => {
                character::Character::run_interaction_autocomplete(command).await
            }
            InteractionContext::ComponentInteraction(command) => character::Character::run_interaction_component(command).await,
            InteractionContext::ModalInteraction(command) => character::Character::run_interaction_modal(command).await,
        },
        #[cfg(feature = "command-dice")]
        "dice" => match ctx {
            InteractionContext::CommandInteraction(command) => dice::Dice::run_interaction_command(command).await,
            InteractionContext::CommandAutocompleteInteraction(command) => dice::Dice::run_interaction_autocomplete(command).await,
            InteractionContext::ComponentInteraction(command) => dice::Dice::run_interaction_component(command).await,
            InteractionContext::ModalInteraction(command) => dice::Dice::run_interaction_modal(command).await,
        },
        #[cfg(feature = "command-say")]
        "say" => match ctx {
            InteractionContext::CommandInteraction(command) => say::Say::run_interaction_command(command).await,
            InteractionContext::CommandAutocompleteInteraction(command) => say::Say::run_interaction_autocomplete(command).await,
            InteractionContext::ComponentInteraction(command) => say::Say::run_interaction_component(command).await,
            InteractionContext::ModalInteraction(command) => say::Say::run_interaction_modal(command).await,
        },
        #[cfg(feature = "command-info")]
        "info" => match ctx {
            InteractionContext::CommandInteraction(command) => info::Info::run_interaction_command(command).await,
            InteractionContext::CommandAutocompleteInteraction(command) => info::Info::run_interaction_autocomplete(command).await,
            InteractionContext::ComponentInteraction(command) => info::Info::run_interaction_component(command).await,
            InteractionContext::ModalInteraction(command) => info::Info::run_interaction_modal(command).await,
        },
        #[cfg(feature = "command-owner")]
        "owner" => match ctx {
            InteractionContext::CommandInteraction(command) => owner::Owner::run_interaction_command(command).await,
            InteractionContext::CommandAutocompleteInteraction(command) => owner::Owner::run_interaction_autocomplete(command).await,
            InteractionContext::ComponentInteraction(command) => owner::Owner::run_interaction_component(command).await,
            InteractionContext::ModalInteraction(command) => owner::Owner::run_interaction_modal(command).await,
        },
        #[cfg(feature = "command-marry")]
        "marry" | "marry-accept" | "marry-deny" => match ctx {
            InteractionContext::CommandInteraction(command) => marry::Marry::run_interaction_command(command).await,
            InteractionContext::CommandAutocompleteInteraction(command) => marry::Marry::run_interaction_autocomplete(command).await,
            InteractionContext::ComponentInteraction(command) => marry::Marry::run_interaction_component(command).await,
            InteractionContext::ModalInteraction(command) => marry::Marry::run_interaction_modal(command).await,
        },
        name => ctx.simple_response(Response::UnknownCommand(name)).await,
    };

    if let Err(why) = response {
        response_handler.simple_response(Response::InternalError(why)).await?;
    }

    Ok(())
}
