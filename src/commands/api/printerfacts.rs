use luro_core::{Context, Error};

/// Random printer facts!
#[poise::command(slash_command, prefix_command, category = "API")]
pub async fn printerfacts(ctx: Context<'_>) -> Result<(), Error> {
    let body = reqwest::get("https://printerfacts.cetacean.club/fact").await?.text().await?;

    ctx.say(body).await?;
    Ok(())
}
