use sqlx::postgres::PgQueryResult;
use time::OffsetDateTime;
use twilight_model::{
    gateway::payload::incoming::{GuildCreate, GuildUpdate},
    guild::{Guild, UnavailableGuild},
};

use crate::database::sqlx::{Database, Error};

impl Database {
    pub async fn update_guild(&self, guild: impl Into<GuildSync<'_>>) -> Result<u64, Error> {
        Ok(match guild.into() {
            GuildSync::Guild(guild) => handle_guild(self, guild).await?,
            GuildSync::GuildUpdate(guild) => handle_guild_update(self, guild).await?.rows_affected(),
            GuildSync::GuildCreate(guild) => handle_guild_create(self, guild).await?,
            GuildSync::GuildUnavailable(guild) => unavailable_guild(self, guild).await?.rows_affected(),
        })
    }
}

async fn unavailable_guild(db: &Database, guild: &UnavailableGuild) -> Result<PgQueryResult, sqlx::Error> {
    sqlx::query_file!("queries/update/guild_unavailable.sql", guild.id.get() as i64, guild.unavailable,)
        .execute(&db.pool)
        .await
}

async fn handle_guild(db: &Database, guild: &Guild) -> Result<u64, Error> {
    Ok(sqlx::query_file!(
        "queries/guild/guild_update.sql",
        guild.afk_channel_id.map(|x| x.get() as i64),
        guild.afk_timeout.get() as i16,
        guild.application_id.map(|x| x.get() as i64),
        guild.approximate_presence_count.map(|x| x as i64),
        guild.banner.map(|x| x.to_string()),
        u8::from(guild.default_message_notifications) as i16,
        guild.discovery_splash.map(|x| x.to_string()),
        u8::from(guild.explicit_content_filter) as i16,
        guild.id.get() as i64,
        guild.icon.map(|x| x.to_string()),
        match guild.joined_at {
            Some(time) => Some(OffsetDateTime::from_unix_timestamp(time.as_secs())?),
            None => None,
        },
        guild.large,
        guild.max_members.map(|x| x as i64),
        guild.max_presences.map(|x| x as i64),
        guild.max_video_channel_users.map(|x| x as i64),
        u8::from(guild.mfa_level) as i16,
        guild.name,
        u8::from(guild.nsfw_level) as i16,
        guild.owner,
        guild.owner_id.get() as i64,
        guild.permissions.map(|x| x.bits() as i64),
        guild.preferred_locale,
        guild.premium_progress_bar_enabled,
        guild.premium_subscription_count.map(|x| x as i64),
        u8::from(guild.premium_tier) as i16,
        guild.public_updates_channel_id.map(|x| x.get() as i64),
        guild.rules_channel_id.map(|x| x.get() as i64),
        guild.safety_alerts_channel_id.map(|x| x.get() as i64),
        guild.splash.map(|x| x.to_string()),
        guild.system_channel_flags.bits() as i64,
        guild.system_channel_id.map(|x| x.get() as i64),
        guild.unavailable,
        guild.vanity_url_code,
        u8::from(guild.verification_level) as i16,
        guild.widget_channel_id.map(|x| x.get() as i64),
        guild.widget_enabled
    )
    .execute(&db.pool)
    .await?
    .rows_affected())
}

async fn handle_guild_update(db: &Database, guild: &GuildUpdate) -> Result<PgQueryResult, sqlx::Error> {
    sqlx::query_file!(
        "queries/guild/guild_update_twilight_update.sql",
        guild.afk_channel_id.map(|x| x.get() as i64),
        guild.afk_timeout.get() as i16,
        guild.application_id.map(|x| x.get() as i64),
        guild.banner.map(|x| x.to_string()),
        u8::from(guild.default_message_notifications) as i16,
        guild.discovery_splash.map(|x| x.to_string()),
        u8::from(guild.explicit_content_filter) as i16,
        guild.id.get() as i64,
        guild.icon.map(|x| x.to_string()),
        guild.max_members.map(|x| x as i64),
        guild.max_presences.map(|x| x as i64),
        u8::from(guild.mfa_level) as i16,
        guild.name,
        u8::from(guild.nsfw_level) as i16,
        guild.owner,
        guild.owner_id.get() as i64,
        guild.permissions.map(|x| x.bits() as i64),
        guild.preferred_locale,
        guild.premium_progress_bar_enabled,
        guild.premium_subscription_count.map(|x| x as i64),
        u8::from(guild.premium_tier) as i16,
        guild.public_updates_channel_id.map(|x| x.get() as i64),
        guild.rules_channel_id.map(|x| x.get() as i64),
        guild.splash.map(|x| x.to_string()),
        guild.system_channel_flags.bits() as i64,
        guild.system_channel_id.map(|x| x.get() as i64),
        guild.vanity_url_code,
        u8::from(guild.verification_level) as i16,
        guild.widget_channel_id.map(|x| x.get() as i64),
        guild.widget_enabled
    )
    .execute(&db.pool)
    .await
}

