use time::OffsetDateTime;
use twilight_model::{
    gateway::payload::incoming::{MemberAdd, MemberChunk, MemberRemove, MemberUpdate, UserUpdate},
    guild::{Member, PartialMember},
    id::{
        marker::{GuildMarker, UserMarker},
        Id,
    },
    user::{CurrentUser, UserFlags},
};

use crate::{
    database::sqlx::{Database, Error},
    user::User,
};

impl Database {
    pub async fn update_user(&self, user: impl Into<UserSync<'_>>) -> Result<u64, Error> {
        let rows_modified = match user.into() {
            UserSync::User(user) => luro_user(self, user).await?,
            UserSync::CurrentUser(user) => current_user(self, user).await?,
            UserSync::TwilightUser(user) => twilight_user(self, user).await?,
            UserSync::UserID(user) => user_id(self, user).await?,
            UserSync::UserUpdate(user) => user_update(self, user).await?,
            UserSync::Member(guild_id, member) => twilight_member(self, guild_id, member).await?,
            UserSync::MemberAdd(member) => handle_member_add(self, member).await?,
            UserSync::MemberChunk(member) => handle_member_chunk(self, member).await?,
            UserSync::MemberRemove(member) => handle_member_remove(self, member).await?,
            UserSync::MemberUpdate(member) => handle_member_update(self, member).await?,
            UserSync::PartialMember(guild_id, member) => handle_partial_member(self, guild_id, member).await?,
        };

        tracing::debug!("DB Member: Updated `{rows_modified}` rows!");

        Ok(rows_modified)
    }
}

async fn luro_user(db: &Database, user: &User) -> Result<u64, sqlx::Error> {
    match user {
        User::Member(_member) => todo!(),
        User::User(user) => twilight_user(db, &user.twilight_user).await,
    }
}

async fn user_flags(db: &Database, flags: Option<&UserFlags>, user_id: Id<UserMarker>) -> Result<u64, sqlx::Error> {
    sqlx::query_file!(
        "queries/user/user_update_flags.sql",
        flags.map(|x| x.bits() as i64),
        user_id.get() as i64
    )
    .execute(&db.pool)
    .await
    .map(|x| x.rows_affected())
}

async fn public_flags(db: &Database, flags: Option<&UserFlags>, user_id: Id<UserMarker>) -> Result<u64, sqlx::Error> {
    sqlx::query_file!(
        "queries/user/user_update_public_flags.sql",
        flags.map(|x| x.bits() as i64),
        user_id.get() as i64
    )
    .execute(&db.pool)
    .await
    .map(|x| x.rows_affected())
}

async fn user_id(db: &Database, user_id: Id<UserMarker>) -> Result<u64, sqlx::Error> {
    sqlx::query_file!("queries/user/user_update_twilight_user_id.sql", user_id.get() as i64)
        .execute(&db.pool)
        .await
        .map(|x| x.rows_affected())
}

async fn current_user(db: &Database, user: &CurrentUser) -> Result<u64, sqlx::Error> {
    let mut rows_modified = 0;

    if user.flags.is_some() {
        rows_modified += user_flags(db, user.flags.as_ref(), user.id).await?
    }

    if user.public_flags.is_some() {
        rows_modified += public_flags(db, user.public_flags.as_ref(), user.id).await?
    }

    rows_modified += sqlx::query_file!(
        "queries/user/user_update_current_user.sql",
        user.accent_color.map(|x| x as i32),
        user.bot,
        user.discriminator as i16,
        user.email,
        user.locale,
        user.mfa_enabled,
        user.premium_type.map(|x| u8::from(x) as i16),
        user.avatar.map(|x| x.to_string()),
        user.banner.map(|x| x.to_string()),
        user.id.get() as i64,
        user.name,
        user.verified
    )
    .execute(&db.pool)
    .await?
    .rows_affected();

    Ok(rows_modified)
}

