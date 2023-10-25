use sqlx::{types::Json, postgres::PgQueryResult};
use twilight_model::{
    channel::{
        forum::{DefaultReaction, ForumTag},
        permission_overwrite::PermissionOverwrite,
        thread::{ThreadMember, ThreadMetadata},
        Channel,
    },
    gateway::payload::incoming::{ChannelCreate, ChannelDelete, ChannelUpdate}, id::{Id, marker::ChannelMarker},
};

use crate::{sqlx::channel::DbChannel, DbChannelType, LuroDatabase};

impl LuroDatabase {
    pub async fn update_channel(&self, channel: impl Into<DbChannelType>) -> Result<u64, sqlx::Error> {
        let channel: DbChannelType = channel.into();

        match channel {
            DbChannelType::ChannelID(channel) => handle_channel_id(self, channel).await.map(|x|x.rows_affected()),
            DbChannelType::DbChannel(_) => todo!(),
            DbChannelType::Channel(channel) => handle_channel(self, channel).await.map(|_|0),
            DbChannelType::ChannelCreate(channel) => handle_channel_create(self, channel).await.map(|_|0),
            DbChannelType::ChannelDelete(channel) => handle_channel_delete(self, channel).await.map(|_|0),
            DbChannelType::ChannelUpdate(channel) => handle_channel_update(self, channel).await.map(|_|0),
            DbChannelType::ChannelPinsUpdate(_) => todo!(),
        }
    }
}

async fn handle_channel_id(db: &LuroDatabase, channel: Id<ChannelMarker>) -> Result<PgQueryResult, sqlx::Error> {
    sqlx::query_file!(
        "queries/channels/update_channel_id.sql",
        channel.get() as i64
    ).execute(&db.pool).await
}

async fn handle_channel(db: &LuroDatabase, channel: Channel) -> Result<DbChannel, sqlx::Error> {
    sqlx::query_as!(
        DbChannel,
        "
        INSERT INTO channels (
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
            application_id,
            applied_tags,
            available_tags as \"available_tags: Vec<Json<ForumTag>>\",
            bitrate,
            channel_id,
            default_auto_archive_duration,
            default_forum_layout,
            default_reaction_emoji as \"default_reaction_emoji: Json<DefaultReaction>\",
            default_sort_order,
            default_thread_rate_limit_per_user,
            deleted,
            flags,
            guild_id,
            icon,
            invitable,
            kind,
            last_message_id,
            last_pin_timestamp,
            managed,
            member_count,
            member as \"member: Json<ThreadMember>\",
            message_count,
            name,
            newly_created,
            nsfw,
            owner_id,
            parent_id,
            permission_overwrites as \"permission_overwrites: Vec<Json<PermissionOverwrite>>\",
            position,
            rate_limit_per_user,
            recipients,
            rtc_region,
            thread_metadata as \"thread_metadata: Json<ThreadMetadata>\",
            topic,
            user_limit,
            video_quality_mode
        ",
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
            application_id,
            applied_tags,
            available_tags as \"available_tags: Vec<Json<ForumTag>>\",
            bitrate,
            channel_id,
            default_auto_archive_duration,
            default_forum_layout,
            default_reaction_emoji as \"default_reaction_emoji: Json<DefaultReaction>\",
            default_sort_order,
            default_thread_rate_limit_per_user,
            deleted,
            flags,
            guild_id,
            icon,
            invitable,
            kind,
            last_message_id,
            last_pin_timestamp,
            managed,
            member_count,
            member as \"member: Json<ThreadMember>\",
            message_count,
            name,
            newly_created,
            nsfw,
            owner_id,
            parent_id,
            permission_overwrites as \"permission_overwrites: Vec<Json<PermissionOverwrite>>\",
            position,
            rate_limit_per_user,
            recipients,
            rtc_region,
            thread_metadata as \"thread_metadata: Json<ThreadMetadata>\",
            topic,
            user_limit,
            video_quality_mode
        ",
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
            application_id,
            applied_tags,
            available_tags as \"available_tags: Vec<Json<ForumTag>>\",
            bitrate,
            channel_id,
            default_auto_archive_duration,
            default_forum_layout,
            default_reaction_emoji as \"default_reaction_emoji: Json<DefaultReaction>\",
            default_sort_order,
            default_thread_rate_limit_per_user,
            deleted,
            flags,
            guild_id,
            icon,
            invitable,
            kind,
            last_message_id,
            last_pin_timestamp,
            managed,
            member_count,
            member as \"member: Json<ThreadMember>\",
            message_count,
            name,
            newly_created,
            nsfw,
            owner_id,
            parent_id,
            permission_overwrites as \"permission_overwrites: Vec<Json<PermissionOverwrite>>\",
            position,
            rate_limit_per_user,
            recipients,
            rtc_region,
            thread_metadata as \"thread_metadata: Json<ThreadMetadata>\",
            topic,
            user_limit,
            video_quality_mode
        ",
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
            application_id,
            applied_tags,
            available_tags as \"available_tags: Vec<Json<ForumTag>>\",
            bitrate,
            channel_id,
            default_auto_archive_duration,
            default_forum_layout,
            default_reaction_emoji as \"default_reaction_emoji: Json<DefaultReaction>\",
            default_sort_order,
            default_thread_rate_limit_per_user,
            deleted,
            flags,
            guild_id,
            icon,
            invitable,
            kind,
            last_message_id,
            last_pin_timestamp,
            managed,
            member_count,
            member as \"member: Json<ThreadMember>\",
            message_count,
            name,
            newly_created,
            nsfw,
            owner_id,
            parent_id,
            permission_overwrites as \"permission_overwrites: Vec<Json<PermissionOverwrite>>\",
            position,
            rate_limit_per_user,
            recipients,
            rtc_region,
            thread_metadata as \"thread_metadata: Json<ThreadMetadata>\",
            topic,
            user_limit,
            video_quality_mode
        ",
        channel.id.get() as i64,
        true,
        channel.guild_id.map(|x| x.get() as i64)
    )
    .fetch_one(&db.pool)
    .await
}
