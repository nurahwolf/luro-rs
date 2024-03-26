use twilight_model::{
    channel::Channel,
    gateway::payload::incoming::{ChannelCreate, ChannelDelete, ChannelPinsUpdate, ChannelUpdate},
    id::{marker::ChannelMarker, Id},
};

use crate::{
    database::sqlx::{Database, Error},
    sync::ChannelSync,
};

impl Database {
    pub async fn update_channel(&self, channel: impl Into<ChannelSync<'_>>) -> Result<u64, Error> {
        Ok(match channel.into() {
            ChannelSync::ChannelID(channel) => handle_channel_id(self, channel).await,
            ChannelSync::Channel(channel) => handle_channel(self, channel).await,
            ChannelSync::ChannelCreate(channel) => handle_channel_create(self, channel).await,
            ChannelSync::ChannelDelete(channel) => handle_channel_delete(self, channel).await,
            ChannelSync::ChannelUpdate(channel) => handle_channel_update(self, channel).await,
            ChannelSync::ChannelPinsUpdate(channel) => handle_channel_pins(self, channel).await,
        }?)
    }
}

async fn handle_channel_pins(db: &Database, channel: &ChannelPinsUpdate) -> Result<u64, sqlx::Error> {
    sqlx::query_file!(
        "queries/channel/channel_update_twilight_channel.sql",
        channel.channel_id.get() as i64,
        channel.guild_id.map(|x| x.get() as i64)
    )
    .execute(&db.pool)
    .await
    .map(|x| x.rows_affected())
}

async fn handle_channel_id(db: &Database, channel: Id<ChannelMarker>) -> Result<u64, sqlx::Error> {
    sqlx::query_file!("queries/channel/channel_update_channel_id.sql", channel.get() as i64)
        .execute(&db.pool)
        .await
        .map(|x| x.rows_affected())
}

async fn handle_channel(db: &Database, channel: &Channel) -> Result<u64, sqlx::Error> {
    sqlx::query_file!(
        "queries/channel/channel_update_twilight_channel.sql",
        channel.id.get() as i64,
        channel.guild_id.map(|x| x.get() as i64)
    )
    .execute(&db.pool)
    .await
    .map(|x| x.rows_affected())
}

async fn handle_channel_create(db: &Database, channel: &ChannelCreate) -> Result<u64, sqlx::Error> {
    sqlx::query_file!(
        "queries/channel/channel_update_twilight_channel.sql",
        channel.id.get() as i64,
        channel.guild_id.map(|x| x.get() as i64)
    )
    .execute(&db.pool)
    .await
    .map(|x| x.rows_affected())
}

async fn handle_channel_update(db: &Database, channel: &ChannelUpdate) -> Result<u64, sqlx::Error> {
    sqlx::query_file!(
        "queries/channel/channel_update_twilight_channel.sql",
        channel.id.get() as i64,
        channel.guild_id.map(|x| x.get() as i64)
    )
    .execute(&db.pool)
    .await
    .map(|x| x.rows_affected())
}

async fn handle_channel_delete(db: &Database, channel: &ChannelDelete) -> Result<u64, sqlx::Error> {
    sqlx::query_file!(
        "queries/channel/channel_update_twilight_channel.sql",
        channel.id.get() as i64,
        channel.guild_id.map(|x| x.get() as i64)
    )
    .execute(&db.pool)
    .await
    .map(|x| x.rows_affected())
}
