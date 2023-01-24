use luro_utilities::guild_accent_colour;
use poise::serenity_prelude::{Channel, CreateEmbed, Mentionable, Role};

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

    let mut embed = CreateEmbed::default();
    embed.author(|a| {
        a.name(ctx.author().tag())
            .icon_url(ctx.author().avatar_url().unwrap_or_default())
    });
    embed.colour(guild_accent_colour(accent_colour, ctx.guild()));
    embed.thumbnail(ctx.author().avatar_url().unwrap_or_default());
    embed.description("Your guild settings are now as follows:");
    embed.footer(|f| f.text("Contact Nurah#5103 if you have any troubles"));
    if let Some(discord_events) = guild_settings.discord_events_log_channel {
        embed.field("Discord Events Logging Channel", discord_events.mention(), true);
    }

    if let Some(moderation_actions) = guild_settings.moderator_actions_log_channel {
        embed.field("Moderator Actions Logging Channel", moderation_actions.mention(), true);
    }

    if let Some(moderator_role_override) = guild_settings.moderator_role_override {
        let mut description = "".to_string();
        for role in moderator_role_override {
            description.push_str(&format!("{}, ", &role.mention()).to_string());
        }
        embed.field("Discord Events Logging Channel", description, true);
    }

    ctx.send(|message| {
        message.embed(|e| {
            *e = embed;
            e
        })
    })
    .await?;

    Ok(())
}
