use anyhow::Result;
use tracing::warn;
use twilight_interactions::command::{CommandInputData, CommandModel, CreateCommand};
use twilight_model::{
    application::interaction::Interaction,
    http::interaction::{InteractionResponse, InteractionResponseData, InteractionResponseType},
};

use crate::{Luro, State};

#[derive(CommandModel, CreateCommand)]
#[command(
    name = "usage",
    desc = "View the count of interactions since the last restart"
)]
pub struct CommandUsage {}

pub async fn command_usage(state: State, interaction: &Interaction) -> Result<()> {
    let command_data = match Luro::get_interaction_data(interaction).await {
        Ok(ok) => ok,
        Err(why) => {
            warn!("Failed to get interaction data - {why}");
            return Ok(());
        }
    };

    let _data = match CommandUsage::from_interaction(CommandInputData::from(*command_data)) {
        Ok(ok) => ok,
        Err(err) => {
            warn!("Failed to parse interaction data - {err}");
            CommandUsage {}
        }
    };

    let value = match state.data.interaction_count.read() {
        Ok(ok) => *ok,
        Err(_) => todo!(),
    };

    let response = InteractionResponse {
        kind: InteractionResponseType::ChannelMessageWithSource,
        data: Some(InteractionResponseData {
            content: Some(format!(
                "**{}** Interactions since I last restarted.",
                value
            )),
            ..Default::default()
        }),
    };

    match state
        .twilight_client
        .interaction(state.data.application_info.id)
        .create_response(interaction.id, &interaction.token, &response)
        .await
    {
        Ok(ok) => ok,
        Err(_) => todo!(),
    };

    Ok(())
}
