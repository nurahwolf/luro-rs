use anyhow::Context;
use luro_framework::{CommandInteraction, Luro, LuroCommand, Response};
use regex::Regex;
use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_model::id::Id;

#[derive(CommandModel, CreateCommand, Debug)]
#[command(name = "message", desc = "Information about a message")]
pub struct Message {
    /// Pass either the message ID, or a message link
    message: String,
}

impl LuroCommand for Message {
    async fn interaction_command(self, ctx: CommandInteraction) -> anyhow::Result<luro_model::types::CommandResponse> {
        let link_regex = Regex::new(r"^https.*/(\d+)/(\d+)")?;
        let number_regex = Regex::new(r"\d+")?;

        let message = if let Some(captures) = link_regex.captures(&self.message) {
            tracing::debug!("{captures:#?}");
            let message_id = captures.get(2).context("Expected to get message_id")?.as_str().parse()?;
            let channel_id = captures.get(1).context("Expected to get channel_id")?.as_str().parse()?;

            match ctx.database.message_fetch(Id::new(message_id), Some(Id::new(channel_id))).await {
                Ok(message) => message,
                Err(why) => return ctx.response_simple(Response::InternalError(why)).await,
            }
        } else if let Some(message_id) = number_regex.find(&self.message).map(|mat| mat.as_str().parse()) {
            match ctx
                .database
                .message_fetch(Id::new(message_id.context("Expected to get message_id")?), Some(ctx.channel.id))
                .await
            {
                Ok(message) => message,
                Err(why) => return ctx.response_simple(Response::InternalError(why)).await,
            }
        } else {
            return ctx
                .respond(|r| r.content("No message ID or message link passed!").ephemeral())
                .await;
        };

        let toml = toml::to_string_pretty(&message)?;
        let mut embed = ctx.default_embed().await;
        embed
            .author(|author| {
                author
                    .name(format!("Message by {} | {}", message.author.name(), message.author.user_id))
                    .icon_url(message.author.avatar_url())
                    .url(message.link())
            })
            .description(message.content)
            .create_field("Channel", &format!("<#{}>", message.channel_id), true)
            .create_field("Message ID", &self.message, true)
            .create_field("Raw Message Data", &format!("```toml\n{toml}\n```"), false);

        ctx.respond(|r| r.add_embed(embed)).await
    }
}
