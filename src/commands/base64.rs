use anyhow::{anyhow, Context};
use async_trait::async_trait;
use std::str;

use base64::{engine::general_purpose, Engine};
use regex::Regex;
use tracing::{debug, error};
use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_model::{
    application::interaction::message_component::MessageComponentInteractionData,
    channel::message::{
        component::{ActionRow, Button, ButtonStyle},
        Component, Embed
    }
};

use crate::{models::LuroResponse, LuroContext, REGEX_CODE_BLOCK};

use crate::traits::luro_command::LuroCommand;
#[derive(CommandModel, CreateCommand)]
#[command(name = "base64", desc = "Convert to and from base64")]
pub enum Base64Commands {
    #[command(name = "decode")]
    Decode(Base64Decode),
    #[command(name = "encode")]
    Encode(Base64Encode)
}

#[async_trait]
impl LuroCommand for Base64Commands {
    async fn run_commands(self, ctx: &LuroContext, slash: LuroResponse) -> anyhow::Result<()> {
        // Call the appropriate subcommand.
        match self {
            Self::Decode(command) => command.run_command(ctx, slash).await,
            Self::Encode(command) => command.run_command(ctx, slash).await
        }
    }
}

#[derive(CommandModel, CreateCommand, Default, Debug, PartialEq, Eq)]
#[command(name = "decode", desc = "Convert a string from base64")]
pub struct Base64Decode {
    /// Decode this string from base64
    #[command(max_length = 2039)]
    string: String
}

#[async_trait]
impl LuroCommand for Base64Decode {
    async fn run_command(self, ctx: &LuroContext, mut slash: LuroResponse) -> anyhow::Result<()> {
        let button = button("encode".to_owned(), "Encode".to_owned());
        let decoded = format!("```\n{}\n```", decode(&self.string)?);

        if decoded.len() > 1000 {
            let embed = ctx.default_embed(&slash.interaction.guild_id).description(decoded);
            slash.embed(embed.build())?.components(button);
            ctx.respond(&mut slash).await
        } else {
            slash.content(decoded).components(button);
            ctx.respond(&mut slash).await
        }
    }

    async fn handle_component(
        _data: Box<MessageComponentInteractionData>,
        ctx: &LuroContext,
        slash: &mut LuroResponse
    ) -> anyhow::Result<()> {
        response(ctx, true, slash).await
    }
}

#[derive(CommandModel, CreateCommand, Default, Debug, PartialEq, Eq)]
#[command(name = "encode", desc = "Convert a string to base64")]
pub struct Base64Encode {
    /// Encode this string to base64
    #[command(max_length = 2039)]
    string: String,
    /// Set to true if you want to call out someone for clicking decoding this
    bait: Option<bool>
}

#[async_trait]
impl LuroCommand for Base64Encode {
    async fn run_command(self, ctx: &LuroContext, mut slash: LuroResponse) -> anyhow::Result<()> {
        let button = button("decode".to_owned(), "Decode".to_owned());
        debug!("Recevied {} string", self.string.len());
        let encoded = if let Some(bait) = self.bait && bait {
            format!("```s\n{}\n```", encode(&self.string))
        } else {
            format!("```\n{}\n```", encode(&self.string))
        };

        if encoded.len() > 1000 {
            let embed = ctx.default_embed(&slash.interaction.guild_id).description(encoded);
            slash.embed(embed.build())?.components(button);
            ctx.respond(&mut slash).await
        } else {
            slash.content(encoded).components(button);
            ctx.respond(&mut slash).await
        }
    }

    async fn handle_component(
        _data: Box<MessageComponentInteractionData>,
        ctx: &LuroContext,
        slash: &mut LuroResponse
    ) -> anyhow::Result<()> {
        response(ctx, false, slash).await
    }
}

/// Encode the passed text
fn encode(input: &str) -> String {
    general_purpose::STANDARD.encode(input)
}

/// Decode the passed text
fn decode(input: &str) -> anyhow::Result<String> {
    Ok(str::from_utf8(&general_purpose::STANDARD.decode(input)?)?.to_owned())
}

/// Return a button
fn button(custom_id: String, label: String) -> Vec<Component> {
    Vec::from([Component::ActionRow(ActionRow {
        components: Vec::from([Component::Button(Button {
            custom_id: Some(custom_id),
            disabled: false,
            emoji: None,
            label: Some(label),
            style: ButtonStyle::Primary,
            url: None
        })])
    })])
}

/// Extract the message within an embed, otherwise fallback to message content
/// Returns formatted content depending on the requested operation and optionally an embed if the interaction contained an embed
async fn extract_message(
    ctx: &LuroContext,
    decode_operation: bool,
    slash: &mut LuroResponse
) -> anyhow::Result<(String, Option<Embed>)> {
    let message = slash.interaction.message.clone();

    let (message, embed) = if let Some(message) = &message {
        if let Some(embed) = message.embeds.first() {
            match embed.description.clone() {
                Some(description) => (description, Some(embed)),
                None => return Err(anyhow!("No description field in embed".to_owned()))
            }
        } else {
            (message.content.clone(), None)
        }
    } else {
        return Err(anyhow!("No message in original interaction".to_owned()));
    };

    // Captures
    debug!(message);
    let regex = Regex::new(REGEX_CODE_BLOCK).unwrap(); // Safe to unwrap as clippy checked to make sure the regex is valid.
    let captures = match regex.captures(&message) {
        Some(captures) => captures,
        None => {
            error!(message, "Failed to match code block regex in this message");
            return Err(anyhow!("Could not find a code block.".to_owned()));
        }
    };

    // Some fancy trickery. Our first group type is for if there is a hidden 'secret', which means we should mention the person if they click the button. If it is not present (group 2) then we just act normal.
    let (secret, capture) = if let Some(capture) = captures.get(1) {
        if decode_operation {
            slash.content(format!(
                "Looks like <@{}> just got baited into revealing the message...",
                slash.interaction.author_id().context("Expected interaction user id")?
            ));
            ctx.send_message(slash).await?;
        }
        ("s", capture.as_str())
    } else if let Some(capture) = captures.get(2) {
        ("", capture.as_str())
    } else {
        return Err(anyhow!("Captures found, but could not find any matches.".to_owned()));
    };

    let content = if decode_operation {
        format!("```{}\n{}\n```", secret, decode(capture)?)
    } else {
        format!("```{}\n{}\n```", secret, encode(capture))
    };

    Ok((content, embed.cloned()))
}

async fn response(ctx: &LuroContext, decode_operation: bool, slash: &mut LuroResponse) -> anyhow::Result<()> {
    let (content, interaction_embed) = extract_message(ctx, decode_operation, slash).await?;
    let button = if !decode_operation {
        button("decode".to_owned(), "Decode".to_owned())
    } else {
        button("encode".to_owned(), "Encode".to_owned())
    };

    if let Some(mut embed) = interaction_embed {
        // If an embed is already defined, modify and return it
        embed.description = Some(content);
        slash.embed(embed)?.content(String::new()).components(button).update();
        ctx.respond(slash).await
    } else if content.len() > 1000 {
        // If our string is over 1000 characters, return an embed
        let embed = ctx.default_embed(&slash.interaction.guild_id).description(content).build();
        slash.embed(embed)?.content(String::new()).components(button).update();
        ctx.respond(slash).await
    } else {
        // Otherwise, just return it as text
        slash.content(content).components(button).update();
        ctx.respond(slash).await
    }
}
