use std::{
    collections::{hash_map::Entry, HashMap},
    time::Duration
};

use crate::commands::favourites::embed::embed;
use crate::commands::favourites::get::get;
use crate::commands::favourites::list::list;
use crate::commands::favourites::remove::remove;
use crate::commands::favourites::{
    change_category::change_category,
    constants::{favorite_manipulation_row, favourite_categories_row, initial_menu_row, reply_builder}
};
use futures::StreamExt;
use luro_core::{
    favourites::{Favorite, Favs},
    Context, Data, Error, FAVOURITES_FILE_PATH, TIMEOUT_DURIATION
};
use poise::serenity_prelude::{CreateSelectMenu, CreateSelectMenuOption, InteractionResponseType, Message};

mod change_category;
mod constants;
mod embed;
mod get;
mod list;
mod remove;

/// Save the new favourite to the database
async fn save_new_favourite(data: &Data, new_favourite: Vec<Favorite>, author_id: String) -> usize {
    let favourites_db = &mut data.user_favourites.write().await;
    // This is hardcoded to get to the 'uncatagorised' category by default. Users are permitted to change category later
    let category = String::from("uncategorised");
    let new_category = HashMap::from([(category.clone(), new_favourite.clone())]);

    let user_favourites = match favourites_db.favs.entry(author_id) {
        Entry::Occupied(occupied) => occupied.into_mut().entry(category.clone()),
        Entry::Vacant(vacant) => vacant.insert(new_category).entry(category.clone())
    };

    // Make sure we have something in the hashset, if not add it
    let uncategorised_favourites_length = match user_favourites {
        Entry::Occupied(mut occupied) => {
            occupied.get_mut().append(&mut new_favourite.clone());
            // We are after the 'ID', not the human readable length. Arrays start at 0 after all ;)
            occupied.get().len() - 1
        }
        Entry::Vacant(vacant) => vacant.insert(new_favourite).len() - 1
    };

    Favs::write(favourites_db, FAVOURITES_FILE_PATH).await;
    uncategorised_favourites_length
}

/// Add a message as a 'favorite', allowing you to recall things you love!
#[poise::command(
    context_menu_command = "Add to favs",
    slash_command,
    category = "Favs",
    subcommands("get", "list", "change_category", "remove")
)]
pub async fn favourites(ctx: Context<'_>, message: Message) -> Result<(), Error> {
    let accent_colour = ctx.data().config.read().await.accent_colour;
    let author_id = ctx.author().id.to_string();

    let new_favourite = Favorite {
        message_id: message.id.0,
        channel_id: message.channel_id.0
    };
    let new_favourite_vec = vec![new_favourite.clone()];

    let uncategorised_favourites_length = save_new_favourite(ctx.data(), new_favourite_vec.clone(), author_id.clone()).await;

    let embed = embed(
        &message,
        accent_colour,
        ctx.guild(),
        uncategorised_favourites_length,
        false,
        ctx.serenity_context().cache.clone(),
        "uncategorised".to_string()
    );

    let mut menu = CreateSelectMenu::default();
    menu.custom_id("menu");
    menu.placeholder("Move to a new category");
    if let Some(user_favs) = ctx.data().user_favourites.read().await.favs.get(&author_id) {
        menu.options(|options| {
            for fav in user_favs {
                let mut option = CreateSelectMenuOption::default();
                option.label(fav.0);
                option.value(fav.0);
                options.add_option(option);
            }
            options
        });
    }

    let reply_builder = reply_builder(embed.clone());
    let favourite_categories_row = match favourite_categories_row(ctx.data(), &author_id).await {
        Ok(ok) => ok,
        Err(err) => {
            ctx.say(err).await?;
            return Ok(());
        }
    };

    let reply_handle = ctx
        .send(|b| {
            *b = reply_builder;
            b
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

        if interaction.data.custom_id.contains("menu") && &interaction.user == ctx.author() {
            if let Some(new_category) = interaction.data.values.first() {
                match move_favourite(ctx.data(), new_favourite.clone(), new_category.clone(), author_id.clone()).await {
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
            ctx.say("Not implemented yet!").await?;
        }
    }

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
