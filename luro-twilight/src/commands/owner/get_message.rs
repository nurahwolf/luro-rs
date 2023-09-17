
use async_trait::async_trait;
use luro_framework::{InteractionCommand, Framework, LuroInteraction};
use luro_framework::command::LuroCommandTrait;
use luro_model::database_driver::LuroDatabaseDriver;
use luro_model::message::LuroMessage;
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
#[async_trait]
impl LuroCommandTrait for Message {
    async fn handle_interaction<D: LuroDatabaseDriver>(
        ctx: Framework,
        interaction: InteractionCommand,
    ) -> anyhow::Result<()> {
        let data = Self::new(interaction.data.clone())?;
        let message_id = Id::new(data.message_id.parse()?);
        let channel_id = data.channel_id.unwrap_or(interaction.clone().channel.id);
        let mut embed = interaction.default_embed(&ctx).await;

        // Attempts to fetch in this order
        // User Data -> Client -> Cache
        let mut luro_message = match data.user {
            Some(user) => ctx
                .database
                .get_user(&user.resolved.id)
                .await?
                .messages
                .get(&message_id)
                .cloned(),
            None => None,
        };

        // If not present, try to get from the client
        if luro_message.is_none() {
            luro_message = match ctx.twilight_client.message(channel_id, message_id).await {
                Ok(message) => Some(LuroMessage::from(message.model().await?)),
                Err(_) => None,
            }
        }

        // Last ditch effort, is it in the cache?
        if luro_message.is_none() {
            luro_message = ctx
                .cache
                .message(message_id)
                .map(|message| LuroMessage::from(message.clone()))
        }

        match luro_message {
            Some(message) => {
                let user = ctx.database.get_user(&message.author).await?;

                let toml = toml::to_string_pretty(&message)?;
                embed
                    .author(|author| author.name(user.name()).icon_url(user.avatar()).url(message.link()))
                    .description(message.content)
                    .create_field("Channel", &format!("<#{}>", channel_id), true)
                    .create_field("Message ID", &message_id.to_string(), true)
                    .create_field("LuroMessage", &format!("```toml\n{toml}\n```"), false)
            }
            None => embed
                .description("No message found! If it is hard to find, try specifying my optional parameters!")
                .colour(COLOUR_DANGER),
        };

        interaction.respond(&ctx, |r| r.add_embed(embed).ephemeral()).await
    }
}
