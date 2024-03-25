use twilight_model::{
    channel::{Channel, ChannelType},
    id::{marker::ChannelMarker, Id},
};

use crate::database::sqlx::{Database, Error};

impl Database {
    pub async fn fetch_channel(&self, channel_id: Id<ChannelMarker>) -> Result<Channel, Error> {
        match fetch_channel(self, channel_id).await {
            Ok(Some(data)) => return Ok(data),
            Ok(None) => tracing::debug!("Channel `{channel_id}` was not found in the database."),
            Err(why) => tracing::error!(?why, "Error raised while trying to find channel `{channel_id}`"),
        };

        Ok(self.twilight_driver.fetch_channel(channel_id).await?)
    }
}

async fn fetch_channel(db: &Database, id: Id<ChannelMarker>) -> Result<Option<Channel>, sqlx::Error> {
    // TODO: Complete this
    sqlx::query_file!("src/database/sqlx/queries/channel/channel_fetch.sql", id.get() as i64)
        .fetch_optional(&db.pool)
        .await
        .map(|x| {
            x.map(|channel| twilight_model::channel::Channel {
                application_id: Default::default(),
                applied_tags: Default::default(),
                available_tags: Default::default(),
                bitrate: Default::default(),
                // deleted: Default::default(),
                default_auto_archive_duration: Default::default(),
                default_forum_layout: Default::default(),
                default_reaction_emoji: Default::default(),
                default_sort_order: Default::default(),
                default_thread_rate_limit_per_user: Default::default(),
                flags: Default::default(),
                guild_id: Default::default(),
                icon: Default::default(),
                id,
                invitable: Default::default(),
                kind: ChannelType::from(channel.kind as u8),
                last_message_id: Default::default(),
                last_pin_timestamp: Default::default(),
                managed: Default::default(),
                member: Default::default(),
                member_count: Default::default(),
                message_count: Default::default(),
                name: Default::default(),
                newly_created: Default::default(),
                nsfw: Default::default(),
                owner_id: Default::default(),
                parent_id: Default::default(),
                permission_overwrites: Default::default(),
                position: Default::default(),
                rate_limit_per_user: Default::default(),
                recipients: Default::default(),
                rtc_region: Default::default(),
                thread_metadata: Default::default(),
                topic: Default::default(),
                user_limit: Default::default(),
                video_quality_mode: Default::default(),
            })
        })
}
