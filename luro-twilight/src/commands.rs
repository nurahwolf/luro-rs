use anyhow::anyhow;
use luro_framework::command::CreateLuroCommand;
use luro_framework::{CommandInteraction, InteractionContext};
use tracing::info;
use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_model::application::command::Command;
use twilight_model::application::interaction::application_command::CommandData;

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
// #[cfg(feature = "command-marry")]
// mod marry;
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
    ]
}

/// Handle incoming interaction
pub async fn handle_interaction(ctx: InteractionContext) -> anyhow::Result<()> {
    info!(
        "Handling interaction '{}' of type '{}'",
        ctx.command_name(),
        ctx.command_type()
    );

    match ctx.command_name() {
        #[cfg(feature = "command-about")]
        "about" => match ctx {
            InteractionContext::CommandInteraction(command) => about::About::run_interaction_command(command).await,
            InteractionContext::CommandAutocompleteInteraction(command) => {
                about::About::run_interaction_autocomplete(command).await
            }
            InteractionContext::ComponentInteraction(command) => about::About::run_interaction_component(command).await,
            InteractionContext::ModalInteraction(command) => about::About::run_interaction_modal(command).await,
        },
        name => Err(anyhow!("No handler registered for command '{}'", name)),
    }
}

#[derive(CommandModel)]
#[command(autocomplete = true)]
enum Autocomplete {
    #[cfg(feature = "command-character")]
    #[command(name = "send")]
    Send(character::send::CharacterSendAutocomplete),
    #[cfg(feature = "command-character")]
    #[command(name = "proxy")]
    Proxy(character::send::CharacterSendAutocomplete),
    #[cfg(feature = "command-character")]
    #[command(name = "icon")]
    Icon(character::send::CharacterSendAutocomplete),
    #[cfg(feature = "command-character")]
    #[command(name = "create")]
    Create(character::send::CharacterSendAutocomplete),
}

impl Autocomplete {
    async fn run(self, ctx: CommandInteraction) -> anyhow::Result<()> {
        match self {
            #[cfg(feature = "command-character")]
            Autocomplete::Create(cmd) | Autocomplete::Icon(cmd) | Autocomplete::Send(cmd) | Autocomplete::Proxy(cmd) => {
                cmd.interaction_command(ctx).await
            }
        }
    }

    fn new(data: CommandData) -> anyhow::Result<Self> {
        match Self::from_interaction(data.into()) {
            Ok(ok) => Ok(ok),
            Err(why) => Err(anyhow!(
                "Got interaction data, but failed to parse it to the command type specified: {why}"
            )),
        }
    }
}
