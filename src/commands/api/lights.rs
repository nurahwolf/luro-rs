use luro_core::{Context, Error};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Lights {
    #[serde(rename = "list")]
    // error: Option<String>,
    success: bool
}

/// Fuck around with someones lights. This may or may not work, depending on if their API is running.
#[poise::command(slash_command, prefix_command, category = "API")]
pub async fn lights(ctx: Context<'_>, #[description = "Enter a hex code"] hex: String) -> Result<(), Error> {
    let client = reqwest::Client::builder().user_agent("Luro/1.0 (nurah@wolfo.tech)").build()?;
    let request = client
        .get("http://rainy-nights.net:5000/api/rain_room/output/light/?auth_key=yingletyule&type=rgb_arr&id=1&mode=fill")
        .query(&[("colour", &hex)])
        .send()
        .await?;

    let response: Lights = request.json().await?;

    if !&response.success {
        ctx.say(format!("We had a fucky wucky: `{}`", &response.success)).await?;
        return Ok(());
    }

    ctx.say(format!("Successfully fucked with: `{}`", &response.success)).await?;
    Ok(())
}
