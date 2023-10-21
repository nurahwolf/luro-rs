use twilight_model::{
    gateway::payload::incoming::GuildUpdate,
    guild::{AfkTimeout, Guild},
};

use crate::LuroGuild;

mod count_guilds;
mod count_members;
mod get_guild;
mod handle_guild;
mod handle_guild_update;
mod handle_luro_guild;
mod update_guild;
pub enum DatabaseGuildType {
    Guild(Guild),
    GuildUpdate(Box<GuildUpdate>),
    LuroGuild(LuroGuild),
}

#[derive(Clone)]
pub struct DatabaseGuild {
    pub accent_colour: Option<i32>,
    pub custom_accent_colour: Option<i32>,
    pub guild_id: i64,
    pub name: String,
    pub owner_id: i64,
    pub afk_timeout: i16,
}

impl From<DatabaseGuild> for LuroGuild {
    fn from(db_guild: DatabaseGuild) -> Self {
        Self {
            custom_accent_colour: db_guild.custom_accent_colour.map(|x| x as u32),
            accent_colour: db_guild.accent_colour.map(|x| x as u32),
            guild_id: db_guild.guild_id,
            name: db_guild.name,
            owner_id: db_guild.owner_id,
            afk_channel_id: Default::default(),
            afk_timeout: AfkTimeout::from(db_guild.afk_timeout as u16),
            application_id: Default::default(),
            approximate_member_count: Default::default(),
            approximate_presence_count: Default::default(),
            banner: Default::default(),
            catchall_log_channel: Default::default(),
            channels: Default::default(),
            default_message_notifications: Default::default(),
            description: Default::default(),
            discovery_splash: Default::default(),
            emojis: Default::default(),
            explicit_content_filter: Default::default(),
            everyone_role: Default::default(),
            features: Default::default(),
            icon: Default::default(),
            joined_at: Default::default(),
            large: Default::default(),
            max_members: Default::default(),
            max_presences: Default::default(),
            max_video_channel_users: Default::default(),
            member_count: Default::default(),
            members: Default::default(),
            message_events_log_channel: Default::default(),
            mfa_level: Default::default(),
            moderator_actions_log_channel: Default::default(),
            nsfw_level: Default::default(),
            owner: Default::default(),
            permissions: Default::default(),
            preferred_locale: Default::default(),
            premium_progress_bar_enabled: Default::default(),
            premium_subscription_count: Default::default(),
            premium_tier: Default::default(),
            presences: Default::default(),
            public_updates_channel_id: Default::default(),
            rules_channel_id: Default::default(),
            safety_alerts_channel_id: Default::default(),
            splash: Default::default(),
            stage_instances: Default::default(),
            stickers: Default::default(),
            system_channel_flags: Default::default(),
            system_channel_id: Default::default(),
            thread_events_log_channel: Default::default(),
            threads: Default::default(),
            unavailable: Default::default(),
            vanity_url_code: Default::default(),
            verification_level: Default::default(),
            voice_states: Default::default(),
            widget_channel_id: Default::default(),
            widget_enabled: Default::default(),
        }
    }
}
