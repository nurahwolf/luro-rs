use anyhow::anyhow;
use async_trait::async_trait;
use base64::{engine::general_purpose, Engine};
use luro_framework::{command::ExecuteLuroCommand, CommandInteraction, ComponentInteraction};
use luro_model::response::LuroResponse;
use std::str;
use tracing::{info, warn};
use twilight_interactions::command::{CommandModel, CreateCommand};

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

#[async_trait]
impl ExecuteLuroCommand for Base64 {
    async fn interaction_command(&self, ctx: CommandInteraction) -> anyhow::Result<()> {
        // Call the appropriate subcommand.
        match self {
            Self::Decode(command) => command.interaction_command(ctx).await,
            Self::Encode(command) => command.interaction_command(ctx).await,
        }
    }

    async fn interaction_component(&self, ctx: ComponentInteraction) -> anyhow::Result<()> {
        let author_id = ctx.author().id;
        // Always insure the input is decoded
        let (input, bait) = match self {
            Self::Decode(command) => (decode(&command.string)?, None),
            Self::Encode(command) => (command.string, command.bait),
        };

        let response = match ctx.data.custom_id.as_str() {
            "decode" => response(&ctx, &input, true).await?,
            "encode" => response(&ctx, &input, false).await?,
            _ => {
                warn!("No match");
                return Ok(());
            }
        };

        if bait.unwrap_or_default() {
            ctx.response_update(&response).await?;
            ctx.respond(|response| {
                response
                    .content(format!("<@{author_id}> got baited..."))
                    .reply(&ctx.message.id)
            })
            .await?;
        } else {
            ctx.response_update(&response).await?;
        }

        Ok(())
    }
}

/// Simply send a response with a few checks.
async fn response(ctx: &ComponentInteraction, input: &str, decode_operation: bool) -> anyhow::Result<LuroResponse> {
    let mut response = match decode_operation {
        true => decode_response(ctx.accent_colour().await, input).await?,
        false => encode_response(ctx.accent_colour().await, &encode(input)).await?,
    };
    response.update();
    Ok(response)
}

pub async fn decode_response(accent_colour: u32, input: &str) -> anyhow::Result<LuroResponse> {
    let mut response = LuroResponse::default();

    response.components(|c| c.action_row(|a| a.button(|button| button.custom_id("encode").label("Encode"))));

    match input.len() > 2000 {
        true => response.embed(|embed| embed.colour(accent_colour).description(format!("```\n{input}\n```"))),
        false => response.content(format!("```\n{input}\n```")),
    };
    Ok(response)
}

pub async fn encode_response(accent_colour: u32, input: &str) -> anyhow::Result<LuroResponse> {
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
