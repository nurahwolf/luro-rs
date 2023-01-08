use crate::{functions::saucenao::interactive_response, Context, Error, SOURCE_FINDER_REGEX};
use futures::{future, StreamExt};
use poise::serenity_prelude::Message;
use regex::Regex;

/// Reverse lookup an image via SauceNAO / FuzzySearch!
#[poise::command(prefix_command, slash_command, category = "Furry")]
pub async fn saucenao_lookup(
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

/// Reverse lookup an image via SauceNAO / FuzzySearch!
#[poise::command(context_menu_command = "SauceNAO: Find source", category = "Furry")]
pub async fn saucenao_context(ctx: Context<'_>, msg: Message) -> Result<(), Error> {
    let api_key = &ctx.data().secrets.saucenao_token;
    let regex = Regex::new(SOURCE_FINDER_REGEX).unwrap();
    let mut urls = Vec::new();
    for cap in regex.captures_iter(&msg.content) {
        urls.push(cap["url"].to_string())
    }

    if urls.is_empty() {
        for attachment in msg.attachments {
            urls.push(attachment.proxy_url)
        }
    }

    let mut stream = futures::stream::iter(urls);
    let mut futures = Vec::new();

    while let Some(url) = stream.next().await {
        futures.push(interactive_response(ctx, url, api_key));
    }

    // TODO: Error handle this
    let _results = future::join_all(futures).await;

    Ok(())

}
