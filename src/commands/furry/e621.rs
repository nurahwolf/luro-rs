use crate::functions::guild_accent_colour::guild_accent_colour;
use crate::functions::random_remove::random_remove;
use crate::structs::e621::{E621Posts, E621Post};
use crate::{Context, Error};

use chrono::DateTime;
use poise::serenity_prelude::{ButtonStyle, Colour, CreateActionRow, CreateButton, CreateComponents, CreateEmbed};
use rand::seq::SliceRandom;
use reqwest::Response;
use std::fmt::Write;
use std::vec;
use urlencoding::encode;

fn e621_description(e621_post: &E621Post, disable_blacklist: bool, blacklist: String, search: String) -> String {
    let mut description = String::new();
    let rating = format!("{} 👍 {} 👎 {} ❤️", e621_post.score.up, e621_post.score.down, e621_post.fav_count);


    // Create detailed description for the response
    if e621_post.comment_count.is_positive() {
        writeln!(description, "**Comment count**: {}", e621_post.comment_count);
    }
    if !e621_post.created_at.is_empty() {
        let time = DateTime::parse_from_rfc3339(&e621_post.created_at).unwrap();
        writeln!(description, "**Created at**: <t:{}>", time.timestamp());
    }
    if !e621_post.updated_at.is_empty() {
        let time = DateTime::parse_from_rfc3339(&e621_post.updated_at).unwrap();
        writeln!(description, "**Uploaded at**: <t:{}>", time.timestamp());
    }
    if e621_post.has_notes {
        writeln!(description, "**Has notes**: {}", e621_post.has_notes);
    }
    writeln!(description, "**Artist**: {}", e621_post.tags.artist.join(", "));
    writeln!(description, "**Character**: {}", e621_post.tags.character.join(", "));
    writeln!(description, "**General tags**: {}", e621_post.tags.general.join(", "));
    writeln!(description, "**Species**: {}", e621_post.tags.species.join(", "));
    if !e621_post.tags.lore.is_empty() {
        writeln!(description, "**Lore**: {}", e621_post.tags.lore.join(", "));
    }
    writeln!(description, "**Meta**: {}", e621_post.tags.meta.join(", "));
    if disable_blacklist {
        writeln!(description, "**Blacklisted tags**: {}", blacklist);
    }

    description
}

async fn e621_client(user_agent_string: String, token: Option<String>, disable_blacklist: bool, mut search: String, blacklist: String) -> Result<E621Posts, reqwest::Error> {
    let client = match reqwest::Client::builder().user_agent(user_agent_string).build() {
        Ok(client) => client,
        Err(err) => panic!("E621: Failed to create request builder - {err}")
    };
    let search_query = if disable_blacklist {
        search.clone()
    } else {
        search.push_str(&blacklist);
        search
    };

    let response = match client
        .get("https://e621.net/posts.json")
        .basic_auth("nurahwolf", token)
        .query(&[("tags", &search_query)])
        .send()
        .await
    {
        Ok(response) => response,
        Err(err) => panic!("E621: Failed to resolve response - {err}")
    };

    response.json::<E621Posts>().await
}

