use luro_framework::responses::Response;
use luro_framework::CreateLuroCommand;
use luro_framework::InteractionContext;
use tracing::info;
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
#[cfg(feature = "command-hello")]
mod hello;
#[cfg(feature = "command-info")]
mod info;
// #[cfg(feature = "command-lewd")]
// mod lewd;
// #[cfg(feature = "command-luro")]
// mod luro;
#[cfg(feature = "command-images")]
mod images;
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
#[cfg(feature = "command-wordcount")]
mod wordcount;

pub fn default_commands() -> Vec<Command> {
    vec![
        #[cfg(feature = "command-about")]
        about::About::setup_command(),
        #[cfg(feature = "command-base64")]
        base64::Base64::setup_command(),
        #[cfg(feature = "command-boop")]
        boop::Boop::setup_command(),
        #[cfg(feature = "command-character")]
        character::Character::setup_command(),
        #[cfg(feature = "command-dice")]
        dice::Dice::setup_command(),
        #[cfg(feature = "command-hello")]
        hello::Hello::setup_command(),
        #[cfg(feature = "command-info")]
        info::Info::setup_command(),
        #[cfg(feature = "command-marry")]
        marry::Marry::setup_command(),
        #[cfg(feature = "command-muzzle")]
        muzzle::Muzzle::setup_command(),
        #[cfg(feature = "command-music")]
        music::Music::setup_command(),
        #[cfg(feature = "command-owner")]
        owner::Owner::setup_command(),
        #[cfg(feature = "command-say")]
        say::Say::setup_command(),
        #[cfg(feature = "command-uwu")]
        uwu::UwU::setup_command(),
        #[cfg(feature = "command-images")]
        images::Images::setup_command(),
        #[cfg(feature = "command-wordcount")]
        wordcount::Wordcount::setup_command(),
    ]
}

/// Handle incoming interaction
pub async fn handle_interaction(ctx: InteractionContext) -> anyhow::Result<()> {
    info!("{}: Handling interaction '{}'", ctx.command_type(), ctx.command_name());

    let response_handler = ctx.clone();
    let response = match ctx.command_name() {
        "about" => about::About::handle_interaction(ctx).await,
        "base64" => base64::Base64::handle_interaction(ctx).await,
        "boop" => boop::Boop::handle_interaction(ctx).await,
        "character" | "character-fetish" | "character-image" | "character-update" | "character-image-nsfw" => {
            character::Character::handle_interaction(ctx).await
        }
        "dice" => dice::Dice::handle_interaction(ctx).await,
        "hello" => hello::Hello::handle_interaction(ctx).await,
        "images" => images::Images::handle_interaction(ctx).await,
        "info" => info::Info::handle_interaction(ctx).await,
        "marry" | "marry-accept" | "marry-deny" => marry::Marry::handle_interaction(ctx).await,
        #[cfg(feature = "command-music")]
        "music" => music::Music::handle_interaction(ctx).await,
        "muzzle" => muzzle::Muzzle::handle_interaction(ctx).await,
        "owner" => owner::Owner::handle_interaction(ctx).await,
        "say" => say::Say::handle_interaction(ctx).await,
        #[cfg(feature = "command-uwu")]
        "uwu" => uwu::UwU::handle_interaction(ctx).await,
        #[cfg(feature = "command-wordcount")]
        "wordcount" => wordcount::Wordcount::handle_interaction(ctx).await,
        name => ctx.simple_response(Response::UnknownCommand(name)).await,
    };

    if let Err(why) = response {
        response_handler.simple_response(Response::InternalError(why)).await?;
    }

    Ok(())
}
