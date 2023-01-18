use chrono::DateTime;
use luro_core::Error;
use poise::serenity_prelude::{ButtonStyle, Colour, CreateComponents, CreateEmbed};
use std::fmt::Write;

use crate::e621_structs::{E621Post, E621Posts};

pub fn e621_description(e621_post: &E621Post, disable_blacklist: bool, blacklist: String) -> Result<String, Error> {
    let mut description = String::new();

    // Create detailed description for the response
    if e621_post.comment_count.is_positive() {
        writeln!(description, "**Comment count**: {}", e621_post.comment_count)?;
    }
    if !e621_post.created_at.is_empty() {
        let time = DateTime::parse_from_rfc3339(&e621_post.created_at).unwrap();
        writeln!(description, "**Created at**: <t:{}>", time.timestamp())?;
    }
    if !e621_post.updated_at.is_empty() {
        let time = DateTime::parse_from_rfc3339(&e621_post.updated_at).unwrap();
        writeln!(description, "**Uploaded at**: <t:{}>", time.timestamp())?;
    }
    if e621_post.has_notes {
        writeln!(description, "**Has notes**: {}", e621_post.has_notes)?;
    }
    writeln!(description, "**Artist**: {}", e621_post.tags.artist.join(", "))?;
    writeln!(description, "**Character**: {}", e621_post.tags.character.join(", "))?;
    writeln!(description, "**General tags**: {}", e621_post.tags.general.join(", "))?;
    writeln!(description, "**Species**: {}", e621_post.tags.species.join(", "))?;
    if !e621_post.tags.lore.is_empty() {
        writeln!(description, "**Lore**: {}", e621_post.tags.lore.join(", "))?;
    }
    writeln!(description, "**Meta**: {}", e621_post.tags.meta.join(", "))?;
    if disable_blacklist {
        writeln!(description, "**Blacklisted tags**: {blacklist}")?;
    }

    Ok(description)
}

pub async fn e621_client(
    user_agent_string: String,
    token: Option<String>,
    disable_blacklist: bool,
    mut search: String,
    blacklist: String
) -> Result<E621Posts, reqwest::Error> {
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

pub fn embed(
    title: &String,
    description: &String,
    colour: Colour,
    file_url: &String,
    no_description: bool,
    search_terms: &String,
    e621_post: &E621Post
) -> CreateEmbed {
    let mut embed = CreateEmbed::default();
    embed.title(title);
    embed.url(format!("https://e621.net/posts?tags={search_terms}"));
    embed.colour(colour);
    embed.image(file_url);
    if !no_description {
        embed.description(description);
    }
    embed.footer(|footer| {
        footer.text(format!(
            "{} 👍 {} 👎 {} ❤️",
            e621_post.score.up, e621_post.score.down, e621_post.fav_count
        ))
    });
    embed
}

#[derive(Clone)]
pub struct E621ComponentData {
    pub e621_post_link: String,
    pub e621_search_tags_link: String,
    pub sources: Vec<String>
}

pub fn components(disabled: bool, mut e621_component_data: E621ComponentData) -> CreateComponents {
    let mut components = CreateComponents::default();
    components.create_action_row(|row| {
        row.create_button(|button| {
            button
                .label("Another random image")
                .style(ButtonStyle::Primary)
                .custom_id("random")
                .disabled(disabled)
        });

        row.create_button(|button| {
            button
                .label("View on E621")
                .style(ButtonStyle::Link)
                .url(e621_component_data.e621_post_link)
        });

        row.create_button(|button| {
            button
                .label("Search E621")
                .style(ButtonStyle::Link)
                .url(e621_component_data.e621_search_tags_link)
        });

        if !e621_component_data.sources.is_empty() {
            for n in 0..2 {
                if !e621_component_data.sources.is_empty() {
                    row.create_button(|button| {
                        button
                            .label(format!("Source {}", n + 1))
                            .style(ButtonStyle::Link)
                            .url(e621_component_data.sources.pop().unwrap())
                    });
                }
            }
        }

        row
    });

    // Fill the rest of the rows with sources
    for _ in 0..4 {
        if !e621_component_data.sources.is_empty() {
            components.create_action_row(|row| {
                for n in 0..5 {
                    if !e621_component_data.sources.is_empty() {
                        row.create_button(|button| {
                            button
                                .label(format!("Source {}", n + 1))
                                .style(ButtonStyle::Link)
                                .url(e621_component_data.sources.pop().unwrap())
                        });
                    }
                }

                row
            });
        }
    }

    components
}
