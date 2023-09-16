use anyhow::anyhow;
use async_trait::async_trait;
use base64::{engine::general_purpose, Engine};
use luro_framework::{
    command::{LuroCommandBuilder, LuroCommandTrait},
    Framework, InteractionCommand, InteractionComponent, LuroInteraction,
};
use luro_model::{database_driver::LuroDatabaseDriver, response::LuroResponse};
use std::str;
use tracing::{info, warn};
use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_model::application::interaction::InteractionData;

mod decode;
mod encode;

#[derive(CommandModel, CreateCommand)]
#[command(name = "base64", desc = "Convert to and from base64")]
pub enum Base64 {
    #[command(name = "decode")]
    Decode(decode::Decode),
    #[command(name = "encode")]
    Encode(encode::Encode),
}
impl<D: LuroDatabaseDriver + 'static> LuroCommandBuilder<D> for Base64 {}

#[async_trait]
impl LuroCommandTrait for Base64 {
    async fn handle_interaction<D: LuroDatabaseDriver>(
        ctx: Framework<D>,
        interaction: InteractionCommand,
    ) -> anyhow::Result<()> {
        let data = Self::new(interaction.data.clone())?;
        // Call the appropriate subcommand.
        match data {
            Self::Decode(_) => decode::Decode::handle_interaction(ctx, interaction).await,
            Self::Encode(_) => encode::Encode::handle_interaction(ctx, interaction).await,
        }
    }

    async fn handle_component<D: LuroDatabaseDriver>(
        ctx: Framework<D>,
        interaction: InteractionComponent,
    ) -> anyhow::Result<()> {
        let mut message = interaction.message.clone();
        let mut original_interaction = interaction.original.clone();
        let mut new_id = true;

        while new_id {
            original_interaction = ctx.database.get_interaction(&message.id.to_string()).await?;

            new_id = match original_interaction.message {
                Some(ref new_message) => {
                    message = new_message.clone();
                    true
                }
                None => false,
            }
        }

        let command = match original_interaction.data {
            Some(InteractionData::ApplicationCommand(ref data)) => data.clone(),
            _ => {
                return Err(anyhow!(
                    "unable to parse modal data due to not receiving ApplicationCommand data\n{:#?}",
                    interaction.data
                ))
            }
        };

        let data = Self::new(command)?;
        let author_id = interaction.author_id();
        // Always insure the input is decoded
        let (input, bait) = match data {
            Self::Decode(command) => (decode(&command.string)?, None),
            Self::Encode(command) => (command.string, command.bait),
        };

        let response = match interaction.data.custom_id.as_str() {
            "decode" => response(&ctx, &interaction, &input, true).await?,
            "encode" => response(&ctx, &interaction, &input, false).await?,
            _ => {
                warn!("No match");
                return Ok(());
            }
        };

        if bait.unwrap_or_default() {
            interaction.response_update(&ctx, &response).await?;
            interaction
                .respond(&ctx, |response| {
                    response
                        .content(format!("<@{author_id}> got baited..."))
                        .reply(&interaction.message.id)
                })
                .await?;
        } else {
            interaction.response_update(&ctx, &response).await?;
        }

        Ok(())
    }
}

/// Simply send a response with a few checks.
async fn response<D: LuroDatabaseDriver>(
    ctx: &Framework<D>,
    interaction: &InteractionComponent,
    input: &str,
    decode_operation: bool,
) -> anyhow::Result<LuroResponse> {
    let mut response = match decode_operation {
        true => decode_response(ctx, interaction, input).await?,
        false => encode_response(ctx, interaction, &encode(input)).await?,
    };
    response.update();
    Ok(response)
}

pub async fn decode_response<D: LuroDatabaseDriver, I: LuroInteraction>(
    ctx: &Framework<D>,
    interaction: &I,
    input: &str,
) -> anyhow::Result<LuroResponse> {
    let accent_colour = interaction.accent_colour(ctx).await;
    let mut response = LuroResponse::default();

    response.components(|c| c.action_row(|a| a.button(|button| button.custom_id("encode").label("Encode"))));

    match input.len() > 2000 {
        true => response.embed(|embed| embed.colour(accent_colour).description(format!("```\n{input}\n```"))),
        false => response.content(format!("```\n{input}\n```")),
    };
    Ok(response)
}

pub async fn encode_response<D: LuroDatabaseDriver, I: LuroInteraction>(
    ctx: &Framework<D>,
    interaction: &I,
    input: &str,
) -> anyhow::Result<LuroResponse> {
    let accent_colour = interaction.accent_colour(ctx).await;
    let mut response = LuroResponse::default();

    response.components(|c| c.action_row(|a| a.button(|button| button.custom_id("decode").label("Decode"))));

    match input.len() > 2000 {
        true => response.embed(|embed| embed.colour(accent_colour).description(format!("```\n{input}\n```"))),
        false => response.content(format!("```\n{input}\n```")),
    };
    Ok(response)
}

/// Decode the passed text
pub fn decode(input: &str) -> anyhow::Result<String> {
    info!("Decoding `{input}`");
    let result = match general_purpose::STANDARD.decode(input) {
        Ok(decoded) => match String::from_utf8(decoded) {
            Ok(decoded_string) => Ok(decoded_string),
            Err(why) => Err(anyhow!("Failed to convert bytes into string - {why}")),
        },
        Err(_) => Ok(format!(
            "Don't be a cunt, I know that this is not base64 you bitch\n\n{input}"
        )),
    };
    info!("Result - `{:?}`", result);
    result
}

/// Encode the passed text
pub fn encode(input: &str) -> String {
    info!("Encoding - `{input}`");
    let result = general_purpose::STANDARD.encode(input);
    info!("Result - `{result}`");
    result
}
