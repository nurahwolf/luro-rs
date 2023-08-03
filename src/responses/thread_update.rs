use tracing::debug;
use twilight_model::gateway::payload::incoming::ThreadUpdate;

use twilight_util::builder::embed::{EmbedBuilder, EmbedFieldBuilder};

use crate::models::LuroFramework;

impl LuroFramework {
    pub async fn response_thread_update(&self, event: &ThreadUpdate) -> anyhow::Result<()> {
        let embed = self.embed_thread_update(event);
        self.send_log_channel(&event.guild_id, embed, crate::models::LuroLogChannel::Thread)
            .await
    }

    pub fn embed_thread_update(&self, event: &ThreadUpdate) -> EmbedBuilder {
        debug!(thread = ?event, "Thread Updated!");
        let mut embed = self.default_embed(&event.guild_id).title("Thread Updated");

        match &event.name {
            Some(name) => embed = embed.field(EmbedFieldBuilder::new("Name", format!("{name} - <#{}>", event.id)).inline()),
            None => embed = embed.field(EmbedFieldBuilder::new("ID", format!("<#{}>", event.id)).inline())
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
        embed = embed.field(EmbedFieldBuilder::new("Type", channel_type).inline());
        embed
    }
}
