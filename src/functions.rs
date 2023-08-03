use anyhow::Error;
use twilight_http::client::InteractionClient;
use twilight_model::http::interaction::InteractionResponse;
use twilight_model::{application::interaction::Interaction, http::interaction::InteractionResponseType};
use twilight_util::builder::InteractionResponseDataBuilder;

/// A simple function to respond with `ChannelMessageWithSource`
pub async fn respond_to_interaction(
    interaction_client: &InteractionClient<'_>,
    interaction: &Interaction,
    content: String
) -> Result<(), Error> {
    let data = InteractionResponseDataBuilder::new().content(content).build();

    let response = InteractionResponse {
        kind: InteractionResponseType::ChannelMessageWithSource,
        data: Some(data)
    };

    interaction_client
        .create_response(interaction.id, &interaction.token, &response)
        .await?;

    Ok(())
}

/// Parse a string into a u32, used for hex codes colours
pub fn parse_string_to_u32(input: String) -> anyhow::Result<u32> {
    Ok(if input.starts_with("0x") {
        u32::from_str_radix(input.as_str().strip_prefix("0x").unwrap(), 16)?
    } else if input.chars().all(|char| char.is_ascii_hexdigit()) {
        u32::from_str_radix(input.as_str(), 16)?
    } else {
        input.parse::<u32>()?
    })
}
