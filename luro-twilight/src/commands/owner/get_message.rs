use luro_framework::{CommandInteraction, Luro, LuroCommand};
use luro_model::COLOUR_DANGER;
use twilight_interactions::command::{CommandModel, CreateCommand, ResolvedUser};
use twilight_model::id::marker::ChannelMarker;
use twilight_model::id::Id;

#[derive(CommandModel, CreateCommand, Debug, PartialEq, Eq)]
#[command(name = "get_message", desc = "Gets a particular message from the cache, or user's data")]
pub struct Message {
    /// The message ID to get
    message_id: String,
    /// If defined, attempts to find the message from this user's data if not found in the cache
    user: Option<ResolvedUser>,
    /// If defined, attempts to use the client to fetch the message
    channel_id: Option<Id<ChannelMarker>>,
}

impl LuroCommand for Message {
    async fn interaction_command(self, ctx: CommandInteraction) -> anyhow::Result<()> {
        let channel_id = self.channel_id.unwrap_or(ctx.clone().channel.id);
        let mut embed = ctx.default_embed().await;

        // Attempts to fetch in this order
        // Database -> Client -> Cache
        let luro_message = ctx.database.get_message(self.message_id.parse()?).await?;

        match luro_message {
            Some(message) => {
                let user = ctx.fetch_user(message.author.id).await?;

                let toml = toml::to_string_pretty(&message)?;
                embed
                    .author(|author| author.name(user.name()).icon_url(user.avatar_url()).url(message.link()))
                    .description(message.content)
                    .create_field("Channel", &format!("<#{}>", channel_id), true)
                    .create_field("Message ID", &self.message_id, true)
                    .create_field("LuroMessage", &format!("```toml\n{toml}\n```"), false)
            }
            None => embed
                .description("No message found! If it is hard to find, try specifying my optional parameters!")
                .colour(COLOUR_DANGER),
        };

        ctx.respond(|r| r.add_embed(embed).ephemeral()).await
    }
}
