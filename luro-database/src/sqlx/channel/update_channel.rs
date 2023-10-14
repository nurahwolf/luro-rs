use twilight_model::gateway::payload::incoming::{ChannelCreate, ChannelUpdate, ChannelDelete};

use crate::{LuroDatabase, sqlx::channel::DbChannel, DbChannelType};

impl LuroDatabase {
    pub async fn update_channel(&self, channel: impl Into<DbChannelType>) -> Result<DbChannel, sqlx::Error> {
        let channel: DbChannelType = channel.into();

        match channel {
            DbChannelType::DbChannel(_) => todo!(),
            DbChannelType::Channel(_) => todo!(),
            DbChannelType::ChannelCreate(channel) => handle_channel_create(self, channel).await,
            DbChannelType::ChannelDelete(channel) => handle_channel_delete(self, channel).await,
            DbChannelType::ChannelUpdate(channel) => handle_channel_update(self, channel).await,
            DbChannelType::ChannelPinsUpdate(_) => todo!(),
            
        }
    }
}

async fn handle_channel_create(db: &LuroDatabase, channel: Box<ChannelCreate>) -> Result<DbChannel, sqlx::Error> {
    sqlx::query_as!(
        DbChannel,
        "INSERT INTO guild_channels (
            channel_id
        ) VALUES
            ($1)
        ON CONFLICT
            (channel_id)
        DO UPDATE SET
            channel_id = $1
        RETURNING
            channel_id,
            deleted",
        channel.id.get() as i64
    ).fetch_one(&db.pool).await
}

async fn handle_channel_update(db: &LuroDatabase, channel: Box<ChannelUpdate>) -> Result<DbChannel, sqlx::Error> {
    sqlx::query_as!(
        DbChannel,
        "INSERT INTO guild_channels (
            channel_id
        ) VALUES
            ($1)
        ON CONFLICT
            (channel_id)
        DO UPDATE SET
            channel_id = $1
        RETURNING
            channel_id,
            deleted",
        channel.id.get() as i64
    ).fetch_one(&db.pool).await
}

async fn handle_channel_delete(db: &LuroDatabase, channel: Box<ChannelDelete>) -> Result<DbChannel, sqlx::Error> {
    sqlx::query_as!(
        DbChannel,
        "INSERT INTO guild_channels (
            channel_id,
            deleted
        ) VALUES
            ($1, $2)
        ON CONFLICT
            (channel_id)
        DO UPDATE SET
            channel_id = $1,
            deleted = $2
        RETURNING
            channel_id,
            deleted",
        channel.id.get() as i64,
        true
    ).fetch_one(&db.pool).await
}