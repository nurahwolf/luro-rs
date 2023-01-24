use crate::functions::saucenao::interactive_response;
use luro_core::{Context, Error};

/// Reverse lookup an image via SauceNAO / FuzzySearch!
#[poise::command(prefix_command, slash_command, category = "Furry")]
pub async fn saucenao(
    ctx: Context<'_>,
    #[rest]
    #[description = "URL to lookup"]
    url: String
) -> Result<(), Error> {
    let api_key = &ctx.data().secrets.saucenao_token;
    let response = interactive_response(ctx, url, api_key).await;

    match response {
        Ok(_) => Ok(()),
        Err(err) => {
            ctx.say(format!("Find Source: Had an error - {err}")).await?;
            Ok(())
        }
    }
}
