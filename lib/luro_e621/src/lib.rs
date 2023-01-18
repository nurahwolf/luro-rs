mod e621_structs;
mod functions;

use crate::functions::e621_description;
use crate::functions::{components, e621_client, embed, E621ComponentData};
use e621_structs::E621Post;
use luro_core::{Command, Context, Error};
use luro_utilities::guild_accent_colour;
use poise::{
    futures_util::StreamExt,
    serenity_prelude::{CreateComponents, InteractionResponseType}
};
use rand::{seq::IteratorRandom, thread_rng};
use std::{fmt::Write, time::Duration};
use urlencoding::encode;

pub fn random_remove(input: &mut Vec<E621Post>) -> Option<E621Post> {
    let i = (0..input.len()).choose(&mut thread_rng())?;
    Some(input.swap_remove(i))
}

pub fn e621_commands() -> [Command; 1] {
    [e621_command()]
}

/// Search e621 you lewd thing
#[poise::command(slash_command, prefix_command, nsfw_only, category = "Furry")]
async fn e621_command(
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
    let config = ctx.data().config.read().await;
    let colour = guild_accent_colour(config.accent_colour, ctx.guild());

    let mut e621_posts = match e621_client(
        config.e621_useragent.clone(),
        ctx.data().secrets.e621_token.clone(),
        disable_blacklist,
        search.clone(),
        config.e621_blacklist.clone()
    )
    .await
    {
        Ok(e621_posts) => e621_posts,
        Err(err) => {
            ctx.say(format!("Failed to resolve posts because of the following reason - {err}"))
                .await?;
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
    let mut e621_component_data = E621ComponentData {
        e621_post_link: String::new(),
        e621_search_tags_link: String::new(),
        sources: vec![String::new()]
    };

    if disable_blacklist {
        writeln!(content_description, "**Blacklist set:** {}", config.e621_blacklist.clone())?;
    }

    let posts_selected = if tiled { 4 } else { 1 };

    for _ in 0..posts_selected {
        if let Some(post) = random_remove(&mut e621_posts.posts) {
            let e621_link = format!("https://e621.net/posts/{}", post.id);
            let search_encoded = encode(&search);
            let search_string = &search_encoded.replace(' ', "+");
            let search_tags = format!("https://e621.net/posts?tags={}", &search_string);
            e621_component_data = E621ComponentData {
                e621_post_link: e621_link,
                e621_search_tags_link: search_tags,
                sources: post.sources.clone()
            };
            comp = components(false, e621_component_data.clone());

            let description = match e621_description(&post, disable_blacklist, config.e621_blacklist.clone()) {
                Ok(ok) => ok,
                Err(err) => panic!("E621: Failed to create description - {err}")
            };

            let embed = embed(
                &search.clone(),
                &description,
                colour,
                &post.file.url,
                no_description,
                search_string,
                &post
            );
            embeds.push(embed);

            writeln!(content_description, "{}", post.file.url)?;
        }
    }

    let reply_handle = ctx
        .send(|builder| {
            builder.components(|c| {
                *c = comp.clone();
                c
            });

            if just_image {
                builder.content(content_description.clone());
            } else {
                for embed in embeds.clone() {
                    builder.embed(|f| {
                        *f = embed;
                        f
                    });
                }
            }
            builder
        })
        .await?;

    let mut interaction_stream = reply_handle
        .message()
        .await?
        .await_component_interactions(ctx)
        .timeout(Duration::from_secs(60 * 3))
        .build();

    // Act on our interaction context
    while let Some(interaction) = interaction_stream.next().await {
        interaction
            .create_interaction_response(ctx, |f| f.kind(InteractionResponseType::UpdateMessage))
            .await?;

        if interaction.data.custom_id.contains("random") {
            embeds.clear();
            for _ in 0..posts_selected {
                if let Some(post) = random_remove(&mut e621_posts.posts) {
                    let e621_link = format!("https://e621.net/posts/{}", post.id);
                    let search_encoded = encode(&search);
                    let search_string = &search_encoded.replace(' ', "+");
                    let search_tags = format!("https://e621.net/posts?tags={}", &search_string);
                    let e621_component_data = E621ComponentData {
                        e621_post_link: e621_link,
                        e621_search_tags_link: search_tags,
                        sources: post.sources.clone()
                    };
                    comp = components(false, e621_component_data.clone());

                    let description = match e621_description(&post, disable_blacklist, config.e621_blacklist.clone()) {
                        Ok(ok) => ok,
                        Err(err) => panic!("E621: Failed to create description - {err}")
                    };

                    let embed = embed(
                        &search.clone(),
                        &description,
                        colour,
                        &post.file.url,
                        no_description,
                        search_string,
                        &post
                    );
                    embeds.push(embed);

                    writeln!(content_description, "{}", post.file.url)?;
                }
            }

            reply_handle
                .edit(ctx, |builder| {
                    builder.components(|c| {
                        *c = comp.clone();
                        c
                    });

                    if just_image {
                        builder.content(content_description.clone());
                    } else {
                        for embed in embeds.clone() {
                            builder.embed(|f| {
                                *f = embed;
                                f
                            });
                        }
                    }
                    builder
                })
                .await?;
        }
    }

    reply_handle
        .edit(ctx, |builder| {
            builder.components(|c| {
                comp = components(true, e621_component_data.clone());
                *c = comp.clone();
                c
            })
        })
        .await?;

    Ok(())
}
