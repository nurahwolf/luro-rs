use luro_core::{Context, Error};

use crate::joinvoice::joinvoice;

/// Join a VC channel.
#[poise::command(slash_command, prefix_command, guild_only, ephemeral, category = "Music")]
pub async fn join(
    ctx: Context<'_>,
    #[description = "Announce how long the bot has been in the voice channel"] announce: Option<bool>
) -> Result<(), Error> {
    joinvoice(ctx, announce).await?;
    Ok(())
}
