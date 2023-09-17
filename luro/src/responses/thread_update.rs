use luro_builder::embed::EmbedBuilder;
use luro_model::{database_driver::LuroDatabaseDriver, guild::log_channel::LuroLogChannel};
use tracing::debug;
use twilight_model::gateway::payload::incoming::ThreadUpdate;

use crate::framework::Framework;

impl<D: LuroDatabaseDriver> Framework {
    pub async fn response_thread_update(&self, event: &ThreadUpdate) -> anyhow::Result<()> {
        let embed = self.embed_thread_update(event).await;
        self.send_log_channel(&event.guild_id, embed.into(), LuroLogChannel::Thread)
            .await
    }

    pub async fn embed_thread_update(&self, event: &ThreadUpdate) -> EmbedBuilder {
        debug!(thread = ?event, "Thread Updated!");
        let mut embed = self.default_embed(&event.guild_id).await;
        embed.title("Thread Updated");

        match &event.name {
            Some(name) => embed.create_field("Name", &format!("{name} - <#{}>", event.id), true),
            None => embed.create_field("ID", &format!("<#{}>", event.id), true),
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
            _ => "Unknown",
        };
        embed.create_field("Type", channel_type, true);
        embed
    }
}
