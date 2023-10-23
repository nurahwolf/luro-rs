use sqlx::{postgres::PgQueryResult, Error};
use time::OffsetDateTime;
use tracing::debug;
use twilight_model::{
    gateway::payload::incoming::{MemberAdd, MemberRemove, MemberUpdate, MemberChunk},
    guild::{Member, PartialMember}, id::{Id, marker::GuildMarker},
};

use crate::{DbMemberType, LuroDatabase};

impl LuroDatabase {
    /// Updates a supported member type. Returns the total number of rows modified in the database.
    pub async fn update_member(&self, member: impl Into<DbMemberType>) -> anyhow::Result<u64> {
        let rows_modified = match member.into() {
            DbMemberType::Member(guild_id, member) => handle_member(self, guild_id, member).await?,
            DbMemberType::MemberAdd(member) => handle_member_add(self, member).await?,
            DbMemberType::MemberChunk(member) => handle_member_chunk(self, member).await?,
            DbMemberType::MemberRemove(member) => handle_member_remove(self, member).await?.rows_affected(),
            DbMemberType::MemberUpdate(member) => handle_member_update(self, member).await?,
            DbMemberType::PartialMember(guild_id, member) => handle_partial_member(self, guild_id, member).await?,
        };

        debug!("DB Member: Updated `{rows_modified}` rows!");

        Ok(rows_modified)
    }
}

async fn handle_member_chunk(db: &LuroDatabase, event: MemberChunk) -> anyhow::Result<u64> {
    let mut rows_modified = 0;
    for member in event.members {
        rows_modified += db.update_user(member.user).await?;
        rows_modified += sqlx::query_file!(
            "queries/guild_members/update_twilight_member.sql",
            match member.premium_since {
                Some(timestamp) => Some(OffsetDateTime::from_unix_timestamp(timestamp.as_secs())?),
                None => None,
            },
            match member.communication_disabled_until {
                Some(timestamp) => Some(OffsetDateTime::from_unix_timestamp(timestamp.as_secs())?),
                None => None,
            },
            member.deaf,
            event.guild_id.get() as i64,
            OffsetDateTime::from_unix_timestamp(member.joined_at.as_secs())?,
            member.avatar.map(|x|x.to_string()),
            member.flags.bits() as i64,
            member.mute,
            member.nick,
            member.pending,
        )
        .execute(&db.pool)
        .await?.rows_affected()
    }

    Ok(rows_modified)
}

async fn handle_member_remove(db: &LuroDatabase, member: MemberRemove) -> Result<PgQueryResult, Error>{
    sqlx::query_file!(
        "queries/guild_members/guild_member_remove.sql",
        member.guild_id.get() as i64,
        member.user.id.get() as i64
    )
    .execute(&db.pool)
    .await
}

async fn handle_member(db: &LuroDatabase, guild_id: Id<GuildMarker>, member: Member) -> anyhow::Result<u64> {
    let mut rows_modified = db.update_user(member.user.clone()).await?;
    rows_modified += sqlx::query_file!(
        "queries/guild_members/update_twilight_member.sql",
        match member.premium_since {
            Some(timestamp) => Some(OffsetDateTime::from_unix_timestamp(timestamp.as_secs())?),
            None => None,
        },
        match member.communication_disabled_until {
            Some(timestamp) => Some(OffsetDateTime::from_unix_timestamp(timestamp.as_secs())?),
            None => None,
        },
        member.deaf,
        guild_id.get() as i64,
        OffsetDateTime::from_unix_timestamp(member.joined_at.as_secs())?,
        member.avatar.map(|x|x.to_string()),
        member.flags.bits() as i64,
        member.mute,
        member.nick,
        member.pending,
    )
    .execute(&db.pool)
    .await?.rows_affected();

    Ok(rows_modified)
}

async fn handle_partial_member(db: &LuroDatabase, guild_id: Id<GuildMarker>, member: PartialMember) -> anyhow::Result<u64> {
    let mut rows_modified = 0;

    if let Some(ref user) = member.user {
        rows_modified += db.update_user(user.clone()).await?;
    }

    rows_modified += sqlx::query_file!(
        "queries/guild_members/update_twilight_partial_member.sql",
        match member.premium_since {
            Some(timestamp) => Some(OffsetDateTime::from_unix_timestamp(timestamp.as_secs())?),
            None => None,
        },
        match member.communication_disabled_until {
            Some(timestamp) => Some(OffsetDateTime::from_unix_timestamp(timestamp.as_secs())?),
            None => None,
        },
        member.deaf,
        guild_id.get() as i64,
        OffsetDateTime::from_unix_timestamp(member.joined_at.as_secs())?,
        member.avatar.map(|x|x.to_string()),
        member.flags.bits() as i64,
        member.mute,
        member.nick,
    )
    .execute(&db.pool)
    .await?.rows_affected();

    Ok(rows_modified)
}

async fn handle_member_add(db: &LuroDatabase, member: Box<MemberAdd>) -> anyhow::Result<u64> {
    let mut rows_modified = db.update_user(member.user.clone()).await?;
    rows_modified += sqlx::query_file!(
        "queries/guild_members/update_twilight_member.sql",
        match member.premium_since {
            Some(timestamp) => Some(OffsetDateTime::from_unix_timestamp(timestamp.as_secs())?),
            None => None,
        },
        match member.communication_disabled_until {
            Some(timestamp) => Some(OffsetDateTime::from_unix_timestamp(timestamp.as_secs())?),
            None => None,
        },
        member.deaf,
        member.guild_id.get() as i64,
        OffsetDateTime::from_unix_timestamp(member.joined_at.as_secs())?,
        member.avatar.map(|x|x.to_string()),
        member.flags.bits() as i64,
        member.mute,
        member.nick,
        member.pending,
    )
    .execute(&db.pool)
    .await?.rows_affected();

    Ok(rows_modified)
}

async fn handle_member_update(db: &LuroDatabase, member: Box<MemberUpdate>) -> anyhow::Result<u64> {
    let mut rows_modified = db.update_user(member.user.clone()).await?;
    rows_modified += sqlx::query_file!(
        "queries/guild_members/update_twilight_member_update.sql",
        match member.premium_since {
            Some(timestamp) => Some(OffsetDateTime::from_unix_timestamp(timestamp.as_secs())?),
            None => None,
        },
        match member.communication_disabled_until {
            Some(timestamp) => Some(OffsetDateTime::from_unix_timestamp(timestamp.as_secs())?),
            None => None,
        },
        member.deaf,
        member.guild_id.get() as i64,
        OffsetDateTime::from_unix_timestamp(member.joined_at.as_secs())?,
        member.avatar.map(|x|x.to_string()),
        member.mute,
        member.nick,
        member.pending,
    )
    .execute(&db.pool)
    .await?.rows_affected();

    Ok(rows_modified)
}
