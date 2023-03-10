use luro_core::{Context, Error};
use luro_utilities::guild_accent_colour;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Response {
    #[serde(rename = "list")]
    definitions: Vec<Definition>
}

#[derive(Debug, Deserialize)]
pub struct Definition {
    #[serde(rename = "definition")]
    description: String,
    example: String,
    word: String,
    thumbs_up: usize,
    thumbs_down: usize,
    permalink: String
}

/// Facts from the Urban Dictionary
#[poise::command(slash_command, prefix_command, category = "API")]
pub async fn urban(ctx: Context<'_>, #[description = "Search Term"] search: String) -> Result<(), Error> {
    if search.is_empty() {
        ctx.say("You did not provide a word to look up. Please provide one.").await?;
        return Ok(());
    }
    let accent_colour = ctx.data().config.read().await.accent_colour;

    let client = reqwest::Client::new();
    let request = client
        .get("https://api.urbandictionary.com/v0/define")
        .query(&[("term", &search)])
        .send()
        .await?;
    let response: Response = request.json().await?;

    if response.definitions.is_empty() {
        ctx.say(format!("No definitions found for `{}`. Try a different word.", &search))
            .await?;
        return Ok(());
    }

    let definition = match response.definitions.get(0) {
        Some(ok) => ok,
        None => {
            ctx.say("Failed to find any definitions! Sorry").await?;
            return Ok(());
        }
    };

    let word = &definition.word;
    let description = &definition.description;
    let example = &definition.example;
    let permalink = &definition.permalink;
    let thumbs_up = &definition.thumbs_up;
    let thumbs_down = &definition.thumbs_down;

    let rating = format!("{thumbs_up} 👍 | {thumbs_down} 👎");

    ctx.send(|message| {
        message.embed(|embed| {
            embed.author(|a| a.name(word).url(permalink));
            embed.colour(guild_accent_colour(accent_colour, ctx.guild()));
            embed.description(format!("*{description}*\n\n{example}\n\n**Ratings**: {rating}"));
            embed.footer(|f| f.text("Powered by the Urban Dictionary."))
        })
    })
    .await?;

    Ok(())
}

/// Facts from the Urban Dictionary, but randomly chosen! Spooky
#[poise::command(slash_command, prefix_command, category = "API")]
pub async fn random_urban(ctx: Context<'_>) -> Result<(), Error> {
    let accent_colour = ctx.data().config.read().await.accent_colour;

    let client = reqwest::Client::new();
    let request = client.get("http://api.urbandictionary.com/v0/random").send().await?;
    let response: Response = request.json().await?;
    let definition = match response.definitions.get(0) {
        Some(ok) => ok,
        None => {
            ctx.say("Failed to find any definitions! Sorry").await?;
            return Ok(());
        }
    };

    let word = &definition.word;
    let description = &definition.description;
    let example = &definition.example;
    let permalink = &definition.permalink;
    let thumbs_up = &definition.thumbs_up;
    let thumbs_down = &definition.thumbs_down;

    let rating = format!("{thumbs_up} 👍 | {thumbs_down} 👎");

    ctx.send(|message| {
        message.embed(|embed| {
            embed.author(|a| a.name(word).url(permalink));
            embed.colour(guild_accent_colour(accent_colour, ctx.guild()));
            embed.description(format!("*{description}*\n\n{example}\n\n**Ratings**: {rating}"));
            embed.footer(|f| f.text("Powered by the Urban Dictionary."))
        })
    })
    .await?;

    Ok(())
}
