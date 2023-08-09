use luro_model::{luro_database_driver::LuroDatabaseDriver, luro_log_channel::LuroLogChannel};
use tracing::debug;
use twilight_model::gateway::payload::incoming::ThreadCreate;

use twilight_util::builder::embed::{EmbedBuilder, EmbedFieldBuilder};

use crate::framework::Framework;

impl<D: LuroDatabaseDriver> Framework<D> {
    // TODO: Change this to a response type
    pub async fn response_thread_created(&self, event: &ThreadCreate) -> anyhow::Result<()> {
        let embed = self.embed_thread_created(event).await;
        self.send_log_channel(&event.guild_id, embed, LuroLogChannel::Thread).await
    }

    /// Returns an embed containing a standardised error message that we were unable to get the channel that an interaction took place in.
    pub async fn embed_thread_created(&self, event: &ThreadCreate) -> EmbedBuilder {
        debug!(thread = ?event, "Thread created");
        let mut embed = self.default_embed(&event.guild_id).await.title("Thread Created");

        match &event.name {
            Some(name) => embed = embed.field(EmbedFieldBuilder::new("Name", format!("{name} - <#{}>", event.id)).inline()),
            None => embed = embed.field(EmbedFieldBuilder::new("Name", format!("<#{}>", event.id)).inline())
        }

        let channel_type = match event.kind {
            twilight_model::channel::ChannelType::GuildText => "GuildText",
            twilight_model::channel::ChannelType::Private => "Private",
            twilight_model::channel::ChannelType::GuildVoice => "GuildVoice",
            twilight_model::channel::ChannelType::Group => "Group",
            twilight_model::channel::ChannelType::GuildCategory => "GuildCategory",
            twilight_model::channel::ChannelType::GuildAnnouncement => "GuildAnnouncement",
            twilight_model::channel::ChannelType::AnnouncementThread => "AnnouncementThread",
            twilight_model::channel::ChannelType::PublicThread => "PublicThread",
            twilight_model::channel::ChannelType::PrivateThread => "PrivateThread",
            twilight_model::channel::ChannelType::GuildStageVoice => "GuildStageVoice",
            twilight_model::channel::ChannelType::GuildDirectory => "GuildDirectory",
            twilight_model::channel::ChannelType::GuildForum => "GuildForum",
            _ => "Unknown"
        };

        if let Some(parent_id) = event.parent_id {
            embed = embed.field(EmbedFieldBuilder::new("Parent Channel", format!("<#{parent_id}>")).inline());
        }
        embed = embed.field(EmbedFieldBuilder::new("Type", channel_type).inline());
        embed
    }
}
