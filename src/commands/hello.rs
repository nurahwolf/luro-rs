use anyhow::Error;
use tracing::warn;
use twilight_interactions::command::{CommandInputData, CommandModel, CreateCommand, ResolvedUser};
use twilight_model::application::{
    command::{Command, CommandType},
    interaction::Interaction,
};
use twilight_util::builder::{
    command::CommandBuilder, embed::EmbedBuilder, InteractionResponseDataBuilder,
};

use crate::{functions::get_interaction_data, luro::Luro};

use super::create_response;

pub fn commands() -> Vec<Command> {
    vec![
        CommandBuilder::new("hello", "Hello World!", CommandType::ChatInput).build(),
        HelloCommand::create_command().into(),
    ]
}

#[derive(CommandModel, CreateCommand)]
#[command(name = "hellov2", desc = "Say hello")]
pub struct HelloCommand {
    /// The message to send.
    message: String,
    /// The user to send the message to.
    user: Option<ResolvedUser>,
}

pub async fn hellov2(luro: &Luro, interaction: &Interaction) -> Result<(), Error> {
    let data = match HelloCommand::from_interaction(CommandInputData::from(
        *get_interaction_data(interaction).await?,
    )) {
        Ok(ok) => ok,
        Err(why) => {
            warn!("Failed to parse interaction data - {why}");
            HelloCommand {
                message: "You can this without specifying anything...".to_string(),
                user: None,
            }
        }
    };

    let embed = match data.user {
        Some(user) => EmbedBuilder::default()
            .description(format!("{}\n - {}", data.message, user.resolved.name)),
        None => EmbedBuilder::default().description(format!(
            "{}\n - {}",
            data.message,
            interaction.author().unwrap().name
        )),
    };

    let data = InteractionResponseDataBuilder::new().embeds([embed.build()]);

    create_response(luro, interaction, data.build()).await?;

    Ok(())
}

pub async fn hello(luro: &Luro, interaction: &Interaction) -> Result<(), Error> {
    let user = luro.twilight_cache.current_user().unwrap();
    let user_name = user.name;

    let embed = EmbedBuilder::default().description(format!("Hello World! I am {user_name}"));
    let data = InteractionResponseDataBuilder::new().embeds([embed.build()]);

    create_response(luro, interaction, data.build()).await?;

    Ok(())
}
