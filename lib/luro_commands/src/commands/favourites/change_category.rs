use std::collections::hash_map::Entry;

use luro_core::{Context, Error, favourites::Favs, FAVOURITES_FILE_PATH};

use crate::commands::favourites::embed::embed;

/// Move a message to a category, creating it if it does not exist
#[poise::command(slash_command, category = "Favourites", rename = "move")]
pub async fn change_category(
    ctx: Context<'_>,
    #[description = "The Favourite ID of the favourite you wish to move"]
    id: usize,
    #[description = "The category name of the ID you wish to move"] category_from: String,
    #[description = "The category name you wish to move it to"] category_to: String
) -> Result<(), Error> {
    // Get favourites and accent_colour from datastore / config
    let favourites_db = &mut ctx.data().user_favourites.write().await;
    let accent_colour = ctx.data().config.read().await.accent_colour;

    // Get favorites from author
    let user_favourites = match favourites_db.favs.get_mut(&ctx.author().id.to_string()) {
        Some(ok) => ok,
        None => {
            ctx.say("Looks like you don't have any favorites saved yet!").await?;
            return Ok(());
        }
    };

    // Attempt to resolve the category they want
    let favourites_from = match user_favourites.get_mut(&category_from) {
        Some(ok) => ok,
        None => {
            ctx.say(format!("The category {category_from} does not exist")).await?;
            return Ok(());
        }
    };

    // Make sure the ID actually exists
    let favourite = if id < favourites_from.len() {
        favourites_from.remove(id)
    } else {
        ctx.say(format!("No ID of {id} found in {category_from}. Make sure you are using the Favourite ID and NOT the message ID!")).await?;
        return Ok(())
    };

    // Now move to the new category
    let new_favourite_length = match user_favourites.entry(category_to.clone()) {
        Entry::Occupied(occupied) => {
            let vector_length = occupied.get().len();
            occupied.into_mut().append(&mut vec![favourite.clone()]);
            vector_length
        },
        Entry::Vacant(vacant) => {
            ctx.say(format!("The category {category_to} was created, and your favourite moved.")).await?;
            let inserted = vacant.insert(vec![favourite.clone()]);
            inserted.len()
        }
    };

    // Save to DB
    Favs::write(favourites_db, FAVOURITES_FILE_PATH).await;

    // Attempt to resolve the message
    let message = match ctx
        .serenity_context()
        .http
        .get_message(favourite.channel_id, favourite.message_id)
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

    let mut embed = embed(
        &message,
        accent_colour,
        ctx.guild(),
        new_favourite_length,
        false,
        ctx.serenity_context().cache.clone()
    );
    embed.title(format!("Message moved to {}", category_to.clone()));

    // Message resolved, send it!
    ctx.send(|builder| {
        builder.embed(|e| {
            *e = embed;
            e
        })
    })
    .await?;

    Ok(())
}
