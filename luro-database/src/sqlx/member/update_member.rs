use sqlx::types::Json;
use time::OffsetDateTime;
use tracing::error;
use twilight_model::gateway::payload::incoming::{MemberRemove, MemberUpdate};
use twilight_model::guild::PartialMember;
use twilight_model::util::ImageHash;
use twilight_model::{gateway::payload::incoming::MemberAdd, guild::Member};

use crate::{DbMember, DbMemberType, LuroDatabase};

impl LuroDatabase {
    pub async fn update_member(&self, member: impl Into<DbMemberType>) -> Result<Option<DbMember>, sqlx::Error> {
        let member = member.into();

        match member {
            DbMemberType::Member(guild_id, member) => handle_member(self, guild_id.get() as i64, member).await,
            DbMemberType::MemberAdd(member) => handle_member_add(self, member).await,
            DbMemberType::MemberChunk(_) => todo!(),
            DbMemberType::MemberRemove(member) => handle_member_remove(self, member).await,
            DbMemberType::MemberUpdate(member) => handle_member_update(self, member).await,
            DbMemberType::PartialMember(guild_id, member) => handle_partial_member(self, guild_id.get() as i64, member).await,
        }
    }
}

async fn handle_partial_member(db: &LuroDatabase, guild_id: i64, member: PartialMember) -> Result<Option<DbMember>, sqlx::Error> {
    let user = match member.user {
        Some(user) => user,
        None => return Ok(None),
    };

    if let Err(why) = db.update_user(user.clone()).await {
        error!("Failed to update user: {why}")
    }

    sqlx::query_as!(
        DbMember,
        "INSERT INTO guild_members (
            user_id,
            guild_id,
            avatar,
            boosting_since,
            communication_disabled_until,
            deafened,
            flags,
            muted,
            nickname
        ) VALUES
            ($1, $2, $3, $4, $5, $6, $7, $8, $9)
        ON CONFLICT
            (user_id, guild_id)
        DO UPDATE SET
            avatar = $3,
            boosting_since = $4,
            communication_disabled_until = $5,
            deafened = $6,
            flags = $7,
            muted = $8,
            nickname = $9
        RETURNING
            user_id,
            guild_id,
            avatar as \"avatar: Json<ImageHash>\",
            boosting_since,
            communication_disabled_until,
            deafened,
            flags,
            muted,
            nickname,
            pending
        ",
        user.id.get() as i64,
        guild_id,
        member.avatar.map(Json) as _,
        member
            .premium_since
            .map(|x| OffsetDateTime::from_unix_timestamp(x.as_secs()).unwrap()),
        member
            .communication_disabled_until
            .map(|x| OffsetDateTime::from_unix_timestamp(x.as_secs()).unwrap()),
        member.deaf,
        member.flags.bits() as i32,
        member.mute,
        member.nick,
    )
    .fetch_optional(&db.pool)
    .await
}

async fn handle_member_remove(db: &LuroDatabase, member: MemberRemove) -> Result<Option<DbMember>, sqlx::Error> {
    if let Err(why) = db.update_user(member.user.clone()).await {
        error!("Failed to update user: {why}")
    }
    sqlx::query_as!(
        DbMember,
        "INSERT INTO guild_members (
            user_id,
            guild_id,
            removed
        ) VALUES (
            $1, $2, $3
        ) ON CONFLICT (
            user_id, guild_id
        ) DO UPDATE SET
            user_id = $1,
            guild_id = $2,
            removed = $3
        RETURNING
            user_id,
            guild_id,
            avatar as \"avatar: Json<ImageHash>\",
            boosting_since,
            communication_disabled_until,
            deafened,
            flags,
            muted,
            nickname,
            pending
        ",
        member.user.id.get() as i64,
        member.guild_id.get() as i64,
        true
    )
    .fetch_optional(&db.pool)
    .await
}

