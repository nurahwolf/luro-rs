use anyhow::Result;
use tracing::warn;
use twilight_interactions::command::{CommandInputData, CommandModel, CreateCommand, ResolvedUser};
use twilight_model::{
    application::interaction::Interaction,
    http::interaction::{InteractionResponse, InteractionResponseData, InteractionResponseType},
};

use crate::Luro;

#[derive(CommandModel, CreateCommand)]
#[command(name = "hello", desc = "Say hello to someone")]
pub struct HelloCommand {
    /// Message to send
    message: Option<String>,
    /// User to send the message to
    user: Option<ResolvedUser>,
}

pub async fn hello_world<'a>(luro: &Luro, interaction: &Interaction) -> Result<()> {
    let command_data = match Luro::get_interaction_data(interaction).await {
        Ok(ok) => ok,
        Err(why) => {
            warn!("Failed to get interaction data - {why}");
            return Ok(());
        }
    };

    let data = match HelloCommand::from_interaction(CommandInputData::from(*command_data)) {
        Ok(ok) => ok,
        Err(err) => {
            warn!("Failed to parse interaction data - {err}");
            HelloCommand {
                message: None,
                user: None,
            }
        }
    };

    let mut content = if let Some(message_defined) = data.message {
        message_defined
    } else {
        "Hello World!".to_string()
    };

    if let Some(user_defined) = data.user {
        content.push_str(format!(" - <@{}>", user_defined.resolved.id).as_str());
    };

    let response = InteractionResponse {
        kind: InteractionResponseType::ChannelMessageWithSource,
        data: Some(InteractionResponseData {
            content: Some(content),
            ..Default::default()
        }),
    };

    match luro
        .http
        .interaction(luro.application_id)
        .create_response(interaction.id, &interaction.token, &response)
        .await
    {
        Ok(ok) => ok,
        Err(_) => todo!(),
    };

    Ok(())
}