async fn twilight_user(db: &Database, user: &twilight_model::user::User) -> Result<u64, sqlx::Error> {
    let mut rows_modified = 0;

    if user.flags.is_some() {
        rows_modified += user_flags(db, user.flags.as_ref(), user.id).await?
    }

    if user.public_flags.is_some() {
        rows_modified += public_flags(db, user.public_flags.as_ref(), user.id).await?
    }

    rows_modified += sqlx::query_file!(
        "queries/user/user_update_twilight_user.sql",
        user.accent_color.map(|x| x as i32),
        user.avatar_decoration.map(|x| x.to_string()),
        user.bot,
        user.discriminator as i16,
        user.email,
        user.global_name,
        user.locale,
        user.mfa_enabled,
        user.premium_type.map(|x| u8::from(x) as i16),
        user.avatar.map(|x| x.to_string()),
        user.banner.map(|x| x.to_string()),
        user.id.get() as i64,
        user.name,
        user.system,
        user.verified
    )
    .execute(&db.pool)
    .await?
    .rows_affected();

    Ok(rows_modified)
}

async fn user_update(db: &Database, user: &UserUpdate) -> Result<u64, sqlx::Error> {
    let mut rows_modified = 0;

    if user.flags.is_some() {
        rows_modified += user_flags(db, user.flags.as_ref(), user.id).await?
    }

    if user.public_flags.is_some() {
        rows_modified += public_flags(db, user.public_flags.as_ref(), user.id).await?
    }

    rows_modified += sqlx::query_file!(
        "queries/user/user_update_twilight_user_update.sql",
        user.accent_color.map(|x| x as i32),
        user.bot,
        user.discriminator as i16,
        user.email,
        user.locale,
        user.mfa_enabled,
        user.premium_type.map(|x| u8::from(x) as i16),
        user.avatar.map(|x| x.to_string()),
        user.banner.map(|x| x.to_string()),
        user.id.get() as i64,
        user.name,
        user.verified
    )
    .execute(&db.pool)
    .await?
    .rows_affected();

    Ok(rows_modified)
}

async fn handle_member_chunk(db: &Database, event: &MemberChunk) -> Result<u64, Error> {
    let mut rows_modified = 0;
    for member in &event.members {
        match twilight_member(db, event.guild_id, member).await {
            Ok(updated_rows) => rows_modified += updated_rows,
            Err(why) => tracing::warn!(why = ?why, "MEMBER_CHUNK: Failed to handle member"),
        }
    }

    Ok(rows_modified)
}

async fn handle_member_remove(db: &Database, member: &MemberRemove) -> Result<u64, sqlx::Error> {
    let mut rows_modified = twilight_user(db, &member.user).await?;

    rows_modified += sqlx::query_file!(
        "queries/member/member_remove.sql",
        member.guild_id.get() as i64,
        member.user.id.get() as i64
    )
    .execute(&db.pool)
    .await?
    .rows_affected();

    Ok(rows_modified)
}

async fn twilight_member(db: &Database, guild_id: Id<GuildMarker>, member: &Member) -> Result<u64, Error> {
    let mut rows_modified = twilight_user(db, &member.user).await?;
    rows_modified += db.delete_member_roles(guild_id, member.user.id).await?;

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
        OffsetDateTime::from_unix_timestamp(member.joined_at.unwrap().as_secs())?,
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
        match db.update_role((guild_id, *role)).await {
            Ok(ok) => rows_modified += ok,
            Err(why) => tracing::warn!(why = ?why, "handle_member - Failed to sync role"),
        }
        match db.update_member_role(guild_id, *role, member.user.id).await {
            Ok(ok) => rows_modified += ok,
            Err(why) => tracing::warn!(why = ?why, "handle_member - Failed to sync user role"),
        }
    }

    Ok(rows_modified)
}

