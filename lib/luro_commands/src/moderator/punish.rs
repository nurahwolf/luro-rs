use chrono::{Duration, Utc};
use luro_core::{Context, Error};
use luro_utilities::guild_accent_colour;
use poise::serenity_prelude::{CreateEmbed, Timestamp, User};

#[derive(Debug, poise::ChoiceParameter)]
pub enum PunishType {
    Ban,
    Kick,
    Mute
}

/// Ban, kick or muzzle someone for being bad
#[poise::command(
    slash_command,
    prefix_command,
    guild_only,
    category = "Moderation"
)]
pub async fn punish(
    ctx: Context<'_>,
    #[description = "Punishment type"]
    #[rename = "type"]
    punish_type: PunishType,
    #[description = "User to execute the punishment on"] user: User,
    #[description = "The reason they should be punished"] reason: String,
    #[description = "Purge message history in days from 1 to 7, defaults to 1 if not set"] purge: Option<u8>
) -> Result<(), Error> {
    let mut embed = CreateEmbed::default();
    let accent_colour = ctx.data().config.read().await.accent_colour;
    let guild = match ctx.guild() {
        Some(ok) => ok,
        None => {
            ctx.say("This command only works in a guild!").await?;
            return Ok(());
        }
    };

    let bot_permissions = match guild.member(ctx, ctx.framework().bot_id).await {
        Ok(bot_member) => match bot_member.permissions(ctx) {
            Ok(ok) => ok,
            Err(err) => {
                ctx.say(format!(
                    "Failed to get the permissions for the bot with the following reason: {err}"
                ))
                .await?;
                return Ok(());
            }
        },
        Err(err) => {
            ctx.say(format!("Failed to get the bot's user in the guild: {err}")).await?;
            return Ok(());
        }
    };

    let mut victim_member = match guild.member(ctx, user.id).await {
        Ok(ok) => ok,
        Err(err) => {
            ctx.say(format!(
                "Failed to get the member status of the author with the following reason: {err}"
            ))
            .await?;
            return Ok(());
        }
    };

    let author_member = match guild.member(ctx, ctx.author().id).await {
        Ok(ok) => ok,
        Err(err) => {
            ctx.say(format!(
                "Failed to get the member status of the author with the following reason: {err}"
            ))
            .await?;
            return Ok(());
        }
    };

    let author_permissions = match author_member.permissions(ctx) {
        Ok(ok) => ok,
        Err(err) => {
            ctx.say(format!(
                "Failed to get the permissions of the author with the following reason: {err}"
            ))
            .await?;
            return Ok(());
        }
    };

    // Set embed defaults
    embed.color(guild_accent_colour(accent_colour, ctx.guild()));
    embed.thumbnail(victim_member.clone().avatar.unwrap_or_default());
    embed.field("Reason", &reason, false);
    embed.field("User", &victim_member, true);
    embed.field("ID", victim_member.user.id, true);
    embed.author(|author| {
        author
            .name(author_member.display_name())
            .icon_url(author_member.avatar_url().unwrap_or_default())
    });

    match punish_type {
        PunishType::Ban => {
            if !bot_permissions.ban_members() {
                ctx.say("I'm afraid I'm missing `BAN_MEMBERS`, so I can't ban that user.")
                    .await?;
                return Ok(());
            }

            if ctx.framework().options.owners.contains(&ctx.author().id) || author_permissions.ban_members() {
                let purge_length = match purge {
                    Some(purge) => purge,
                    None => 1
                };

                match victim_member.ban_with_reason(ctx, purge_length, reason).await {
                    Ok(ok) => ok,
                    Err(err) => {
                        ctx.say(format!("Failed to ban the member with the following reason: {err}"))
                            .await?;
                        return Ok(());
                    }
                };

                embed.title("BANNED!");
                embed.description(format!(
                    "Looks like {} got banned. How unfortunate.",
                    victim_member.display_name()
                ));
                embed.field("Purged History", format!("{purge_length} days"), true);
            } else {
                ctx.say("Nice try, but you don't have permission to ban `[BAN_MEMBERS]`.")
                    .await?;
                return Ok(());
            }
        }
        PunishType::Kick => {
            if !bot_permissions.kick_members() {
                ctx.say("I'm afraid I'm missing `KICK_MEMBERS`, so I can't ban that user.")
                    .await?;
                return Ok(());
            }

            if ctx.framework().options.owners.contains(&ctx.author().id) || author_permissions.kick_members() {
                match victim_member.kick_with_reason(ctx, &reason).await {
                    Ok(ok) => ok,
                    Err(err) => {
                        ctx.say(format!("Failed to kick the member with the following reason: {err}"))
                            .await?;
                        return Ok(());
                    }
                };

                embed.title("Kicked");
                embed.description(format!(
                    "Looks like {} got kicked. Seems they are not wanted around these parts.",
                    victim_member.display_name()
                ));
            } else {
                ctx.say("Nice try, but you don't have permission to kick `[KICK_MEMBERS]`.")
                    .await?;
                return Ok(());
            }
        }
        PunishType::Mute => {
            if !bot_permissions.moderate_members() {
                ctx.say("I'm afraid I'm missing `MODERATE_MEMBERS`, so I can't ban that user.")
                    .await?;
                return Ok(());
            }

            if ctx.framework().options.owners.contains(&ctx.author().id) || author_permissions.moderate_members() {
                // Time now, add 10 minutes
                let utc = Utc::now() + Duration::minutes(10);
                let timestamp = Timestamp::from(utc);

                match victim_member.disable_communication_until_datetime(ctx, timestamp).await {
                    Ok(ok) => ok,
                    Err(err) => {
                        ctx.say(format!("Failed to muzzle that member because of the following reason: {err}"))
                            .await?;
                        return Ok(());
                    }
                };

                embed.title("Muzzled");
                embed.description(format!(
                    "Looks like {} got muzzled. Maybe now they will learn to shut the fuck up.",
                    victim_member.display_name()
                ));
            } else {
                ctx.say("Nice try, but you don't have permission to timeout `[MODERATE_MEMBERS]`.")
                    .await?;
                return Ok(());
            }
        }
    };

    ctx.send(|b| {
        b.embed(|e| {
            *e = embed;
            e
        })
    })
    .await?;

    Ok(())
}
