use luro_utilities::guild_accent_colour;
use poise::serenity_prelude::{Channel, Role};

use luro_core::{guild_settings::LuroGuildSettings, Context, Error};

/// Sets the guild's settings
#[poise::command(slash_command, guild_only, required_permissions = "MANAGE_GUILD")]
pub async fn settings(
    ctx: Context<'_>,
    #[description = "Streams Discord's events to a channel, such as message edits and pins"] discord_events_log_channel: Option<
        Channel
    >,
    #[description = "Streams Moderation actions to a channel, such as kicks, warnings and bans"] moderator_actions_log_channel: Option<Channel>,

    #[description = "A list of roles that can use 'Moderator' level commands"] moderator_role_override: Option<Role>
) -> Result<(), Error> {
    let accent_colour = ctx.data().config.read().await.accent_colour;

    let discord_events_log_channel = discord_events_log_channel.map(|channel| channel.id());

    let moderator_actions_log_channel = moderator_actions_log_channel.map(|channel| channel.id());

    let moderator_role_override = moderator_role_override.map(|moderator_logs_channel| vec![moderator_logs_channel.id]);

    let guild_settings = LuroGuildSettings {
        discord_events_log_channel,
        moderator_actions_log_channel,
        moderator_role_override
    };

    // Safe to unwrap since we know we are in a guild
    let mut guild_settings_db = ctx.data().guild_settings.write().await;
    guild_settings_db
        .reload_guild(ctx.guild_id().unwrap(), guild_settings.clone())
        .await;

    ctx.send(|message| {
        message.embed(|embed| {
            embed.author(|a| {
                a.name(ctx.author().tag())
                    .icon_url(ctx.author().avatar_url().unwrap_or_default())
            });
            embed.colour(guild_accent_colour(accent_colour, ctx.guild()));
            embed.thumbnail(ctx.author().avatar_url().unwrap_or_default());
            embed.description("Your guild settings are now as follows:");
            embed.field(
                "Discord Events Logging Channel",
                guild_settings.discord_events_log_channel.is_some(),
                true
            );
            embed.field(
                "Moderation Actions Logging Channel",
                guild_settings.moderator_actions_log_channel.is_some(),
                true
            );
            embed.field(
                "Moderation Role Override",
                guild_settings.moderator_role_override.is_some(),
                true
            );
            embed.footer(|f| f.text("Contact Nurah#5103 if you have any troubles"));
            embed
        })
    })
    .await?;

    Ok(())
}