/// Search e621 you lewd thing
#[poise::command(slash_command, prefix_command, nsfw_only, category = "Furry")]
pub async fn e621(
    ctx: Context<'_>,
    #[description = "Just send the image result"]
    #[flag]
    just_image: bool,
    #[description = "Disable Blacklist"]
    #[flag]
    disable_blacklist: bool,
    #[description = "Enable tiled image display"]
    #[flag]
    tiled: bool,
    #[description = "Remove the description from the embed"]
    #[flag]
    no_description: bool,
    #[description = "Search Term"]
    #[rest]
    search: String
) -> Result<(), Error> {
    // Bail if there is no search term
    if search.is_empty() {
        ctx.say("You did not provide something to search.").await?;
        return Ok(());
    }
    let config = ctx.data().config.lock().unwrap().clone();
    let colour = guild_accent_colour(config.accent_colour, ctx.guild());

    let mut e621_posts = match e621_client(config.e621_useragent, ctx.data().secrets.e621_token.clone(), disable_blacklist, search.clone(), config.e621_blacklist.clone()).await {
        Ok(e621_posts) => e621_posts,
        Err(err) => {
            ctx.say(format!("Failed to resolve posts because of the following reason - {err}")).await?;
            return Ok(());
        }
    };

    // Bail if no posts in response
    if e621_posts.posts.is_empty() {
        ctx.say("Your search returned fuck all.").await?;
        return Ok(());
    }

    let mut embeds = vec![];
    let mut content_description = String::new();
    let mut comp = CreateComponents::default();

    if disable_blacklist {
        writeln!(content_description, "**Blacklist set:** {}", config.e621_blacklist.clone());
    }



    let posts_selected = if tiled {
        4
    } else {
        1
    };

    for _ in 0..posts_selected {
        if let Some(post) = random_remove(&mut e621_posts.posts) {
            let description = e621_description(&post, disable_blacklist, config.e621_blacklist.clone(), search.clone());
            let embed = embed(&search.clone(), &description, colour, &post.file.url, no_description);
            embeds.push(embed);

            let e621_link = format!("https://e621.net/posts/{}", post.id);
            let search_encoded = encode(&search);
            let search_string = &search_encoded.replace(' ', "+");
            let search_tags = format!("https://e621.net/posts?tags={}", &search_string);
            let e621_component_data = E621ComponentData { e621_post_link: e621_link, e621_search_tags_link: search_tags, sources: post.sources.clone() };
            comp = components(false, e621_component_data);
            
            writeln!(content_description, "{}", post.file.url);
        }
    }

    ctx.send(|builder|{
        builder.components(|c|{
            *c = comp;
            c
        });

        if just_image {
            builder.content(content_description);
        } else {
            for embed in embeds {
                builder.embed(|f| {
                    *f = embed;
                    f
                });
            }
        }
        builder
    }).await?;

    Ok(())
}

fn embed(title: &String, description: &String, colour: Colour, file_url: &String, no_description: bool) -> CreateEmbed {
    let mut embed = CreateEmbed::default();
    embed.title(title);
    embed.url("https://e621.net/posts/");
    embed.colour(colour);
    embed.image(file_url);
    if !no_description {
        embed.description(description);
    }
    embed
}

pub struct E621ComponentData {
    e621_post_link: String,
    e621_search_tags_link: String,
    sources: Vec<String>
}

pub fn components(disabled: bool, mut e621_component_data: E621ComponentData) -> CreateComponents {
    let mut components = CreateComponents::default();
    components.create_action_row(|row| {
        row.create_button(|button| button.label("Previous Image").style(ButtonStyle::Primary).custom_id("prev").disabled(disabled));

        row.create_button(|button| button.label("Next Image").style(ButtonStyle::Primary).custom_id("next").disabled(disabled));

        row.create_button(|button| button.label("View on E621").style(ButtonStyle::Link).url(e621_component_data.e621_post_link));

        row.create_button(|button| button.label("Search E621").style(ButtonStyle::Link).url(e621_component_data.e621_search_tags_link));

        if !e621_component_data.sources.is_empty() {
            row.create_button(|button| button.label("Source 0").style(ButtonStyle::Link).url(e621_component_data.sources.pop().unwrap()));
        }

        row
    });

    // Fill the rest of the rows with sources
    for _ in 0..4 {
        if !e621_component_data.sources.is_empty() {
            components.create_action_row(|row| {
                for n in 0..5 {
                    if !e621_component_data.sources.is_empty() {
                        row.create_button(|button| button.label(format!("Source {}", n + 1)).style(ButtonStyle::Link).url(e621_component_data.sources.pop().unwrap()));
                    }
                }

                row
            });
        }
    }

    components
}
