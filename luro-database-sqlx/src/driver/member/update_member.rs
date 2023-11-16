use luro_model::sync::MemberSync;
use time::OffsetDateTime;
use tracing::{debug, warn};
use twilight_model::{
    gateway::payload::incoming::{MemberAdd, MemberChunk, MemberRemove, MemberUpdate},
    guild::{Member, PartialMember},
    id::{
        marker::{GuildMarker, RoleMarker, UserMarker},
        Id,
    },
};

use crate::SQLxDriver;

impl SQLxDriver {
    /// Updates a supported member type. Returns the total number of rows modified in the database.
    pub async fn update_member(&self, member: impl Into<MemberSync<'_>>) -> anyhow::Result<u64> {
        let rows_modified = match member.into() {
            MemberSync::Member(guild_id, member) => handle_member(self, guild_id, member).await?,
            MemberSync::MemberAdd(member) => handle_member_add(self, member).await?,
            MemberSync::MemberChunk(member) => handle_member_chunk(self, member).await?,
            MemberSync::MemberRemove(member) => handle_member_remove(self, member).await?,
            MemberSync::MemberUpdate(member) => handle_member_update(self, member).await?,
            MemberSync::PartialMember(guild_id, member) => handle_partial_member(self, guild_id, member).await?,
        };

        debug!("DB Member: Updated `{rows_modified}` rows!");

        Ok(rows_modified)
    }

    pub async fn update_guild_member_role(
        &self,
        guild_id: Id<GuildMarker>,
        role_id: Id<RoleMarker>,
        user_id: Id<UserMarker>,
    ) -> Result<u64, sqlx::Error> {
        sqlx::query_file!(
            "queries/member/member_update_role.sql",
            guild_id.get() as i64,
            role_id.get() as i64,
            user_id.get() as i64,
        )
        .execute(&self.pool)
        .await
        .map(|x| x.rows_affected())
    }
}

async fn handle_member_chunk(db: &SQLxDriver, event: &MemberChunk) -> anyhow::Result<u64> {
    let mut rows_modified = 0;
    for member in &event.members {
        rows_modified += db.update_user(member).await?;
        for role in &member.roles {
            match db.update_role((event.guild_id, *role)).await {
                Ok(ok) => rows_modified += ok,
                Err(why) => warn!(why = ?why, "handle_member_add - Failed to sync role"),
            }
            match db.update_guild_member_role(event.guild_id, *role, member.user.id).await {
                Ok(ok) => rows_modified += ok,
                Err(why) => warn!(why = ?why, "handle_member_add - Failed to sync user role"),
            }
        }

        rows_modified += sqlx::query_file!(
            "queries/member/member_update_twilight_member.sql",
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
            member.avatar.map(|x| x.to_string()),
            member.flags.bits() as i64,
            member.mute,
            member.nick,
            member.pending,
            member.user.id.get() as i64,
        )
        .execute(&db.pool)
        .await?
        .rows_affected()
    }

    Ok(rows_modified)
}

async fn handle_member_remove(db: &SQLxDriver, member: &MemberRemove) -> anyhow::Result<u64> {
    let mut rows_updated = db.update_user(&member.user).await?;
    rows_updated += sqlx::query_file!(
        "queries/member/member_remove.sql",
        member.guild_id.get() as i64,
        member.user.id.get() as i64
    )
    .execute(&db.pool)
    .await?
    .rows_affected();
    Ok(rows_updated)
}

async fn handle_member(db: &SQLxDriver, guild_id: Id<GuildMarker>, member: &Member) -> anyhow::Result<u64> {
    debug!("handle_member - Trying to handle updating roles");
    let mut rows_modified = db.update_user(&member.user).await?;

    for role in &member.roles {
        match db.update_role((guild_id, *role)).await {
            Ok(ok) => rows_modified += ok,
            Err(why) => warn!(why = ?why, "handle_member_add - Failed to sync role"),
        }
        match db.update_guild_member_role(guild_id, *role, member.user.id).await {
            Ok(ok) => rows_modified += ok,
            Err(why) => warn!(why = ?why, "handle_member_add - Failed to sync user role"),
        }
    }

    debug!("handle_member - Trying to handle updating member");
    rows_modified += sqlx::query_file!(
        "queries/member/member_update_twilight_member.sql",
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
        member.avatar.map(|x| x.to_string()),
        member.flags.bits() as i64,
        member.mute,
        member.nick,
        member.pending,
        member.user.id.get() as i64,
    )
    .execute(&db.pool)
    .await?
    .rows_affected();

    db.clear_member_roles(guild_id, member.user.id).await?;
    for role in &member.roles {
        debug!("handle_member - Trying to handle updating roles");
        db.update_role((guild_id, *role)).await?;
        debug!("handle_member - Trying to handle updating member roles");
        db.update_guild_member_role(guild_id, *role, member.user.id).await?;
    }

    Ok(rows_modified)
}

