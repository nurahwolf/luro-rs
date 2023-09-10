use luro_builder::embed::EmbedBuilder;
use luro_model::database_driver::LuroDatabaseDriver;
use tracing::debug;
use twilight_model::gateway::payload::incoming::ThreadDelete;

use crate::{framework::Framework, COLOUR_DANGER};

impl<D: LuroDatabaseDriver> Framework<D> {
    // TODO: Change this to a response type
    pub async fn response_thread_deleted(&self, event: &ThreadDelete) -> anyhow::Result<()> {
        let embed = self.embed_thread_deleted(event).await;
        self.send_moderator_log_channel(&Some(event.guild_id), embed).await
    }

    /// Returns an embed containing a standardised error message that we were unable to get the channel that an interaction took place in.
    pub async fn embed_thread_deleted(&self, event: &ThreadDelete) -> EmbedBuilder {
        debug!(thread = ?event, "Thread Deleted!");
        let mut embed = self.default_embed(&Some(event.guild_id)).await;

        let channel_type = if let Some(thread) = self.twilight_cache.channel(event.id) {
            match &thread.name {
                Some(name) => embed.create_field("Name", &format!("{name} - <#{0}> - {0}", thread.id), true),
                None => embed.create_field("ID", &format!("<#{0}> - {0}", thread.id), true),
            };
            match thread.kind {
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
                _ => "Unknown",
            }
        } else {
            embed.create_field("ID", &format!("<#{0}> - {0}", event.id), true);
            match event.kind {
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
                _ => "Unknown",
            }
        };
        embed.create_field("Parent Channel", &format!("<#{}>", event.parent_id), true);
        embed
            .create_field("Type", channel_type, true)
            .colour(COLOUR_DANGER)
            .title("Thread Deleted!");
        embed
    }
}
