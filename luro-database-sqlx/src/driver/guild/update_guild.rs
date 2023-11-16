use luro_model::sync::GuildSync;
use sqlx::postgres::PgQueryResult;
use time::OffsetDateTime;
use twilight_model::{gateway::payload::incoming::{GuildUpdate, GuildCreate}, guild::Guild};

use crate::SQLxDriver;

impl SQLxDriver {
    pub async fn update_guild(&self, guild: impl Into<GuildSync<'_>>) -> anyhow::Result<u64> {
        Ok(match guild.into() {
            GuildSync::Guild(guild) => handle_guild(self, guild).await?.rows_affected(),
            GuildSync::GuildUpdate(guild) => handle_guild_update(self, guild).await?.rows_affected(),
            GuildSync::GuildCreate(guild) => handle_guild_create(self, guild).await?.rows_affected(),
        })
    }
}

async fn handle_guild(db: &SQLxDriver, guild: &Guild) -> anyhow::Result<PgQueryResult> {
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
    .await?)
}

async fn handle_guild_update(db: &SQLxDriver, guild: &GuildUpdate) -> Result<PgQueryResult, sqlx::Error> {
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

async fn handle_guild_create(db: &SQLxDriver, guild: &GuildCreate) -> anyhow::Result<PgQueryResult> {
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
    .await?)
}