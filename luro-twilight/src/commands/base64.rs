use anyhow::anyhow;
use base64::{engine::general_purpose, Engine};
use luro_framework::{CommandInteraction, ComponentInteraction, CreateLuroCommand, Luro, LuroCommand};
use luro_model::{response::LuroResponse, types::CommandResponse};
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

impl CreateLuroCommand for Base64 {
    async fn interaction_command(self, ctx: CommandInteraction) -> anyhow::Result<luro_model::types::CommandResponse> {
        // Call the appropriate subcommand.
        match self {
            Self::Decode(command) => command.interaction_command(ctx).await,
            Self::Encode(command) => command.interaction_command(ctx).await,
        }
    }

    async fn interaction_component(
        self,
        ctx: ComponentInteraction,
        _original_interaction: twilight_model::application::interaction::Interaction,
    ) -> anyhow::Result<luro_model::types::CommandResponse> {
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
                return Ok(CommandResponse::default());
            }
        };

        match bait.unwrap_or_default() {
            true => {
                ctx.response_update(&response).await?;
                ctx.respond(|response| {
                    response
                        .content(format!("<@{}> got baited...", ctx.author.user_id))
                        .reply(&ctx.message.id)
                })
                .await
            }
            false => ctx.response_update(&response).await,
        }
    }
}

/// Simply send a response with a few checks.
async fn response(ctx: &ComponentInteraction, input: &str, decode_operation: bool) -> anyhow::Result<LuroResponse> {
    let mut response = match decode_operation {
        true => decode_response(ctx.accent_colour(), input).await?,
        false => encode_response(ctx.accent_colour(), &encode(input)).await?,
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
        Err(_) => Ok(format!("Don't be a cunt, I know that this is not base64 you bitch\n\n{input}")),
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