async fn handle_partial_member(db: &SQLxDriver, guild_id: Id<GuildMarker>, member: &PartialMember) -> anyhow::Result<u64> {
    let mut rows_modified = 0;

    if let Some(ref user) = member.user {
        rows_modified += db.update_user(user).await?;
        for role in &member.roles {
            match db.update_role((guild_id, *role)).await {
                Ok(ok) => rows_modified += ok,
                Err(why) => warn!(why = ?why, "handle_member_add - Failed to sync role"),
            }
            match db.update_guild_member_role(guild_id, *role, user.id).await {
                Ok(ok) => rows_modified += ok,
                Err(why) => warn!(why = ?why, "handle_member_add - Failed to sync user role"),
            }
        }
    }

    rows_modified += sqlx::query_file!(
        "queries/member/member_update_twilight_partial_member.sql",
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
        member.avatar.map(|x| x.to_string()),
        member.flags.bits() as i64,
        member.mute,
        member.nick,
        member.user.as_ref().map(|x| x.id.get() as i64).unwrap()
    )
    .execute(&db.pool)
    .await?
    .rows_affected();

    Ok(rows_modified)
}

async fn handle_member_add(db: &SQLxDriver, member: &MemberAdd) -> anyhow::Result<u64> {
    let mut rows_modified = db.update_user(&member.user).await?;
    rows_modified += sqlx::query_file!(
        "queries/member/member_update_twilight_member.sql",
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
        member.avatar.map(|x| x.to_string()),
        member.flags.bits() as i64,
        member.mute,
        member.nick,
        member.pending,
        member.user.id.get() as i64,
    )
    .execute(&db.pool)
    .await?
    .rows_affected();

    for role in &member.roles {
        match db.update_role((member.guild_id, *role)).await {
            Ok(ok) => rows_modified += ok,
            Err(why) => warn!(why = ?why, "handle_member_add - Failed to sync role"),
        }
        match db.update_guild_member_role(member.guild_id, *role, member.user.id).await {
            Ok(ok) => rows_modified += ok,
            Err(why) => warn!(why = ?why, "handle_member_add - Failed to sync user role"),
        }
    }

    Ok(rows_modified)
}

async fn handle_member_update(db: &SQLxDriver, member: &MemberUpdate) -> anyhow::Result<u64> {
    let mut rows_modified = db.update_user(&member.user).await?;

    rows_modified += sqlx::query_file!(
        "queries/member/member_update_twilight_member_update.sql",
        match member.premium_since {
            Some(timestamp) => Some(OffsetDateTime::from_unix_timestamp(timestamp.as_secs())?),
            None => None,
        },
        match member.communication_disabled_until {
            Some(timestamp) => Some(OffsetDateTime::from_unix_timestamp(timestamp.as_secs())?),
            None => None,
        },
        member.guild_id.get() as i64,
        OffsetDateTime::from_unix_timestamp(member.joined_at.as_secs())?,
        member.avatar.map(|x| x.to_string()),
        member.nick,
        member.pending,
        member.user.id.get() as i64,
    )
    .execute(&db.pool)
    .await?
    .rows_affected();

    for role in &member.roles {
        match db.update_role((member.guild_id, *role)).await {
            Ok(ok) => rows_modified += ok,
            Err(why) => warn!(why = ?why, "handle_member_add - Failed to sync role"),
        }
        match db.update_guild_member_role(member.guild_id, *role, member.user.id).await {
            Ok(ok) => rows_modified += ok,
            Err(why) => warn!(why = ?why, "handle_member_add - Failed to sync user role"),
        }
    }

    Ok(rows_modified)
}
