use luro_utilities::guild_accent_colour;
use poise::serenity_prelude::{Channel, Role};

use luro_core::{guild_settings::LuroGuildSettings, Context, Error};

/// Sets the guild's settings
#[poise::command(slash_command, guild_only)]
pub async fn settings(
    ctx: Context<'_>,
    #[description = "The channel to output discord event logs"] moderator_logs_channel: Option<Channel>,
    #[description = "A list of roles that can use 'Moderator' level commands"] moderator_role_override: Option<Role>
) -> Result<(), Error> {
    let moderator_logs_channel = match moderator_logs_channel {
        Some(moderator_logs_channel) => Some(moderator_logs_channel.id()),
        None => None
    };

    let moderator_role_override = match moderator_role_override {
        Some(moderator_logs_channel) => Some(vec![moderator_logs_channel]),
        None => None
    };

    let guild_settings = LuroGuildSettings {
        moderator_logs_channel,
        moderator_role_override
    };

    // Safe to unwrap since we know we are in a guild
    let mut guild_settings_db = ctx.data().guild_settings.write().await;
    guild_settings_db.reload_guild(ctx.guild_id().unwrap(), guild_settings.clone()).await;
    ctx.say(format!("Your Guild settings are as follows:\n{:?}", guild_settings))
        .await?;

    // ctx.send(|message| {
    //     message.embed(|embed| {
    //         embed.author(|a| a.name(msg.author.tag()).icon_url(msg.author.avatar_url().unwrap_or_default()));
    //         embed.colour(guild_accent_colour(accent_colour, ctx.guild()));
    //         embed.thumbnail(msg.author.avatar_url().unwrap_or_default());
    //         embed.description(&msg.content);
    //         embed.timestamp(msg.timestamp);
    //         embed.field("❯ Jump To Message", format!("[Click Here]({msg_link})"), true);
    //         embed.footer(|f| f.text(format!("Message ID: {}", msg.id)));
    //         embed
    //     })
    // })
    // .await?;

    Ok(())
}
