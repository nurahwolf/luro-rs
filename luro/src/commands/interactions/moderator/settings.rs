use anyhow::Context;
use luro_framework::{CommandInteraction, Luro, LuroCommand};
use std::fmt::Write;
use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_model::id::{marker::ChannelMarker, Id};

#[derive(CommandModel, CreateCommand, Debug, PartialEq, Eq)]
#[command(
    name = "settings",
    desc = "Set guild settings, such a logging channel and accent colour.",
    dm_permission = false
)]
pub struct Settings {
    /// Set this to true to completely clear all settings.
    pub clear_settings: Option<bool>,
    /// The accent colour for this guild. By default Luro will use the highest role colour.
    pub accent_colour: Option<String>,
    /// Log ALL events here, unless you set more specific channels
    pub catchall_log_channel: Option<Id<ChannelMarker>>,
    /// Events relating to threads (Create, modify, Delete) are logged here
    pub thread_events_log_channel: Option<Id<ChannelMarker>>,
    /// Events relating to messages (Create, modify, Delete) are logged here
    pub message_events_log_channel: Option<Id<ChannelMarker>>,
    /// Events relating to moderation (Ban, Kick) are logged here
    pub moderator_actions_log_channel: Option<Id<ChannelMarker>>,
}

impl LuroCommand for Settings {
    async fn interaction_command(self, ctx: CommandInteraction) -> anyhow::Result<luro_model::types::CommandResponse> {
        let mut guild = ctx.guild.clone().context("Expected this to be a guild")?;
        let mut guild_data = guild
            .data
            .clone()
            .context("Sorry, can't get your guild from the database at this time!")?;

        let mut embed = ctx.default_embed().await;
        embed.title(format!("Guild Setting - {}", guild.name));
        if let Some(clear_settings) = self.clear_settings
            && clear_settings
        {
            guild_data.accent_colour_custom = Default::default();
            // guild_data.catchall_log_channel = Default::default();
            // guild_data.message_events_log_channel = Default::default();
            guild_data.moderator_actions_log_channel = Default::default();
            // guild_data.thread_events_log_channel = Default::default();
            guild_data.accent_colour_custom = self
                .accent_colour
                .clone()
                .map(|c| parse_string_to_u32(c.as_ref()).unwrap_or_default());
        };

        if let Some(accent_colour) = &self.accent_colour {
            guild_data.accent_colour_custom = match parse_string_to_u32(accent_colour) {
                Ok(accent_colour) => Some(accent_colour),
                Err(_) => None,
            }
        }

        // if let Some(catchall_log_channel) = self.catchall_log_channel {
        //     guild_data.catchall_log_channel = Some(catchall_log_channel)
        // }

        // if let Some(message_events_log_channel) = self.message_events_log_channel {
        //     guild_data.message_events_log_channel = Some(message_events_log_channel)
        // }

        if let Some(moderator_actions_log_channel) = self.moderator_actions_log_channel {
            guild_data.moderator_actions_log_channel = Some(moderator_actions_log_channel)
        }

        // if let Some(thread_events_log_channel) = self.thread_events_log_channel {
        //     guild_data.thread_events_log_channel = Some(thread_events_log_channel)
        // }

        // Call manage guild settings, which allows us to make sure that they are present both on disk and in the cache.
        // ctx.database.modify_guild(&guild_id, &guild).await?;
        if let Some((position, role)) = guild.accent_colour() {
            guild_data.accent_colour = Some(role.colour);
            embed.create_field(
                "Guild Accent Colour",
                &format!("`{:X}` - <@&{role}> - Position `{position}`", role.colour),
                true,
            );
        }

        if let Some(accent_colour) = guild_data.accent_colour_custom {
            embed.create_field("Custom Accent Colour", &format!("`{:X}`", accent_colour), true);
        }
        // if let Some(channel) = guild_data.catchall_log_channel {
        //     embed.create_field("Catchall Log Channel", &format!("<#{channel}>"), true);
        // }
        // if let Some(channel) = guild_data.message_events_log_channel {
        //     embed.create_field("Message Log Channel", &format!("<#{channel}>"), true);
        // }
        if let Some(channel) = guild_data.moderator_actions_log_channel {
            embed.create_field("Moderation Log Channel", &format!("<#{channel}>"), true);
        }
        // if let Some(channel) = guild_data.thread_events_log_channel {
        //     embed.create_field("Thread Log Channel", &format!("<#{channel}>"), true);
        // }

        let mut blacklist = String::new();
        for role in &guild_data.role_blacklist {
            writeln!(blacklist, "- <@&{role}>")?;
        }
        if !blacklist.is_empty() {
            embed.create_field("Blacklisted Roles from Selfassign", &blacklist, false);
        }

        ctx.database.guild_update_data(&guild_data).await?;

        ctx.respond(|r| r.add_embed(embed)).await
    }
}

/// Parse a string into a u32, used for hex codes colours
pub fn parse_string_to_u32(input: &str) -> anyhow::Result<u32> {
    Ok(if input.starts_with("0x") {
        u32::from_str_radix(input.strip_prefix("0x").unwrap(), 16)?
    } else if input.chars().all(|char| char.is_ascii_hexdigit()) {
        u32::from_str_radix(input, 16)?
    } else {
        input.parse::<u32>()?
    })
}
