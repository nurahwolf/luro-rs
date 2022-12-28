use crate::{Context, Error};

/// Set bot nickname
#[poise::command(
    slash_command,
    prefix_command,
    guild_only,
    required_bot_permissions = "MANAGE_NICKNAMES",
    category = "Guild",
    identifying_name = "bot nickname"
)]
pub async fn botnick(
    ctx: Context<'_>,
    #[description = "Nickname to set"]
    #[rest]
    nick: Option<String>
) -> Result<(), Error> {
    let gid = ctx.guild_id().unwrap();
    let gid_u64 = gid.as_u64();

    match nick {
        Some(nick) => {
            ctx.serenity_context().http.edit_nickname(*gid_u64, Some(&nick.to_owned())).await?;
            ctx.say(format!("Set my nickname to \"{nick}\".")).await?;
        }
        None => {
            ctx.serenity_context().http.edit_nickname(*gid_u64, None).await?;
            ctx.say("Cleared my nickname / left it the same.").await?;
        }
    };

    Ok(())
}
