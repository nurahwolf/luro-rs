use anyhow::Context;
use luro_framework::{CommandInteraction, Luro, LuroCommand};
use luro_model::response::SimpleResponse;
use regex::Regex;
use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_model::id::Id;

#[derive(CommandModel, CreateCommand, Debug, PartialEq, Eq)]
#[command(name = "message", desc = "Resolve a message. This also adds the message to the DB.")]
pub struct Message {
    /// Give a message link or message ID.
    message_id: String,
}

impl LuroCommand for Message {
    async fn interaction_command(self, ctx: CommandInteraction) -> anyhow::Result<luro_model::types::CommandResponse> {
        let link_regex = Regex::new(r"^https.*/(\d+)/(\d+)")?;
        let number_regex = Regex::new(r"\d+")?;

        let message = if let Some(captures) = link_regex.captures(&self.message_id) {
            tracing::debug!("{captures:#?}");
            let message_id = captures.get(2).context("Expected to get message_id")?.as_str().parse()?;
            let channel_id = captures.get(1).context("Expected to get channel_id")?.as_str().parse()?;

            match ctx.database.message_fetch(Id::new(message_id), Some(Id::new(channel_id))).await {
                Ok(message) => message,
                Err(why) => return ctx.simple_response(SimpleResponse::InternalError(&why)).await,
            }
        } else if let Some(message_id) = number_regex.find(&self.message_id).map(|mat| mat.as_str().parse()) {
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

        ctx.respond(|response| {
            response.embed(|embed| embed.title(message.source.to_string()).description(message.content).colour(ctx.accent_colour()))
        })
        .await
    }
}
