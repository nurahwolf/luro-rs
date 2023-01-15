use crate::{Context, Error};

/// Set bot nickname
#[poise::command(
    slash_command,
    prefix_command,
    guild_only,
    category = "Guild",
)]
pub async fn nickname(
    ctx: Context<'_>,
    #[description = "Nickname to set"]
    #[rest]
    nick: Option<String>
) -> Result<(), Error> {
    let gid = match ctx.guild_id() {
        Some(ok) => ok,
        None => {
            ctx.say("Failed to get the guild").await?;
            return Ok(());
        }
    };
    let gid_u64 = gid.as_u64();

    match nick {
        Some(nick) => {
            ctx.serenity_context()
                .http
                .edit_nickname(*gid_u64, Some(&nick.to_owned()))
                .await?;
            ctx.say(format!("Set my nickname to `{nick}`.")).await?;
        }
        None => {
            ctx.serenity_context().http.edit_nickname(*gid_u64, None).await?;
            ctx.say("Cleared my nickname / left it the same.").await?;
        }
    };

    Ok(())
}
