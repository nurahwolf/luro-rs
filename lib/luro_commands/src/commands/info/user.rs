use poise::serenity_prelude::{Guild, User};

use luro_core::{Context, Error};

use crate::functions::userinfo::user_info;

/// Show some information about a user
#[poise::command(prefix_command, slash_command, category = "Guild")]
pub async fn user(
    ctx: Context<'_>,
    #[description = "The user to get"] user: Option<User>,
    #[description = "Specify a guild if you wish to get their information from that guild"] guild: Option<Guild>
) -> Result<(), Error> {
    // Get the user, otherwise set the message author as the user to get
    let user = match user {
        Some(user_specified) => user_specified,
        None => ctx.author().clone()
    };

    user_info(ctx, user, guild).await?;
    Ok(())
}
