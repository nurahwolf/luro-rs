use crate::{functions::guild_accent_colour::guild_accent_colour, Context, Error};
use poise::serenity_prelude::{self as serenity};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct LodestoneNews {
    id: String,
    url: String,
    title: String,
    time: String,
    image: String,
    description: String
}

/// Fetch the latest news from the Lodestone API!
#[poise::command(slash_command, prefix_command)]
pub async fn lodestonenews(ctx: Context<'_>) -> Result<(), Error> {
    let accent_colour = ctx.data().config.read().await.accent_colour;
    let client = reqwest::Client::new();
    let request = client.get("https://lodestonenews.com/news/topics?locale=gb").send().await?;
    let response: Vec<LodestoneNews> = request.json().await?;

    let newsitem = match response.get(1) {
        Some(ok) => ok,
        None => {
            ctx.say("Failed to get any news :(").await?;
            return Ok(());
        }
    };

    ctx.send(|message| {
        message.embed(|embed| {
            embed.colour(guild_accent_colour(accent_colour, ctx.guild()));
            embed.title(&newsitem.title);
            embed.description(newsitem.description.to_string());
            embed.image(&newsitem.image);
            embed.footer(|f| f.text("Powered by https://lodestonenews.com"))
        });
        message.components(|c| {
            c.create_action_row(|row| {
                row.create_button(|b| {
                    b.label("View lodestone")
                        .style(serenity::ButtonStyle::Link)
                        .url(&newsitem.url)
                })
            });
            c
        });
        message
    })
    .await?;

    Ok(())
}
