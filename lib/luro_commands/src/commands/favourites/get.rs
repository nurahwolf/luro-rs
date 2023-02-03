use std::{collections::hash_map::Entry, time::Duration};

use futures::{Stream, StreamExt};
use luro_core::{
    favourites::{Favorite, Favs},
    Context, Data, Error, FAVOURITES_FILE_PATH, TIMEOUT_DURIATION
};
use poise::{serenity_prelude::InteractionResponseType, CreateReply};
use rand::Rng;

use crate::commands::favourites::{
    constants::{favorite_manipulation_row, favourite_categories_row, initial_menu_row, reply_builder},
    embed::embed
};

async fn autocomplete_category<'a>(ctx: Context<'_>, partial: &'a str) -> impl Stream<Item = String> + 'a {
    // Get favourites and accent_colour from datastore / config
    let favourites = ctx.data().user_favourites.read().await.favs.clone();

    let user_favourites = match favourites.get(&ctx.author().id.to_string()) {
        Some(ok) => ok.to_owned(),
        None => panic!("No user favourites!")
    };

    futures::stream::iter(user_favourites)
        .filter(move |(category, _)| futures::future::ready(category.starts_with(partial)))
        .map(|(category, _)| category)
}

async fn get_favourite(
    data: &Data,
    author_id: &String,
    category: &String,
    id: Option<usize>
) -> Result<(Favorite, usize), String> {
    let favourites = &data.user_favourites.read().await.favs;

    // Get favorites from author
    let user_favourites = match favourites.get(author_id) {
        Some(ok) => ok,
        None => {
            return Err("Looks like you don't have any favorites saved yet!".into());
        }
    };

    let favourites = match user_favourites.get(category) {
        Some(ok) => ok,
        None => return Err("Looks like you don't have any favorites with the category `{category}`.".into())
    };

    if favourites.is_empty() {
        return Err("You have no favourites in that category, sorry!".into());
    }

    // If a favorite is specified, get it, otherwise get a random one
    let cursor = match id {
        Some(fav_id) => fav_id,
        None => rand::thread_rng().gen_range(0..favourites.len())
    };
    let fav = match favourites.get(cursor) {
        Some(user_favorites) => user_favorites.clone(),
        None => return Err("Seems there is no favorite with that ID, sorry!".into())
    };
    Ok((fav, cursor))
}

