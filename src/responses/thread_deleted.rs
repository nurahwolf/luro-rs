use tracing::debug;

use twilight_model::gateway::payload::incoming::ThreadDelete;

use twilight_util::builder::embed::{EmbedBuilder, EmbedFieldBuilder};

use crate::COLOUR_DANGER;

use crate::framework::LuroFramework;
use crate::models::LuroLogChannel;

impl LuroFramework {
    // TODO: Change this to a response type
    pub async fn response_thread_deleted(&self, event: &ThreadDelete) -> anyhow::Result<()> {
        let embed = self.embed_thread_deleted(event);
        self.send_log_channel(&Some(event.guild_id), embed, LuroLogChannel::Moderator)
            .await
    }

    /// Returns an embed containing a standardised error message that we were unable to get the channel that an interaction took place in.
    pub fn embed_thread_deleted(&self, event: &ThreadDelete) -> EmbedBuilder {
        debug!(thread = ?event, "Thread Deleted!");
        let mut embed = self
            .default_embed(&Some(event.guild_id))
            .color(COLOUR_DANGER)
            .title("Thread Deleted!");

        let channel_type = if let Some(thread) = self.twilight_cache.channel(event.id) {
            match &thread.name {
                Some(name) => {
                    embed = embed.field(EmbedFieldBuilder::new("Name", format!("{name} - <#{0}> - {0}", thread.id)).inline())
                }
                None => embed = embed.field(EmbedFieldBuilder::new("ID", format!("<#{0}> - {0}", thread.id)).inline())
            }
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
                _ => "Unknown"
            }
        } else {
            embed = embed.field(EmbedFieldBuilder::new("ID", format!("<#{0}> - {0}", event.id)).inline());
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
                _ => "Unknown"
            }
        };
        embed = embed.field(EmbedFieldBuilder::new("Parent Channel", format!("<#{}>", event.parent_id)).inline());
        embed = embed.field(EmbedFieldBuilder::new("Type", channel_type).inline());
        embed
    }
}
