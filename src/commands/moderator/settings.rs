use async_trait::async_trait;

use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_model::{
    guild::Permissions,
    id::{marker::ChannelMarker, Id}
};
use twilight_util::builder::embed::EmbedFieldBuilder;

use crate::{
    models::{GuildSetting, LuroSlash, RoleOrdering},
    traits::luro_command::LuroCommand
};

#[derive(CommandModel, CreateCommand, Debug, PartialEq, Eq)]
#[command(
    name = "settings",
    desc = "Set guild settings, such a logging channel and accent colour.",
    dm_permission = false,
    default_permissions = "Self::default_permissions"
)]
pub struct GuildSettingsCommand {
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

#[async_trait]
impl LuroCommand for GuildSettingsCommand {
    fn default_permissions() -> Permissions {
        Permissions::MANAGE_GUILD
    }

    async fn run_command(self, ctx: LuroSlash) -> anyhow::Result<()> {
        let guild_id = match ctx.interaction.guild_id {
            Some(guild_id) => guild_id,
            None => return ctx.not_guild_response().await
        };
        let guild = ctx.luro.twilight_client.guild(guild_id).await?.model().await?;
        // Attempt to get the first role after @everyone, otherwise fall back to @everyone
        let mut roles: Vec<_> = guild.roles.iter().map(RoleOrdering::from).collect();
        roles.sort();
        let highest_role_id = roles.last().unwrap().id; // SAFETY: roles is not empty;
        let highest_role = guild.roles.iter().find(|role| role.id == highest_role_id);

        let accent_colour_defined: Option<u32> = if let Some(accent_colour) = self.accent_colour.clone() {
            if accent_colour.starts_with("0x") {
                Some(u32::from_str_radix(accent_colour.as_str().strip_prefix("0x").unwrap(), 16)?)
            } else if accent_colour.chars().all(|char| char.is_ascii_hexdigit()) {
                Some(u32::from_str_radix(accent_colour.as_str(), 16)?)
            } else {
                Some(accent_colour.parse::<u32>()?)
            }
        } else {
            None
        };
        let mut embed = ctx.default_embed().await?;
        embed = embed.title(format!("Guild Setting - {}", guild.name));

        // Create a new guild settings object
        let mut guild_settings = GuildSetting::default();

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

        if let Some(role) = highest_role {
            guild_settings.accent_colour = role.color
        }

        // Call manage guild settings, which allows us to make sure that they are present both on disk and in the cache.
        let guild_settings = GuildSetting::modify_guild_settings(&ctx.luro, &guild_id, &guild_settings).await?;

        embed =
            embed.field(EmbedFieldBuilder::new("Guild Accent Colour", format!("`{}`", guild_settings.accent_colour)).inline());

        if let Some(accent_colour) = guild_settings.accent_colour_custom {
            embed = embed.field(EmbedFieldBuilder::new("Custom Accent Colour", format!("`{accent_colour}`")).inline())
        }
        if let Some(catchall_log_channel) = guild_settings.catchall_log_channel {
            embed = embed.field(EmbedFieldBuilder::new("Catchall Log Channel", format!("<#{catchall_log_channel}>")).inline())
        }
        if let Some(message_events_log_channel) = guild_settings.message_events_log_channel {
            embed = embed.field(EmbedFieldBuilder::new("Message Log Channel", format!("<#{message_events_log_channel}>")).inline())
        }
        if let Some(moderator_actions_log_channel) = guild_settings.moderator_actions_log_channel {
            embed = embed.field(EmbedFieldBuilder::new("Moderation Log Channel", format!("<#{moderator_actions_log_channel}>")).inline())
        }
        if let Some(thread_events_log_channel) = guild_settings.thread_events_log_channel {
            embed = embed.field(EmbedFieldBuilder::new("Thread Log Channel", format!("<#{thread_events_log_channel}>")).inline())
        }

        ctx.luro.send_log_channel(&Some(guild_id), embed.clone(), crate::models::LuroLogChannel::Moderator).await?;

        ctx.embed(embed.build())?.respond().await
    }
}
