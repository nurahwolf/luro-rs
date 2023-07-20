use async_trait::async_trait;
use twilight_gateway::MessageSender;
use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_model::{
    application::interaction::Interaction,
    guild::Permissions,
    id::{marker::ChannelMarker, Id}
};
use twilight_util::builder::embed::EmbedFieldBuilder;

use crate::{
    commands::LuroCommand,
    interactions::InteractionResponse,
    responses::{no_guild_settings::no_guild_settings, unable_to_get_guild::unable_to_get_guild},
    LuroContext, SlashResponse
};

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

    async fn run_command(self, interaction: Interaction, ctx: LuroContext, _shard: MessageSender) -> SlashResponse {
        let ephemeral = ctx.defer_interaction(&interaction, true).await?;
        let (_, _, _) = self.interaction_context(&interaction, "mod setting")?;

        let guild_id = match interaction.guild_id {
            Some(guild_id) => guild_id,
            None => return Ok(unable_to_get_guild("No guild ID for this interaction".to_owned()))
        };
        let guild = ctx.twilight_client.guild(guild_id).await?.model().await?;
        let mut embed = self.default_embed(&ctx, interaction.guild_id);
        let mut guild_settings = ctx.guild_data.write();
        embed = embed.title(format!("Guild Setting - {}", guild.name));

        guild_settings.entry(guild_id).and_modify(|guild_setting| {
            if let Some(accent_colour) = self.accent_colour.clone() {
                let accent_colour: u32 = accent_colour.parse().unwrap();
                guild_setting.accent_colour_custom = Some(accent_colour)
            };

            guild_setting.moderator_actions_log_channel = self.moderator_action_log_channel;
            guild_setting.discord_events_log_channel = self.bot_log_channel;
        });

        let guild_setting = match guild_settings.get(&guild_id) {
            Some(guild_setting) => guild_setting,
            None => return Ok(no_guild_settings(ephemeral, true))
        };

        embed = embed.field(EmbedFieldBuilder::new(
            "Guild Custom Accent Colour",
            if let Some(accent_colour) = guild_setting.accent_colour_custom {
                format!("`{}`", accent_colour)
            } else {
                let accent_colour = guild_setting.accent_colour_custom.unwrap_or(guild_setting.accent_colour);
                format!("`{}`", accent_colour)
            }
        ));

        embed = embed.field(EmbedFieldBuilder::new(
            "Moderator Action Log Channel",
            if let Some(moderator_action_log_channel) = guild_setting.moderator_actions_log_channel {
                format!("<@{}>", moderator_action_log_channel.get())
            } else {
                match guild_setting.moderator_actions_log_channel {
                    Some(moderator_actions_log_channel) => format!("<@{}>", moderator_actions_log_channel.get()),
                    None => "Not set!".to_owned()
                }
            }
        ));

        embed = embed.field(EmbedFieldBuilder::new(
            "Bot Log Channel",
            if let Some(bot_log_channel) = guild_setting.discord_events_log_channel {
                format!("<@{}>", bot_log_channel.get())
            } else {
                match guild_setting.discord_events_log_channel {
                    Some(bot_log_channel) => format!("<@{}>", bot_log_channel.get()),
                    None => "Not set!".to_owned()
                }
            }
        ));

        // Now respond to the original interaction
        Ok(InteractionResponse::Embed {
            embeds: vec![embed.build()],
            ephemeral,
            deferred: true
        })
    }
}
