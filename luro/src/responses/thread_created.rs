use luro_builder::embed::EmbedBuilder;
use luro_model::{database::drivers::LuroDatabaseDriver, guild::log_channel::LuroLogChannel};
use tracing::debug;
use twilight_model::gateway::payload::incoming::ThreadCreate;

use crate::framework::Framework;

impl<D: LuroDatabaseDriver> Framework<D> {
    // TODO: Change this to a response type
    pub async fn response_thread_created(&self, event: &ThreadCreate) -> anyhow::Result<()> {
        let embed = self.embed_thread_created(event).await;
        self.send_log_channel(&event.guild_id, embed.into(), LuroLogChannel::Thread)
            .await
    }

    /// Returns an embed containing a standardised error message that we were unable to get the channel that an interaction took place in.
    pub async fn embed_thread_created(&self, event: &ThreadCreate) -> EmbedBuilder {
        debug!(thread = ?event, "Thread created");
        let mut embed = self.default_embed(&event.guild_id).await;

        match &event.name {
            Some(name) => embed.create_field("Name", &format!("{name} - <#{}>", event.id), true),
            None => embed.create_field("ID", &format!("<#{}>", event.id), true)
        };

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
            embed.create_field("Parent Channel", &format!("<#{parent_id}>"), true);
        }
        embed.create_field("Type", channel_type, true).title("Thread Created");
        embed
    }
}
