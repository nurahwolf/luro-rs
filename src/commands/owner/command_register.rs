use crate::{Context, Error};

/// Register Slash Commands (Owner Only)
#[poise::command(owners_only, slash_command, prefix_command, ephemeral, category = "Owner")]
pub async fn register(ctx: Context<'_>) -> Result<(), Error> {
    poise::builtins::register_application_commands_buttons(ctx).await?;
    Ok(())
}
