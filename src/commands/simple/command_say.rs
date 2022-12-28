use crate::{Context, Error};

/// Make me say garbage
#[poise::command(prefix_command, slash_command, category = "General")]
pub async fn say(
    ctx: Context<'_>,
    #[rest]
    #[description = "Text to say"]
    msg: String
) -> Result<(), Error> {
    ctx.say(msg).await?;
    Ok(())
}
