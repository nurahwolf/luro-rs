use crate::{functions::guild_accent_colour::guild_accent_colour, Context, Error};
use poise::serenity_prelude::{self as serenity, Mentionable};

#[derive(Debug, poise::ChoiceParameter)]
pub enum PunishType {
    Ban,
    Kick,
    Mute
}

/// Ban, kick or muzzle someone for being bad
#[poise::command(slash_command, prefix_command, required_permissions = "BAN_MEMBERS", guild_only, category = "Moderation")]
pub async fn punish(
    ctx: Context<'_>,
    #[description = "Punishment type"]
    #[rename = "type"]
    punish_type: PunishType,
    #[description = "User to execute the punishment on"] user: serenity::User,
    #[description = "The reason they should be punished"] mut reason: String
) -> Result<(), Error> {
    let guild = match ctx.guild() {
        Some(ok) => ok,
        None => {
            ctx.say("This command only works in a guild!").await?;
            return Ok(());
        }
    };
    let user_id = user.id;
    let user_mention = user.mention();
    let user_avatar = user.avatar_url().unwrap_or_default();
    reason = format!("**Actioned by {}:** {}", ctx.author().mention(), reason);

    let (title, description) = match punish_type {
        PunishType::Ban => {
            if reason.is_empty() {
                guild.ban(ctx, user, 1).await?;
            } else {
                guild.ban_with_reason(ctx, user, 1, reason.as_str()).await?;
            }

            (
                "BANNED!",
                format!("Seems like the trash known as {user_mention} (ID:`{user_id}`) has been dumped for good.\nThey were banned for the following reason:\n{reason}")
            )
        }
        PunishType::Kick => {
            if reason.is_empty() {
                guild.kick(ctx, user).await?;
            } else {
                guild.kick_with_reason(ctx, user, reason.as_str()).await?;
            }

            (
                "KICKED",
                format!("It was about time that {user_mention} (ID:`{user_id}`) got kicked.\n\nThey were kicked for the following reason:\n{reason}")
            )
        }
        PunishType::Mute => {
            ctx.say("Currently this is not implemented... Yet.").await?;

            (
                "Muted",
                format!("A muzzle has been placed on {user_mention} (ID:`{user_id}`).\n\nThey were muzzled for the following reason:\n{reason}")
            )
        }
    };
    let accent_colour = ctx.data().config.read().await.accent_colour;

    ctx.send(|b| {
        b.embed(|b| {
            b.title(title)
                .description(description)
                .color(guild_accent_colour(accent_colour, ctx.guild()))
                .thumbnail(user_avatar)
        })
    })
    .await?;

    Ok(())
}
