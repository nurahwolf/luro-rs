use anyhow::{anyhow, Context};

use luro_builder::response::LuroResponse;

use std::str;

use base64::{engine::general_purpose, Engine};

use tracing::{info, warn};
use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_model::application::interaction::message_component::MessageComponentInteractionData;

use crate::interaction::LuroSlash;

use crate::luro_command::LuroCommand;
#[derive(CommandModel, CreateCommand)]
#[command(name = "base64", desc = "Convert to and from base64")]
pub enum Base64Commands {
    #[command(name = "decode")]
    Decode(Base64Decode),
    #[command(name = "encode")]
    Encode(Base64Encode)
}

impl LuroCommand for Base64Commands {
    async fn run_command(self, ctx: LuroSlash) -> anyhow::Result<()> {
        // Call the appropriate subcommand.
        match self {
            Self::Decode(command) => command.run_command(ctx).await,
            Self::Encode(command) => command.run_command(ctx).await
        }
    }

    async fn handle_component(self, data: Box<MessageComponentInteractionData>, ctx: LuroSlash) -> anyhow::Result<()> {
        let author_id = ctx.interaction.author_id().context("Expected to get interaction author ID")?;
        // Always insure the input is decoded
        let (input, bait) = match self {
            Self::Decode(command) => (decode(&command.string)?, None),
            Self::Encode(command) => (command.string, command.bait)
        };

        let response = match data.custom_id.as_str() {
            "decode" => response(&ctx, &input, true).await?,
            "encode" => response(&ctx, &input, false).await?,
            _ => {
                warn!("No match");
                return Ok(());
            }
        };

        if let Some(bait) = bait && bait {
            ctx.create_response(&response).await?;
            ctx.send_message(|response|{
                response.content(format!("<@{author_id}> got baited..."));
                if let Some(message) = &ctx.interaction.message {
                    response.reply(&message.id);
                }
                response
            }).await?;
        } else {
            ctx.create_response(&response).await?;
        }

        Ok(())
    }
}

#[derive(CommandModel, CreateCommand, Default, Debug, PartialEq, Eq)]
#[command(name = "decode", desc = "Convert a string from base64")]
pub struct Base64Decode {
    /// Decode this string from base64
    #[command(max_length = 2039)]
    string: String
}

impl LuroCommand for Base64Decode {
    async fn run_command(self, ctx: LuroSlash) -> anyhow::Result<()> {
        ctx.create_response(&decode_response(&ctx, &decode(&self.string)?).await?)
            .await
    }
}

#[derive(CommandModel, CreateCommand, Default, Debug, PartialEq, Eq)]
#[command(name = "encode", desc = "Convert a string to base64")]
pub struct Base64Encode {
    /// Encode this string to base64
    string: String,
    /// Set to true if you want to call out someone for clicking decoding this
    bait: Option<bool>
}

impl LuroCommand for Base64Encode {
    async fn run_command(self, ctx: LuroSlash) -> anyhow::Result<()> {
        ctx.create_response(&encode_response(&ctx, &encode(&self.string)).await?)
            .await
    }
}

/// Encode the passed text
fn encode(input: &str) -> String {
    info!("Encoding - `{input}`");
    let result = general_purpose::STANDARD.encode(input);
    info!("Result - `{result}`");
    result
}

/// Decode the passed text
fn decode(input: &str) -> anyhow::Result<String> {
    info!("Decoding `{input}`");
    let result = match general_purpose::STANDARD.decode(input) {
        Ok(decoded) => match String::from_utf8(decoded) {
            Ok(decoded_string) => Ok(decoded_string),
            Err(why) => Err(anyhow!("Failed to convert bytes into string - {why}"))
        },
        Err(_) => Ok(format!(
            "Don't be a cunt, I know that this is not base64 you bitch\n\n{input}"
        ))
    };
    info!("Result - `{:?}`", result);
    result
}

/// Simply send a response with a few checks.
async fn response(ctx: &LuroSlash, input: &str, decode_operation: bool) -> anyhow::Result<LuroResponse> {
    let mut response = match decode_operation {
        true => decode_response(ctx, input).await?,
        false => encode_response(ctx, &encode(input)).await?
    };
    response.update();
    Ok(response)
}

async fn decode_response(ctx: &LuroSlash, input: &str) -> anyhow::Result<LuroResponse> {
    let accent_colour = ctx.accent_colour().await;
    let mut response = LuroResponse::default();

    response.components(|c| c.action_row(|a| a.button(|button| button.custom_id("encode").label("Encode"))));

    match input.len() > 2000 {
        true => response.embed(|embed| embed.colour(accent_colour).description(format!("```\n{input}\n```"))),
        false => response.content(format!("```\n{input}\n```"))
    };
    Ok(response)
}

async fn encode_response(ctx: &LuroSlash, input: &str) -> anyhow::Result<LuroResponse> {
    let accent_colour = ctx.accent_colour().await;
    let mut response = LuroResponse::default();

    response.components(|c| c.action_row(|a| a.button(|button| button.custom_id("decode").label("Decode"))));

    match input.len() > 2000 {
        true => response.embed(|embed| embed.colour(accent_colour).description(format!("```\n{input}\n```"))),
        false => response.content(format!("```\n{input}\n```"))
    };
    Ok(response)
}
