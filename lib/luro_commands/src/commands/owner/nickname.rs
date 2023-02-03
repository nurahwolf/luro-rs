use luro_core::{Context, Error};
use poise::serenity_prelude::{Member, Mentionable};

/// Abuse Luro to change someones nickname
#[poise::command(slash_command, prefix_command, guild_only, category = "Owner")]
pub async fn nickname(
    ctx: Context<'_>,
    #[description = "The user to abuse"] member: Member,
    #[description = "Nickname to set"]
    #[rest]
    nick: Option<String>
) -> Result<(), Error> {
    let new_nickname = match nick {
        Some(nick) => nick,
        None => "".to_string()
    };

    match member.edit(ctx, |member| member.nickname(&new_nickname)).await {
        Ok(member) => {
            ctx.say(format!(
                "Looks like {} got admin abused and their nickname is now `{}`.",
                member.mention(),
                &new_nickname
            ))
            .await?
        }
        Err(err) => {
            ctx.say(format!("Had an error: {err}")).await?;
            return Ok(());
        }
    };

    Ok(())
}
