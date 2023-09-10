use luro_model::database_driver::LuroDatabaseDriver;
use std::fmt::Write;
use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_model::{
    guild::Permissions,
    id::{marker::ChannelMarker, Id},
};

use crate::{functions::parse_string_to_u32, interaction::LuroSlash, luro_command::LuroCommand};

#[derive(CommandModel, CreateCommand, Debug, PartialEq, Eq)]
#[command(
    name = "settings",
    desc = "Set guild settings, such a logging channel and accent colour.",
    dm_permission = false,
    default_permissions = "Self::default_permissions"
)]
pub struct GuildSettingsCommand {
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

impl LuroCommand for GuildSettingsCommand {
    fn default_permissions() -> Permissions {
        Permissions::MANAGE_GUILD
    }

    async fn run_command<D: LuroDatabaseDriver>(self, ctx: LuroSlash<D>) -> anyhow::Result<()> {
        let guild_id = match ctx.interaction.guild_id {
            Some(guild_id) => guild_id,
            None => return ctx.not_guild_response().await,
        };
        let mut guild = ctx.framework.database.get_guild(&guild_id).await?;
        if let Ok(twilight_guild) = ctx.framework.twilight_client.guild(guild_id).await {
            guild.update_guild(twilight_guild.model().await?);
        }

        let mut embed = ctx.default_embed().await;
        embed.title(format!("Guild Setting - {}", guild.name));
        if let Some(clear_settings) = self.clear_settings && clear_settings {
            guild.accent_colour_custom = Default::default();
            guild.catchall_log_channel = Default::default();
            guild.commands = Default::default();
            guild.message_events_log_channel = Default::default();
            guild.moderator_actions_log_channel = Default::default();
            guild.thread_events_log_channel = Default::default();
            guild.accent_colour_custom = self.accent_colour.clone().map(|c|parse_string_to_u32(c.as_ref()).unwrap_or_default());
        };

        if let Some(accent_colour) = &self.accent_colour {
            guild.accent_colour_custom = match parse_string_to_u32(accent_colour) {
                Ok(accent_colour) => Some(accent_colour),
                Err(_) => None,
            }
        }

        if let Some(catchall_log_channel) = self.catchall_log_channel {
            guild.catchall_log_channel = Some(catchall_log_channel)
        }

        if let Some(message_events_log_channel) = self.message_events_log_channel {
            guild.message_events_log_channel = Some(message_events_log_channel)
        }

        if let Some(moderator_actions_log_channel) = self.moderator_actions_log_channel {
            guild.moderator_actions_log_channel = Some(moderator_actions_log_channel)
        }

        if let Some(thread_events_log_channel) = self.thread_events_log_channel {
            guild.thread_events_log_channel = Some(thread_events_log_channel)
        }

        // Call manage guild settings, which allows us to make sure that they are present both on disk and in the cache.
        ctx.framework.database.modify_guild(&guild_id, &guild).await?;
        if let Some((colour, position, role)) = guild.highest_role_colour() {
            guild.accent_colour = Some(colour);
            embed.create_field(
                "Guild Accent Colour",
                &format!("`{colour:X}` - <@&{role}> - Position `{position}`"),
                true,
            );
        }

        if let Some(accent_colour) = guild.accent_colour_custom {
            embed.create_field("Custom Accent Colour", &format!("`{:X}`", accent_colour), true);
        }
        if let Some(channel) = guild.catchall_log_channel {
            embed.create_field("Catchall Log Channel", &format!("<#{channel}>"), true);
        }
        if let Some(channel) = guild.message_events_log_channel {
            embed.create_field("Message Log Channel", &format!("<#{channel}>"), true);
        }
        if let Some(channel) = guild.moderator_actions_log_channel {
            embed.create_field("Moderation Log Channel", &format!("<#{channel}>"), true);
        }
        if let Some(channel) = guild.thread_events_log_channel {
            embed.create_field("Thread Log Channel", &format!("<#{channel}>"), true);
        }

        let mut blacklist = String::new();
        for role in guild.assignable_role_blacklist {
            writeln!(blacklist, "- <@&{role}>")?;
        }
        if !blacklist.is_empty() {
            embed.create_field("Blacklisted Roles from Selfassign", &blacklist, false);
        }

        ctx.respond(|r| r.add_embed(embed)).await
    }
}
