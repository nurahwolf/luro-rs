use anyhow::Context;
use luro_framework::Luro;
use luro_model::response::SimpleResponse;
use regex::Regex;
use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_model::id::Id;

#[derive(CommandModel, CreateCommand)]
#[command(name = "add", desc = "Save what someone said!")]
pub struct Add {
    /// Either give the message ID, or a link to the message!
    message: String,
}

impl luro_framework::LuroCommand for Add {
    async fn interaction_command(self, ctx: luro_framework::CommandInteraction) -> anyhow::Result<luro_model::types::CommandResponse> {
        let link_regex = Regex::new(r"^https.*/(\d+)/(\d+)")?;
        let number_regex = Regex::new(r"\d+")?;

        let message = if let Some(captures) = link_regex.captures(&self.message) {
            tracing::debug!("{captures:#?}");
            let message_id = captures.get(2).context("Expected to get message_id")?.as_str().parse()?;
            let channel_id = captures.get(1).context("Expected to get channel_id")?.as_str().parse()?;

            match ctx.database.message_fetch(Id::new(message_id), Some(Id::new(channel_id))).await {
                Ok(message) => message,
                Err(why) => return ctx.simple_response(SimpleResponse::InternalError(&why)).await,
            }
        } else if let Some(message_id) = number_regex.find(&self.message).map(|mat| mat.as_str().parse()) {
            match ctx
                .database
                .message_fetch(Id::new(message_id.context("Expected to get message_id")?), Some(ctx.channel.id))
                .await
            {
                Ok(message) => message,
                Err(why) => return ctx.simple_response(SimpleResponse::InternalError(&why)).await,
            }
        } else {
            return ctx
                .respond(|r| r.content("No message ID or message link passed!").ephemeral())
                .await;
        };

        let quote_id = ctx
            .database
            .driver
            .quote_add(ctx.author.user_id, &message, ctx.channel.nsfw.unwrap_or_default())
            .await?;

        ctx.respond(|response| {
            response.embed(|embed| {
                embed.colour(ctx.accent_colour()).description(message.content).author(|author| {
                    author
                        .name(format!("{} - Quote {quote_id}", message.author.name()))
                        .icon_url(message.author.avatar_url());
                    match message.guild_id {
                        Some(guild_id) => author.url(format!(
                            "https://discord.com/channels/{guild_id}/{}/{}",
                            message.channel_id, message.id
                        )),
                        None => author.url(format!("https://discord.com/channels/{}/{}", message.channel_id, message.id)),
                    }
                })
            })
        })
        .await
    }
}