async fn handle_partial_member(db: &Database, guild_id: Id<GuildMarker>, member: &PartialMember) -> Result<u64, Error> {
    let mut rows_modified = 0;

    if let Some(user) = &member.user {
        rows_modified += twilight_user(db, user).await?;
        rows_modified += db.delete_member_roles(guild_id, user.id).await?;

        for role in &member.roles {
            match db.update_member_role(guild_id, *role, user.id).await {
                Ok(ok) => rows_modified += ok,
                Err(why) => tracing::warn!(why = ?why, "handle_partial_member - Failed to sync user role"),
            }
        }
    }

    for role in &member.roles {
        match db.update_role((guild_id, *role)).await {
            Ok(ok) => rows_modified += ok,
            Err(why) => tracing::warn!(why = ?why, "handle_partial_member - Failed to sync role"),
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
        OffsetDateTime::from_unix_timestamp(member.joined_at.unwrap().as_secs())?,
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

async fn handle_member_add(db: &Database, member: &MemberAdd) -> Result<u64, Error> {
    let mut rows_modified = twilight_user(db, &member.user).await?;
    rows_modified += db.delete_member_roles(member.guild_id, member.user.id).await?;

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
        OffsetDateTime::from_unix_timestamp(member.joined_at.unwrap().as_secs())?,
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
            Err(why) => tracing::warn!(why = ?why, "handle_member_add - Failed to sync role"),
        }
        match db.update_member_role(member.guild_id, *role, member.user.id).await {
            Ok(ok) => rows_modified += ok,
            Err(why) => tracing::warn!(why = ?why, "handle_member_add - Failed to sync user role"),
        }
    }

    Ok(rows_modified)
}

async fn handle_member_update(db: &Database, member: &MemberUpdate) -> Result<u64, Error> {
    let mut rows_modified = twilight_user(db, &member.user).await?;
    rows_modified += db.delete_member_roles(member.guild_id, member.user.id).await?;

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
        OffsetDateTime::from_unix_timestamp(member.joined_at.unwrap().as_secs())?,
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
            Err(why) => tracing::warn!(why = ?why, "handle_member_update - Failed to sync role"),
        }
        match db.update_member_role(member.guild_id, *role, member.user.id).await {
            Ok(ok) => rows_modified += ok,
            Err(why) => tracing::warn!(why = ?why, "handle_member_update - Failed to sync user role"),
        }
    }

    Ok(rows_modified)
}

pub enum UserSync<'a> {
    User(&'a User),
    TwilightUser(&'a twilight_model::user::User),
    UserID(Id<UserMarker>),
    UserUpdate(&'a UserUpdate),
    CurrentUser(&'a CurrentUser),
    Member(Id<GuildMarker>, &'a Member),
    MemberAdd(&'a MemberAdd),
    MemberChunk(&'a MemberChunk),
    MemberRemove(&'a MemberRemove),
    MemberUpdate(&'a MemberUpdate),
    PartialMember(Id<GuildMarker>, &'a PartialMember),
}

impl<'a> From<&'a CurrentUser> for UserSync<'a> {
    fn from(user: &'a CurrentUser) -> Self {
        Self::CurrentUser(user)
    }
}

impl<'a> From<&'a UserUpdate> for UserSync<'a> {
    fn from(user: &'a UserUpdate) -> Self {
        Self::UserUpdate(user)
    }
}

impl<'a> From<&'a User> for UserSync<'a> {
    fn from(user: &'a User) -> Self {
        Self::User(user)
    }
}

impl<'a> From<&'a twilight_model::user::User> for UserSync<'a> {
    fn from(user: &'a twilight_model::user::User) -> Self {
        Self::TwilightUser(user)
    }
}

impl<'a> From<&'a twilight_model::guild::Member> for UserSync<'a> {
    fn from(member: &'a twilight_model::guild::Member) -> Self {
        Self::TwilightUser(&member.user)
    }
}

impl<'a> From<Id<UserMarker>> for UserSync<'a> {
    fn from(user: Id<UserMarker>) -> Self {
        Self::UserID(user)
    }
}

impl<'a> From<(Id<GuildMarker>, &'a Member)> for UserSync<'a> {
    fn from(data: (Id<GuildMarker>, &'a Member)) -> Self {
        Self::Member(data.0, data.1)
    }
}

impl<'a> From<(Id<GuildMarker>, &'a PartialMember)> for UserSync<'a> {
    fn from(data: (Id<GuildMarker>, &'a PartialMember)) -> Self {
        Self::PartialMember(data.0, data.1)
    }
}

impl<'a> From<&'a MemberAdd> for UserSync<'a> {
    fn from(data: &'a MemberAdd) -> Self {
        Self::MemberAdd(data)
    }
}

impl<'a> From<&'a MemberChunk> for UserSync<'a> {
    fn from(data: &'a MemberChunk) -> Self {
        Self::MemberChunk(data)
    }
}

impl<'a> From<&'a MemberRemove> for UserSync<'a> {
    fn from(data: &'a MemberRemove) -> Self {
        Self::MemberRemove(data)
    }
}

impl<'a> From<&'a MemberUpdate> for UserSync<'a> {
    fn from(data: &'a MemberUpdate) -> Self {
        Self::MemberUpdate(data)
    }
}
