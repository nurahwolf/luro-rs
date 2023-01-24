use poise::serenity_prelude::User;

use luro_core::{Context, Error};

/// Admin abuse
#[poise::command(slash_command, prefix_command, owners_only, ephemeral, category = "Owner")]
pub async fn adminabuse(
    ctx: Context<'_>,
    #[description = "The user I should pretend to be"] user: User,
    #[description = "What bullshit should they say"]
    #[rest]
    msg: String
) -> Result<(), Error> {
    let guild = ctx.serenity_context().cache.guild(ctx.guild_id().unwrap()).unwrap();
    let member = guild.member(ctx, &user).await?;
    let mut webhooks = ctx.channel_id().webhooks(ctx).await?;

    if !webhooks.iter().any(|w| w.name.contains(&"LuroHook")) || webhooks.is_empty() {
        ctx.channel_id().create_webhook(ctx, "LuroHook").await?;
        webhooks = ctx.channel_id().webhooks(ctx).await?;
    }

    for webhook in webhooks {
        if webhook.name.contains(&"LuroHook") {
            webhook
                .execute(ctx, false, |w| {
                    w.content(&msg);

                    if !member.display_name().is_empty() {
                        w.username(&member.display_name());
                    } else {
                        w.username(&user.name);
                    }

                    if member.avatar.is_some() {
                        w.avatar_url(&member.avatar_url().unwrap());
                    } else {
                        w.avatar_url(
                            &user
                                .avatar_url()
                                .unwrap_or("https://cdn.discordapp.com/avatars/267365356912246784/7d4ed643250f41f18d94fd8377841884.webp?size=1024".to_string())
                        );
                    }
                    w
                })
                .await?;
        }
    }

    ctx.say("Mirrored!").await?;

    Ok(())
}
