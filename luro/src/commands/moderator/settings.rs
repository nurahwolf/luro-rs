

use luro_model::luro_log_channel::LuroLogChannel;
use tracing::debug;
use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_model::{
    guild::Permissions,
    id::{marker::ChannelMarker, Id}
};
use twilight_util::builder::embed::EmbedFieldBuilder;

use crate::{functions::parse_string_to_u32, models::RoleOrdering, slash::Slash, traits::luro_command::LuroCommand};

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
    pub moderator_actions_log_channel: Option<Id<ChannelMarker>>
}


impl LuroCommand for GuildSettingsCommand {
    fn default_permissions() -> Permissions {
        Permissions::MANAGE_GUILD
    }

    async fn run_command(self, mut ctx: Slash) -> anyhow::Result<()> {
        let guild_id = match ctx.interaction.guild_id {
            Some(guild_id) => guild_id,
            None => return ctx.not_guild_response().await
        };
        let guild = ctx.framework.twilight_client.guild(guild_id).await?.model().await?;
        // Attempt to get the first role after @everyone, otherwise fall back to @everyone
        let mut roles: Vec<_> = guild.roles.iter().map(RoleOrdering::from).collect();
        roles.sort_by(|a, b| b.cmp(a));

        let mut highest_role_colour = 0;
        let mut highest_role_id = 0;
        for role in &roles {
            if highest_role_colour != 0 {
                break;
            }

            highest_role_colour = role.colour;
            highest_role_id = role.id.get();
            debug!("Role {highest_role_id} - {}", role.colour)
        }

        let accent_colour_defined: Option<u32> = if let Some(accent_colour) = self.accent_colour.clone() {
            Some(parse_string_to_u32(accent_colour)?)
        } else {
            None
        };
        let mut embed = ctx.default_embed().await?;
        embed = embed.title(format!("Guild Setting - {}", guild.name));

        // Create a new guild settings object
        let mut guild_settings = ctx.framework.database.get_guild(&guild_id).await?;
        if let Some(clear_settings) = self.clear_settings && clear_settings {
            guild_settings.accent_colour_custom = Default::default();
            guild_settings.catchall_log_channel = Default::default();
            guild_settings.commands = Default::default();
            guild_settings.message_events_log_channel = Default::default();
            guild_settings.moderator_actions_log_channel = Default::default();
            guild_settings.thread_events_log_channel = Default::default();
        };

        guild_settings.accent_colour = highest_role_colour;

        if let Some(accent_colour) = accent_colour_defined {
            guild_settings.accent_colour_custom = Some(accent_colour)
        }

        if let Some(catchall_log_channel) = self.catchall_log_channel {
            guild_settings.catchall_log_channel = Some(catchall_log_channel)
        }

        if let Some(message_events_log_channel) = self.message_events_log_channel {
            guild_settings.message_events_log_channel = Some(message_events_log_channel)
        }

        if let Some(moderator_actions_log_channel) = self.moderator_actions_log_channel {
            guild_settings.moderator_actions_log_channel = Some(moderator_actions_log_channel)
        }

        if let Some(thread_events_log_channel) = self.thread_events_log_channel {
            guild_settings.thread_events_log_channel = Some(thread_events_log_channel)
        }

        // Call manage guild settings, which allows us to make sure that they are present both on disk and in the cache.
        ctx.framework.database.update_guild(guild_id, &guild_settings).await?;

        embed = embed.field(
            EmbedFieldBuilder::new(
                "Guild Accent Colour",
                format!("`{:X}` - <@&{highest_role_id}>", guild_settings.accent_colour)
            )
            .inline()
        );

        if let Some(accent_colour) = guild_settings.accent_colour_custom {
            embed = embed.field(EmbedFieldBuilder::new("Custom Accent Colour", format!("`{:X}`", accent_colour)).inline())
        }
        if let Some(catchall_log_channel) = guild_settings.catchall_log_channel {
            embed = embed.field(EmbedFieldBuilder::new("Catchall Log Channel", format!("<#{catchall_log_channel}>")).inline())
        }
        if let Some(message_events_log_channel) = guild_settings.message_events_log_channel {
            embed =
                embed.field(EmbedFieldBuilder::new("Message Log Channel", format!("<#{message_events_log_channel}>")).inline())
        }
        if let Some(moderator_actions_log_channel) = guild_settings.moderator_actions_log_channel {
            embed = embed
                .field(EmbedFieldBuilder::new("Moderation Log Channel", format!("<#{moderator_actions_log_channel}>")).inline())
        }
        if let Some(thread_events_log_channel) = guild_settings.thread_events_log_channel {
            embed =
                embed.field(EmbedFieldBuilder::new("Thread Log Channel", format!("<#{thread_events_log_channel}>")).inline())
        }

        ctx.framework
            .send_log_channel(&Some(guild_id), embed.clone(), LuroLogChannel::Moderator)
            .await?;

        ctx.embed(embed.build())?.respond().await
    }
}