/// Get a message from your favorites.
#[poise::command(slash_command, category = "Favourites")]
pub async fn get(
    ctx: Context<'_>,
    #[description = "The category of favourite to get."]
    #[autocomplete = "autocomplete_category"]
    category: String,
    id: Option<usize>,
    #[description = "Show some extra information about the favourite"]
    #[flag]
    advanced: bool
) -> Result<(), Error> {
    // Get favourites and accent_colour from datastore / config
    let accent_colour = ctx.data().config.read().await.accent_colour;
    let author_id = ctx.author().id.to_string();

    let (favorite, cursor) = match get_favourite(ctx.data(), &author_id, &category, id).await {
        Ok(ok) => ok,
        Err(err) => {
            ctx.say(err).await?;
            return Ok(());
        }
    };

    // Attempt to resolve the message
    let message = match ctx
        .serenity_context()
        .http
        .get_message(favorite.channel_id, favorite.message_id)
        .await
    {
        Ok(message) => message,
        Err(err) => {
            ctx.say(format!(
                "I'm afraid I could not get the original message! This might be because the message was deleted, I don't have access to that channel or I'm no longer in the server the message was sent.\n{err}"
            ))
            .await?;
            return Ok(());
        }
    };

    let embed = embed(
        &message,
        accent_colour,
        ctx.guild(),
        cursor,
        advanced,
        ctx.serenity_context().cache.clone(),
        category.clone()
    );

    let favourite_categories_row = match favourite_categories_row(ctx.data(), &author_id).await {
        Ok(ok) => ok,
        Err(err) => {
            ctx.say(err).await?;
            return Ok(());
        }
    };

    let mut reply_builder = reply_builder(embed.clone());
    // This is to include other embeds in the favourite message, such as twitter embeds.
    for embed in message.clone().embeds {
        // Don't attach the embed if it has no title
        // This is usually Discord turning a link into an embed, which by default formats to a small image.
        // Additionally, there is the check above to include the link in the primary embed.
        if embed.title.is_some() | embed.author.is_some() {
            reply_builder.embed(|e| {
                *e = embed.into();
                e
            });
        }
    }

    let reply_handle = ctx
        .send(|builder| {
            *builder = reply_builder;
            builder
        })
        .await?;

    let mut interaction_stream = reply_handle
        .message()
        .await?
        .await_component_interactions(ctx)
        .timeout(Duration::from_secs(TIMEOUT_DURIATION))
        .build();

    // Act on our interaction context
    while let Some(interaction) = interaction_stream.next().await {
        interaction
            .create_interaction_response(ctx, |f| f.kind(InteractionResponseType::DeferredUpdateMessage))
            .await?;

        // Interactor is NOT the author, so terminate
        if &interaction.user != ctx.author() {
            interaction
                .create_followup_message(ctx, |message| {
                    message
                        .content("Can't interact with someone elses favourite!")
                        .ephemeral(true)
                })
                .await?;
            break;
        }
        if interaction.data.custom_id.contains("show_menu") {
            reply_handle
                .edit(ctx, |builder| {
                    builder.components(|components| {
                        components
                            .add_action_row(favourite_categories_row.clone())
                            .add_action_row(favorite_manipulation_row())
                    })
                })
                .await?;
        };

        if interaction.data.custom_id.contains("close_menu") {
            reply_handle
                .edit(ctx, |builder| {
                    builder.components(|c| {
                        *c = initial_menu_row();
                        c
                    })
                })
                .await?;
        };

        if interaction.data.custom_id.contains("menu") {
            if let Some(new_category) = interaction.data.values.first() {
                match move_favourite(ctx.data(), favorite.clone(), new_category.clone(), author_id.clone()).await {
                    Ok(cursor) => {
                        reply_handle
                            .edit(ctx, |reply| {
                                reply.embed(|e| {
                                    *e = embed.clone();
                                    e.footer(|footer| footer.text(format!("Fav ID: {cursor}    Category: {new_category}")))
                                })
                            })
                            .await?;
                    }
                    Err(err) => {
                        ctx.say(err).await?;
                    }
                };
            }
        }

        if interaction.data.custom_id.contains("remove") {
            reply_handle.delete(ctx).await?;
        }

        if interaction.data.custom_id.contains("delete") {
            delete_favourite(ctx.data(), author_id.clone(), cursor, &category.clone()).await?;
            reply_handle
                .edit(ctx, |builder| builder.content("Favourite deleted!"))
                .await?;
        }
    }

    Ok(())
}

async fn delete_favourite(data: &Data, author_id: String, id: usize, category: &String) -> Result<(), String> {
    let favourites_db = &mut data.user_favourites.write().await;
    // Get favorites from author
    let user_favourites = match favourites_db.favs.get_mut(&author_id) {
        Some(ok) => ok,
        None => {
            return Err("Looks like you don't have any favorites saved yet!".into());
        }
    };

    let favourites = match user_favourites.get_mut(category) {
        Some(ok) => ok,
        None => return Err("Looks like you don't have any favorites with the category `{category}`.".into())
    };

    favourites.remove(id);
    Favs::write(favourites_db, FAVOURITES_FILE_PATH).await;
    Ok(())
}

async fn move_favourite(data: &Data, favourite: Favorite, to: String, author_id: String) -> Result<usize, String> {
    let favourites_db = &mut data.user_favourites.write().await;
    // Get favorites from author
    let user_favourites = match favourites_db.favs.get_mut(&author_id) {
        Some(ok) => ok,
        None => {
            return Err("Looks like you don't have any favorites saved yet!".into());
        }
    };

    // Now move to the new category
    let new_favourite_length = match user_favourites.entry(to) {
        Entry::Occupied(occupied) => {
            let vector_length = occupied.get().len();
            occupied.into_mut().append(&mut vec![favourite.clone()]);
            vector_length
        }
        Entry::Vacant(vacant) => {
            let inserted = vacant.insert(vec![favourite.clone()]);
            inserted.len()
        }
    };

    // Save to DB
    Favs::write(favourites_db, FAVOURITES_FILE_PATH).await;
    Ok(new_favourite_length)
}
