#![feature(let_chains)]

use std::sync::Arc;

use itertools::Itertools;
use luro_core::Data;
use poise::serenity_prelude::{Cache, ChannelId, Colour, Context, Guild, GuildChannel, GuildId, Role, RoleId};
use tracing::{debug, error, info, log::warn};

/// Get the guild accent colour. If no guild is specified, or we fail to get the highest role, fall back to our defined accent colour
pub fn guild_accent_colour(accent: [u8; 3], guild: Option<Guild>) -> Colour {
    if let Some(guild) = guild {
        if let Some(highest_role) = sort_roles(&guild).first() && highest_role.1.colour.0 != 0 {
            return highest_role.1.colour;
        };
    };

    accent_colour(accent)
}

/// Instead of getting a guild accent colour like in [guild_accent_colour], this function just returns the one from the config, or passed through as a RGB array
pub fn accent_colour(accent: [u8; 3]) -> Colour {
    Colour::from_rgb(accent[0], accent[1], accent[2])
}

pub fn sort_roles(guild: &Guild) -> Vec<(&RoleId, &Role)> {
    guild.roles.iter().sorted_by_key(|&(_, r)| -r.position).collect::<Vec<_>>()
}

/// Converts integers to human-readable integers separated by
/// commas, e.g. "1000000" displays as "1,000,000" when fed through
/// this function.
pub fn format_int(int: u64) -> String {
    let mut string = String::new();
    for (idx, val) in int.to_string().chars().rev().enumerate() {
        if idx != 0 && idx % 3 == 0 {
            string.insert(0, ',');
        }
        string.insert(0, val);
    }
    string
}

/// Check to see if the guild is NSFW or not
pub fn nsfw_check(cache: Option<&Arc<Cache>>, channel_id: ChannelId) -> bool {
    match cache {
        Some(cache) => match cache.channel(channel_id) {
            Some(channel) => channel.is_nsfw(),
            None => {
                warn!("Failed to find the channel in cache");
                false
            }
        },
        None => {
            warn!("Failed to resolve the cache");
            false
        }
    }
}

/// Get the Discord event log channel, if it is defined
pub async fn discod_event_log_channel_defined(guild_id: &GuildId, user_data: &Data, ctx: &Context) -> Option<GuildChannel> {
    // Check to see if we have settings for this guild
    match user_data.guild_settings.read().await.guilds.get(&guild_id.to_string()) {
        Some(guild_settings) => match guild_settings.discord_events_log_channel {
            Some(alert_channel) => return alert_channel_defined(guild_id, ctx, alert_channel).await,
            None => {
                debug!("Event Listener: Guild settings defined, but there is no alert channel configured");
                return None;
            }
        },
        None => debug!("Event Listener: No guild settings are available for this guild")
    }
    None
}

/// Get the Moderator action log channel, if it is defined
pub async fn moderator_actions_log_channel_defined(
    guild_id: &GuildId,
    user_data: &Data,
    ctx: &Context
) -> Option<GuildChannel> {
    // Check to see if we have settings for this guild
    match user_data.guild_settings.read().await.guilds.get(&guild_id.to_string()) {
        Some(guild_settings) => match guild_settings.moderator_actions_log_channel {
            Some(alert_channel) => return alert_channel_defined(guild_id, ctx, alert_channel).await,
            None => {
                debug!("Event Listener: Guild settings defined, but there is no alert channel configured");
                return None;
            }
        },
        None => debug!("Event Listener: No guild settings are available for this guild")
    }
    None
}

/// If an alert channel is defined in this guild, this function returns that channel. If not, then it returns none.
pub async fn alert_channel_defined(guild_id: &GuildId, ctx: &Context, alert_channel: ChannelId) -> Option<GuildChannel> {
    match ctx.http.get_guild(guild_id.0).await {
        Ok(guild) => match guild.channels(ctx).await {
            Ok(guild_channels) => match guild_channels.get(&alert_channel) {
                Some(alert_channel) => return Some(alert_channel.clone()),
                None => info!("Event Listener: Got a list of channels, but could not find the configured alert channel")
            },
            Err(err) => {
                error!("Event Listener: Failed to get the channels in the guild with the following error\n{err}")
            }
        },
        Err(err) => error!("Event Listener: Failed to resolve the guild ID to a guild with the following error\n{err}")
    }
    None
}
