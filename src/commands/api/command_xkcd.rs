use crate::{utils::guild_accent_colour, Context, Error};
use poise::serenity_prelude::{self as serenity};
use reqwest::StatusCode;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
struct XkcdComic {
    num: u16,      // the numeric ID of the xkcd comic.
    alt: String,   // the caption of the xkcd comic.
    img: String,   // the image URL of the xkcd comic.
    title: String  // the title of the xkcd comic.
}

/// Retrieves the latest or a given comic from xkcd.
#[poise::command(slash_command, prefix_command, category = "API")]
pub async fn xkcd(ctx: Context<'_>, #[description = "Enter the comic number you want, or 0 for the latest"] comic_number: u32) -> Result<(), Error> {
    let latest_comic = "https://xkcd.com/info.0.json";
    let xkcd_url = format!("https://xkcd.com/{comic_number}/info.0.json");

    let client = reqwest::Client::new();
    let request = client.get(if comic_number == 0 { latest_comic } else { &xkcd_url }).send().await?;

    if request.status() == StatusCode::NOT_FOUND {
        ctx.say("You did not provide a valid xkcd comic ID!").await?;
        return Ok(());
    }

    let response: XkcdComic = request.json().await?;
    let title = &response.title;
    let alt = &response.alt;
    let num = response.num;
    let page = format!("https://xkcd.com/{num}");
    let wiki = format!("https://explainxkcd.com/wiki/index.php/{num}");

    ctx.send(|message| {
        message.embed(|embed| {
            embed.title(title);
            embed.colour(guild_accent_colour(ctx.data().config.lock().unwrap().accent_colour, ctx.guild()));
            embed.description(alt);
            embed.image(response.img.as_str());
            embed.footer(|f| f.text(format!("xkcd comic no. {}", &num)));
            embed
        });
        message.components(|c| {
            c.create_action_row(|row| {
                row.create_button(|b| b.label("View xkcd image page").style(serenity::ButtonStyle::Link).url(page))
                    .create_button(|b| b.label("View explanation").style(serenity::ButtonStyle::Link).url(wiki))
            });
            c
        });
        message
    })
    .await?;

    Ok(())
}
