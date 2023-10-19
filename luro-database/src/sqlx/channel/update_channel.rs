use twilight_model::{
    channel::Channel,
    gateway::payload::incoming::{ChannelCreate, ChannelDelete, ChannelUpdate},
};

use crate::{sqlx::channel::DbChannel, DbChannelType, LuroDatabase};

impl LuroDatabase {
    pub async fn update_channel(&self, channel: impl Into<DbChannelType>) -> Result<DbChannel, sqlx::Error> {
        let channel: DbChannelType = channel.into();

        match channel {
            DbChannelType::DbChannel(_) => todo!(),
            DbChannelType::Channel(channel) => handle_channel(self, channel).await,
            DbChannelType::ChannelCreate(channel) => handle_channel_create(self, channel).await,
            DbChannelType::ChannelDelete(channel) => handle_channel_delete(self, channel).await,
            DbChannelType::ChannelUpdate(channel) => handle_channel_update(self, channel).await,
            DbChannelType::ChannelPinsUpdate(_) => todo!(),
        }
    }
}

async fn handle_channel(db: &LuroDatabase, channel: Channel) -> Result<DbChannel, sqlx::Error> {
    sqlx::query_as!(
        DbChannel,
        "INSERT INTO channels (
            channel_id,
            guild_id
        ) VALUES
            ($1, $2)
        ON CONFLICT
            (channel_id)
        DO UPDATE SET
            channel_id = $1,
            guild_id = $2
        RETURNING
            channel_id,
            deleted,
            guild_id",
        channel.id.get() as i64,
        channel.guild_id.map(|x| x.get() as i64)
    )
    .fetch_one(&db.pool)
    .await
}

async fn handle_channel_create(db: &LuroDatabase, channel: Box<ChannelCreate>) -> Result<DbChannel, sqlx::Error> {
    sqlx::query_as!(
        DbChannel,
        "INSERT INTO channels (
            channel_id,
            guild_id
        ) VALUES
            ($1, $2)
        ON CONFLICT
            (channel_id)
        DO UPDATE SET
            channel_id = $1,
            guild_id = $2
        RETURNING
            channel_id,
            deleted,
            guild_id",
        channel.id.get() as i64,
        channel.guild_id.map(|x| x.get() as i64)
    )
    .fetch_one(&db.pool)
    .await
}

async fn handle_channel_update(db: &LuroDatabase, channel: Box<ChannelUpdate>) -> Result<DbChannel, sqlx::Error> {
    sqlx::query_as!(
        DbChannel,
        "INSERT INTO channels (
            channel_id,
            guild_id
        ) VALUES
            ($1, $2)
        ON CONFLICT
            (channel_id)
        DO UPDATE SET
            channel_id = $1,
            guild_id = $2
        RETURNING
            channel_id,
            deleted,
            guild_id",
        channel.id.get() as i64,
        channel.guild_id.map(|x| x.get() as i64)
    )
    .fetch_one(&db.pool)
    .await
}

async fn handle_channel_delete(db: &LuroDatabase, channel: Box<ChannelDelete>) -> Result<DbChannel, sqlx::Error> {
    sqlx::query_as!(
        DbChannel,
        "INSERT INTO channels (
            channel_id,
            deleted,
            guild_id
        ) VALUES
            ($1, $2, $3)
        ON CONFLICT
            (channel_id)
        DO UPDATE SET
            channel_id = $1,
            deleted = $2, 
            guild_id = $3
        RETURNING
            channel_id,
            deleted,
            guild_id",
        channel.id.get() as i64,
        true,
        channel.guild_id.map(|x| x.get() as i64)
    )
    .fetch_one(&db.pool)
    .await
}
