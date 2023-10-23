mod count_guilds;
mod count_members;
mod get_guild;
mod handle_guild;
mod handle_guild_update;
mod handle_luro_guild;
mod update_guild;

#[derive(Clone)]
pub struct DatabaseGuild {
    pub accent_colour: Option<i32>,
    pub afk_timeout: i16,
    pub custom_accent_colour: Option<i32>,
    pub default_message_notifications: i16,
    pub explicit_content_filter: i16,
    pub guild_id: i64,
    pub mfa_level: i16,
    pub name: String,
    pub nsfw_level: i16,
    pub owner_id: i64,
    pub system_channel_flags: i64,
    pub verification_level: i16,
}