async fn handle_member(db: &LuroDatabase, guild_id: i64, member: Member) -> Result<Option<DbMember>, sqlx::Error> {
    if let Err(why) = db.update_user(member.user.clone()).await {
        error!("Failed to update user: {why}")
    }
    sqlx::query_as!(
        DbMember,
        "INSERT INTO guild_members (
            user_id,
            guild_id,
            avatar,
            boosting_since,
            communication_disabled_until,
            deafened,
            flags,
            muted,
            nickname,
            pending
        ) VALUES
            ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
        ON CONFLICT
            (user_id, guild_id)
        DO UPDATE SET
            avatar = $3,
            boosting_since = $4,
            communication_disabled_until = $5,
            deafened = $6,
            flags = $7,
            muted = $8,
            nickname = $9,
            pending = $10
        RETURNING
            user_id,
            guild_id,
            avatar as \"avatar: Json<ImageHash>\",
            boosting_since,
            communication_disabled_until,
            deafened,
            flags,
            muted,
            nickname,
            pending
        ",
        member.user.id.get() as i64,
        guild_id,
        member.avatar.map(Json) as _,
        member
            .premium_since
            .map(|x| OffsetDateTime::from_unix_timestamp(x.as_secs()).unwrap()),
        member
            .communication_disabled_until
            .map(|x| OffsetDateTime::from_unix_timestamp(x.as_secs()).unwrap()),
        member.deaf,
        member.flags.bits() as i32,
        member.mute,
        member.nick,
        member.pending,
        // member.roles.iter().map(|x| x.get() as i64).collect::<Vec<_>>()
    )
    .fetch_optional(&db.pool)
    .await
}

async fn handle_member_add(db: &LuroDatabase, member: Box<MemberAdd>) -> Result<Option<DbMember>, sqlx::Error> {
    if let Err(why) = db.update_user(member.user.clone()).await {
        error!("Failed to update user: {why}")
    }
    sqlx::query_as!(
        DbMember,
        "INSERT INTO guild_members (
            user_id,
            guild_id,
            avatar,
            boosting_since,
            communication_disabled_until,
            deafened,
            flags,
            muted,
            nickname,
            pending
        ) VALUES
            ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
        ON CONFLICT
            (user_id, guild_id)
        DO UPDATE SET
            avatar = $3,
            boosting_since = $4,
            communication_disabled_until = $5,
            deafened = $6,
            flags = $7,
            muted = $8,
            nickname = $9,
            pending = $10
        RETURNING
            user_id,
            guild_id,
            avatar as \"avatar: Json<ImageHash>\",
            boosting_since,
            communication_disabled_until,
            deafened,
            flags,
            muted,
            nickname,
            pending
        ",
        member.user.id.get() as i64,
        member.guild_id.get() as i64,
        member.avatar.map(Json) as _,
        member
            .premium_since
            .map(|x| OffsetDateTime::from_unix_timestamp(x.as_secs()).unwrap()),
        member
            .communication_disabled_until
            .map(|x| OffsetDateTime::from_unix_timestamp(x.as_secs()).unwrap()),
        member.deaf,
        member.flags.bits() as i32,
        member.mute,
        member.nick,
        member.pending,
        // member.roles.iter().map(|x| x.get() as i64).collect::<Vec<_>>()
    )
    .fetch_optional(&db.pool)
    .await
}

async fn handle_member_update(db: &LuroDatabase, member: Box<MemberUpdate>) -> Result<Option<DbMember>, sqlx::Error> {
    if let Err(why) = db.update_user(member.user.clone()).await {
        error!("Failed to update user: {why}")
    }
    sqlx::query_as!(
        DbMember,
        "INSERT INTO guild_members (
            user_id,
            guild_id,
            avatar,
            boosting_since,
            communication_disabled_until,
            pending
        ) VALUES
            ($1, $2, $3, $4, $5, $6)
        ON CONFLICT
            (user_id, guild_id)
        DO UPDATE SET
            avatar = $3,
            boosting_since = $4,
            communication_disabled_until = $5,
            pending = $6
        RETURNING
            user_id,
            guild_id,
            avatar as \"avatar: Json<ImageHash>\",
            boosting_since,
            communication_disabled_until,
            deafened,
            flags,
            muted,
            nickname,
            pending
        ",
        member.user.id.get() as i64,
        member.guild_id.get() as i64,
        member.avatar.map(Json) as _,
        member
            .premium_since
            .map(|x| OffsetDateTime::from_unix_timestamp(x.as_secs()).unwrap()),
        member
            .communication_disabled_until
            .map(|x| OffsetDateTime::from_unix_timestamp(x.as_secs()).unwrap()),
        member.pending,
        // member.roles.iter().map(|x| x.get() as i64).collect::<Vec<_>>()
    )
    .fetch_optional(&db.pool)
    .await
}