async fn handle_guild_create(db: &Database, guild: &GuildCreate) -> Result<u64, Error> {
    let mut updated_rows = 0;
    for channel in &guild.channels {
        updated_rows += db.update_channel(channel.id).await?;
    }

    updated_rows += sqlx::query_file!(
        "queries/guild/guild_update.sql",
        guild.afk_channel_id.map(|x| x.get() as i64),
        guild.afk_timeout.get() as i16,
        guild.application_id.map(|x| x.get() as i64),
        guild.approximate_presence_count.map(|x| x as i64),
        guild.banner.map(|x| x.to_string()),
        u8::from(guild.default_message_notifications) as i16,
        guild.discovery_splash.map(|x| x.to_string()),
        u8::from(guild.explicit_content_filter) as i16,
        guild.id.get() as i64,
        guild.icon.map(|x| x.to_string()),
        match guild.joined_at {
            Some(time) => Some(OffsetDateTime::from_unix_timestamp(time.as_secs())?),
            None => None,
        },
        guild.large,
        guild.max_members.map(|x| x as i64),
        guild.max_presences.map(|x| x as i64),
        guild.max_video_channel_users.map(|x| x as i64),
        u8::from(guild.mfa_level) as i16,
        guild.name,
        u8::from(guild.nsfw_level) as i16,
        guild.owner,
        guild.owner_id.get() as i64,
        guild.permissions.map(|x| x.bits() as i64),
        guild.preferred_locale,
        guild.premium_progress_bar_enabled,
        guild.premium_subscription_count.map(|x| x as i64),
        u8::from(guild.premium_tier) as i16,
        guild.public_updates_channel_id.map(|x| x.get() as i64),
        guild.rules_channel_id.map(|x| x.get() as i64),
        guild.safety_alerts_channel_id.map(|x| x.get() as i64),
        guild.splash.map(|x| x.to_string()),
        guild.system_channel_flags.bits() as i64,
        guild.system_channel_id.map(|x| x.get() as i64),
        guild.unavailable,
        guild.vanity_url_code,
        u8::from(guild.verification_level) as i16,
        guild.widget_channel_id.map(|x| x.get() as i64),
        guild.widget_enabled
    )
    .execute(&db.pool)
    .await?
    .rows_affected();

    for channel in &guild.channels {
        updated_rows += db.update_channel(channel).await?;
    }

    for role in &guild.roles {
        updated_rows += db.update_role((guild.id, role)).await?;
    }

    for member in &guild.members {
        match db.update_user((guild.id, member)).await {
            Ok(rows) => updated_rows += rows,
            Err(why) => tracing::warn!(why = ?why, "guild_create - Failed to sync member {}", member.user.id),
        }
    }

    Ok(updated_rows)
}

pub enum GuildSync<'a> {
    Guild(&'a Guild),
    GuildUpdate(&'a GuildUpdate),
    GuildCreate(&'a GuildCreate),
    GuildUnavailable(&'a UnavailableGuild),
}

impl<'a> From<&'a UnavailableGuild> for GuildSync<'a> {
    fn from(guild: &'a UnavailableGuild) -> Self {
        Self::GuildUnavailable(guild)
    }
}

impl<'a> From<&'a GuildUpdate> for GuildSync<'a> {
    fn from(guild: &'a GuildUpdate) -> Self {
        Self::GuildUpdate(guild)
    }
}

impl<'a> From<&'a GuildCreate> for GuildSync<'a> {
    fn from(guild: &'a GuildCreate) -> Self {
        Self::GuildCreate(guild)
    }
}

impl<'a> From<&'a Guild> for GuildSync<'a> {
    fn from(guild: &'a Guild) -> Self {
        Self::Guild(guild)
    }
}
