use futures::{Stream, StreamExt};
use luro_core::{favourites::Favs, Context, Error, FAVOURITES_FILE_PATH};

use crate::commands::favourites::embed::embed;

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

/// Remove a message from your favorites.
#[poise::command(slash_command, category = "Favourites", ephemeral)]
pub async fn remove(
    ctx: Context<'_>,
    #[description = "The category of favourite to get. Gets 'uncategorised' if not set"]
    #[autocomplete = "autocomplete_category"]
    category: String,
    id: usize
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

    let favourites = match user_favourites.get_mut(&category) {
        Some(ok) => ok,
        None => {
            ctx.say(format!(
                "Looks like you don't have any favorites with the category `{category}`."
            ))
            .await?;
            return Ok(());
        }
    };

    // Make sure the ID actually exists
    let favourite = if id < favourites.len() {
        favourites.remove(id)
    } else {
        ctx.say(format!(
            "No ID of {id} found in {category}. Make sure you are using the Favourite ID and NOT the message ID!"
        ))
        .await?;
        return Ok(());
    };

    // If that category is now empty, remove it
    let removed = if favourites.is_empty() {
        user_favourites.remove(&category);
        true
    } else {
        false
    };

    // Attempt to resolve the message
    let message = match ctx
        .serenity_context()
        .http
        .get_message(favourite.channel_id, favourite.message_id)
        .await
    {
        Ok(message) => message,
        Err(_) => {
            ctx.say(format!(
                "Looks like the original message does not exist, but I did remove the favourite {}",
                favourite.message_id
            ))
            .await?;
            return Ok(());
        }
    };

    let mut embed = embed(
        &message,
        accent_colour,
        ctx.guild(),
        id,
        false,
        ctx.serenity_context().cache.clone(),
        category
    );
    embed.title("Favourite removed");

    // Message resolved, send it!
    ctx.send(|builder| {
        builder.embed(|e| {
            *e = embed;
            e
        });
        // If that category is now empty, remove it
        if removed {
            builder.content("That category is now empty, so I have removed it.");
        };
        builder
    })
    .await?;

    Favs::write(favourites_db, FAVOURITES_FILE_PATH).await;

    Ok(())
}
