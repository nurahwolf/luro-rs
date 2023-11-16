use luro_model::sync::ChannelSync;
use twilight_model::{
    channel::Channel,
    gateway::payload::incoming::{ChannelCreate, ChannelDelete, ChannelUpdate},
    id::{marker::ChannelMarker, Id},
};

use crate::SQLxDriver;

impl SQLxDriver {
    pub async fn update_channel(&self, channel: impl Into<ChannelSync<'_>>) -> Result<u64, sqlx::Error> {
        let channel: ChannelSync = channel.into();

        match channel {
            ChannelSync::ChannelID(channel) => handle_channel_id(self, channel).await,
            ChannelSync::Channel(channel) => handle_channel(self, channel).await,
            ChannelSync::ChannelCreate(channel) => handle_channel_create(self, channel).await,
            ChannelSync::ChannelDelete(channel) => handle_channel_delete(self, channel).await,
            ChannelSync::ChannelUpdate(channel) => handle_channel_update(self, channel).await,
            ChannelSync::ChannelPinsUpdate(_) => todo!(),
        }
    }
}

async fn handle_channel_id(db: &SQLxDriver, channel: Id<ChannelMarker>) -> Result<u64, sqlx::Error> {
    sqlx::query_file!("queries/channel/channel_update_channel_id.sql", channel.get() as i64)
        .execute(&db.pool)
        .await
        .map(|x| x.rows_affected())
}

async fn handle_channel(db: &SQLxDriver, channel: &Channel) -> Result<u64, sqlx::Error> {
    sqlx::query_file!(
        "queries/channel/channel_update_twilight_channel.sql",
        channel.id.get() as i64,
        channel.guild_id.map(|x| x.get() as i64)
    )
    .execute(&db.pool)
    .await
    .map(|x| x.rows_affected())
}

async fn handle_channel_create(db: &SQLxDriver, channel: &ChannelCreate) -> Result<u64, sqlx::Error> {
    sqlx::query_file!(
        "queries/channel/channel_update_twilight_channel.sql",
        channel.id.get() as i64,
        channel.guild_id.map(|x| x.get() as i64)
    )
    .execute(&db.pool)
    .await
    .map(|x| x.rows_affected())
}

async fn handle_channel_update(db: &SQLxDriver, channel: &ChannelUpdate) -> Result<u64, sqlx::Error> {
    sqlx::query_file!(
        "queries/channel/channel_update_twilight_channel.sql",
        channel.id.get() as i64,
        channel.guild_id.map(|x| x.get() as i64)
    )
    .execute(&db.pool)
    .await
    .map(|x| x.rows_affected())
}

async fn handle_channel_delete(db: &SQLxDriver, channel: &ChannelDelete) -> Result<u64, sqlx::Error> {
    sqlx::query_file!(
        "queries/channel/channel_update_twilight_channel.sql",
        channel.id.get() as i64,
        channel.guild_id.map(|x| x.get() as i64)
    )
    .execute(&db.pool)
    .await
    .map(|x| x.rows_affected())
}
