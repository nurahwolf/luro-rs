use crate::utils::guild_accent_colour;
use crate::{Context, Error};

use chrono::DateTime;
use poise::serenity_prelude::{ButtonStyle, CreateActionRow, CreateButton};
use rand::seq::SliceRandom;
use std::fmt::Write;
use std::vec;
use urlencoding::encode;

use super::struct_e621::E621Posts;

/// Search e621 you lewd thing
#[poise::command(slash_command, prefix_command, nsfw_only, category = "Furry")]
pub async fn e621(
    ctx: Context<'_>,
    #[description = "Just send the image result"]
    #[flag]
    just_image: bool,
    #[description = "Disable the default Blacklist"]
    #[flag]
    disable_blacklist: bool,
    #[description = "Show blacklisted tags"]
    #[flag]
    show_blacklist: bool,
    #[description = "Search Term"]
    #[rest]
    search: String
) -> Result<(), Error> {
    // Bail if there is no search term
    if search.is_empty() {
        ctx.say("You did not provide something to search.").await?;
        return Ok(());
    }

    let blacklist_string = "-gore -scat ";
    let client = reqwest::Client::builder().user_agent(ctx.data().config.lock().unwrap().e621_useragent.clone()).build()?;
    let token = ctx.data().secrets.e621_token.clone();

    // TODO: Create request builder
    let request = if disable_blacklist {
        client
            .get("https://e621.net/posts.json?")
            .basic_auth("nurahwolf", token)
            .query(&[("tags", &search)])
            .send()
            .await?
    } else {
        let mut blacklist_search = blacklist_string.to_string();
        blacklist_search.push_str(&search);

        client
            .get("https://e621.net/posts.json?")
            .basic_auth("nurahwolf", token)
            .query(&[("tags", blacklist_search)])
            .send()
            .await?
    };

    let response = request.json::<E621Posts>().await?;

    // Bail if no posts in response
    if response.posts.is_empty() {
        ctx.say("Your search returned fuck all.").await?;
        return Ok(());
    }

    // Declare variables used for the response
    let mut description = String::new();
    let random = response.posts.choose(&mut rand::thread_rng()).unwrap();
    let rating = format!("{} 👍 {} 👎 {} ❤️", random.score.up, random.score.down, random.fav_count);
    let e621_link = format!("https://e621.net/posts/{}", random.id);
    let search_encoded = encode(&search);
    let search_string = &search_encoded.replace(' ', "+");

    // Create detailed description for the response
    if random.comment_count.is_positive() {
        writeln!(description, "**Comment count**: {}", random.comment_count)?;
    }
    if !random.created_at.is_empty() {
        let time = DateTime::parse_from_rfc3339(&random.created_at).unwrap();
        writeln!(description, "**Created at**: <t:{}>", time.timestamp())?;
    }
    if !random.updated_at.is_empty() {
        let time = DateTime::parse_from_rfc3339(&random.updated_at).unwrap();
        writeln!(description, "**Uploaded at**: <t:{}>", time.timestamp())?;
    }
    if random.has_notes {
        writeln!(description, "**Has notes**: {}", random.has_notes)?;
    }
    writeln!(description, "**Artist**: {}", &random.tags.artist.join(", "))?;
    writeln!(description, "**Character**: {}", &random.tags.character.join(", "))?;
    writeln!(description, "**General tags**: {}", &random.tags.general.join(", "))?;
    writeln!(description, "**Species**: {}", &random.tags.species.join(", "))?;
    if !&random.tags.lore.is_empty() {
        writeln!(description, "**Lore**: {}", &random.tags.lore.join(", "))?;
    }
    writeln!(description, "**Meta**: {}", &random.tags.meta.join(", "))?;
    if !disable_blacklist && show_blacklist {
        writeln!(description, "**Blacklisted tags**: {}", &blacklist_string)?;
    }
    // Button Labels
    let search_e621_label = format!("Search E621 for {}", &search);
    let search_tags = format!("https://e621.net/posts?tags={}", &search_string);

    // Buttons
    let mut view_on_e621_button = CreateButton::default();
    view_on_e621_button.label("View on E621").style(ButtonStyle::Link).url(&e621_link);

    let mut search_on_e621_button = CreateButton::default();
    search_on_e621_button.label(&search_e621_label).style(ButtonStyle::Link).url(search_tags);

    let mut source_buttons = vec![];
    for source in &random.sources {
        let mut button = CreateButton::default();
        button.label("View Source").style(ButtonStyle::Link).url(source);
        source_buttons.push(button);
    }

    // Action Rows
    let mut e621_action_row = CreateActionRow::default();
    e621_action_row.add_button(view_on_e621_button);
    e621_action_row.add_button(search_on_e621_button);
    for _ in 0..3 {
        if !source_buttons.is_empty() {
            e621_action_row.add_button(source_buttons.pop().unwrap());
        }
    }

    let mut source_action_row = CreateActionRow::default();
    for _ in 0..5 {
        if !source_buttons.is_empty() {
            source_action_row.add_button(source_buttons.pop().unwrap());
        }
    }

    let just_image_text = if show_blacklist {
        format!("**Blacklist set:** {}\n{}", blacklist_string, random.file.url.as_str())
    } else {
        random.file.url.to_string()
    };
    // Send just the image, as requested
    if just_image {
        ctx.send(|reply| {
            reply.content(just_image_text).components(|component| {
                component.add_action_row(e621_action_row);
                if !source_action_row.0.is_empty() {
                    component.add_action_row(source_action_row);
                }
                component
            })
        })
        .await?;
        return Ok(());
    }

    // Send an embed
    ctx.send(|reply| {
        reply
            .embed(|embed| {
                embed
                    .title(&search)
                    .color(guild_accent_colour(ctx.data().config.lock().unwrap().accent_colour, ctx.guild()))
                    .image(&random.file.url)
                    .description(description)
                    .footer(|f| f.text(format!("{}\nRatings: {}", random.description, rating)))
            })
            .components(|component| {
                component.add_action_row(e621_action_row);
                if !source_action_row.0.is_empty() {
                    component.add_action_row(source_action_row);
                }
                component
            })
    })
    .await?;
    Ok(())
}
