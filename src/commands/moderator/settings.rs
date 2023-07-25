use async_trait::async_trait;
use tracing::info;

use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_model::{
    guild::Permissions,
    id::{marker::ChannelMarker, Id}
};
use twilight_util::builder::embed::EmbedFieldBuilder;

use crate::{commands::LuroCommand, functions::RoleOrdering, models::GuildSetting, responses::LuroSlash};
use std::collections::hash_map::Entry::Occupied;
use std::collections::hash_map::Entry::Vacant;

#[derive(CommandModel, CreateCommand, Debug, PartialEq, Eq)]
#[command(
    name = "settings",
    desc = "Set guild settings, such a logging channel and accent colour.",
    dm_permission = false,
    default_permissions = "Self::default_permissions"
)]
pub struct GuildSettingsCommand {
    /// Moderator action logging channel
    pub moderator_action_log_channel: Option<Id<ChannelMarker>>,
    /// Bot spam such as message deletes and guild events
    pub bot_log_channel: Option<Id<ChannelMarker>>,
    /// The accent colour for this guild. By default Luro will use the highest role colour.
    pub accent_colour: Option<String>
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
        let mut embed = ctx.default_embed();
        embed = embed.title(format!("Guild Setting - {}", guild.name));

        // Get guild settings, otherwise create a new entry
        let guild_setting = match ctx.luro.guild_data.write().entry(guild_id) {
            Occupied(mut entry) => {
                let guild_setting = entry.get_mut();

                if let Some(accent_colour) = accent_colour_defined {
                    guild_setting.accent_colour_custom = Some(accent_colour)
                }

                if let Some(moderator_action_log_channel) = self.moderator_action_log_channel {
                    guild_setting.moderator_actions_log_channel = Some(moderator_action_log_channel)
                }

                if let Some(bot_log_channel) = self.bot_log_channel {
                    guild_setting.discord_events_log_channel = Some(bot_log_channel)
                }

                if let Some(role) = highest_role {
                    info!(role.name);
                    guild_setting.accent_colour = role.color
                }

                guild_setting.clone()
            }
            Vacant(vacant) => {
                let mut guild_setting: GuildSetting = Default::default();
                if let Some(accent_colour) = accent_colour_defined {
                    guild_setting.accent_colour_custom = Some(accent_colour)
                }

                if let Some(moderator_action_log_channel) = self.moderator_action_log_channel {
                    guild_setting.moderator_actions_log_channel = Some(moderator_action_log_channel)
                }

                if let Some(bot_log_channel) = self.bot_log_channel {
                    guild_setting.discord_events_log_channel = Some(bot_log_channel)
                }

                if let Some(role) = highest_role {
                    info!(role.name);
                    guild_setting.accent_colour = role.color
                }
                vacant.insert(guild_setting.clone());
                guild_setting
            }
        };

        embed =
            embed.field(EmbedFieldBuilder::new("Guild Accent Colour", format!("`{}`", guild_setting.accent_colour)).inline());

        embed = embed.field(
            EmbedFieldBuilder::new(
                "Guild Custom Accent Colour",
                if let Some(accent_colour) = guild_setting.accent_colour_custom {
                    format!("`{}`", accent_colour)
                } else {
                    "Not set!".to_owned()
                }
            )
            .inline()
        );

        embed = embed.field(EmbedFieldBuilder::new(
            "Moderator Action Log Channel",
            if let Some(moderator_action_log_channel) = guild_setting.moderator_actions_log_channel {
                format!("<#{}>", moderator_action_log_channel.get())
            } else {
                match guild_setting.moderator_actions_log_channel {
                    Some(moderator_actions_log_channel) => format!("<@{}>", moderator_actions_log_channel.get()),
                    None => "Not set!".to_owned()
                }
            }
        ));

        embed = embed.field(
            EmbedFieldBuilder::new(
                "Bot Log Channel",
                if let Some(bot_log_channel) = guild_setting.discord_events_log_channel {
                    format!("<#{}>", bot_log_channel.get())
                } else {
                    match guild_setting.discord_events_log_channel {
                        Some(bot_log_channel) => format!("<#{}>", bot_log_channel.get()),
                        None => "Not set!".to_owned()
                    }
                }
            )
            .inline()
        );

        if let Some(moderator_actions_log_channel) = guild_setting.moderator_actions_log_channel {
            ctx.luro
                .twilight_client
                .create_message(moderator_actions_log_channel)
                .embeds(&[embed.clone().build()])?
                .await?;
        }

        ctx.embed(embed.build())?.respond().await
    }
}