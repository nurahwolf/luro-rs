use base64::Engine;

use crate::models::interaction::{InteractionContext, InteractionResult};

mod decode;
mod encode;

#[derive(
    twilight_interactions::command::CommandModel, twilight_interactions::command::CreateCommand,
)]
#[command(name = "base64", desc = "Convert to and from base64")]
pub enum Base64 {
    #[command(name = "decode")]
    Decode(decode::Decode),
    #[command(name = "encode")]
    Encode(encode::Encode),
}

impl crate::models::CreateCommand for Base64 {
    async fn handle_command(self, framework: &mut InteractionContext) -> InteractionResult<()> {
        // Call the appropriate subcommand
        match self {
            Self::Decode(command) => command.handle_command(framework).await,
            Self::Encode(command) => command.handle_command(framework).await,
        }
    }

    async fn handle_component(framework: &mut InteractionContext) -> InteractionResult<()> {
        // Always insure the input is decoded
        let (input, bait) = match Self::command_from_component(framework).await? {
            Self::Decode(command) => (decode(&command.string)?, None),
            Self::Encode(command) => (command.string, command.bait),
        };

        let response = match framework.command_name() {
            "base64-decode" => response(&framework, &input, true).await?,
            "base64-encode" => response(&framework, &input, false).await?,
            name => {
                tracing::warn!("No match for {name}");
                return Ok(());
            }
        };

        match bait.unwrap_or_default() {
            true => {
                let author_id = framework.author_id()?;
                framework.response_send(&response).await?;
                framework
                    .respond(|response| {
                        response
                            .content(format!("<@{author_id}> got baited..."))
                            .reply(&framework.compontent_message().unwrap().id)
                    })
                    .await
            }
            false => framework.response_send(&response).await,
        }
    }
}

/// Simply send a response with a few checks.
async fn response(
    framework: &InteractionContext,
    input: &str,
    decode_operation: bool,
) -> anyhow::Result<crate::builders::InteractionResponseBuilder> {
    let mut response = match decode_operation {
        true => decode_response(framework.accent_colour().await, input)?,
        false => encode_response(framework.accent_colour().await, &encode(input))?,
    };
    response.update();
    Ok(response)
}

pub fn decode_response(
    accent_colour: u32,
    input: &str,
) -> anyhow::Result<crate::builders::InteractionResponseBuilder> {
    let mut response = crate::builders::InteractionResponseBuilder::default();

    response.components(|c| {
        c.action_row(|a| a.button(|button| button.custom_id("base64-encode").label("Encode")))
    });

    match input.len() > 2000 {
        true => response.embed(|embed| {
            embed
                .colour(accent_colour)
                .description(format!("```\n{input}\n```"))
        }),
        false => response.content(format!("```\n{input}\n```")),
    };
    Ok(response)
}

pub fn encode_response(
    accent_colour: u32,
    input: &str,
) -> anyhow::Result<crate::builders::InteractionResponseBuilder> {
    let mut response = crate::builders::InteractionResponseBuilder::default();

    response.components(|c| {
        c.action_row(|a| a.button(|button| button.custom_id("base64-decode").label("Decode")))
    });

    match input.len() > 2000 {
        true => response.embed(|embed| {
            embed
                .colour(accent_colour)
                .description(format!("```\n{input}\n```"))
        }),
        false => response.content(format!("```\n{input}\n```")),
    };
    Ok(response)
}

/// Decode the passed text
pub fn decode(input: &str) -> anyhow::Result<String> {
    tracing::debug!("Decoding `{input}`");
    let result = match base64::engine::general_purpose::STANDARD.decode(input) {
        Ok(decoded) => match String::from_utf8(decoded) {
            Ok(decoded_string) => Ok(decoded_string),
            Err(why) => Err(anyhow::anyhow!(
                "Failed to convert bytes into string - {why}"
            )),
        },
        Err(_) => Ok(format!(
            "Don't be a cunt, I know that this is not base64 you bitch\n\n{input}"
        )),
    };
    tracing::debug!("Result - `{:?}`", result);
    result
}

/// Encode the passed text
pub fn encode(input: &str) -> String {
    tracing::debug!("Encoding - `{input}`");
    let result = base64::engine::general_purpose::STANDARD.encode(input);
    tracing::debug!("Result - `{result}`");
    result
}
