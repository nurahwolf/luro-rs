use luro_core::{Context, Error};
use poise::serenity_prelude::User;

use crate::functions::userinfo::user_info;

/// Show some information about a user
#[poise::command(category = "Guild", context_menu_command = "User info")]
pub async fn userinfo(ctx: Context<'_>, #[description = "The user to get"] user: User) -> Result<(), Error> {
    user_info(ctx, user, None).await?;
    Ok(())
}
