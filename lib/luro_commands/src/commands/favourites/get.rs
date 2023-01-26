use luro_core::{Context, Error};
use rand::Rng;

use crate::commands::favourites::embed::embed;

/// Get a message from your favorites.
#[poise::command(slash_command, category = "Favourites")]
pub async fn get(
    ctx: Context<'_>,
    id: Option<usize>,
    #[description = "Hide advanced information in the embed"]
    #[flag]
    hide: bool,
    #[description = "The category of favourite to get. Gets 'uncategorised' if not set"] category: Option<String>
) -> Result<(), Error> {
    // Get favourites and accent_colour from datastore / config
    let favourites = &ctx.data().user_favourites.read().await.favs;
    let accent_colour = ctx.data().config.read().await.accent_colour;

    // Get favorites from author
    let user_favourites = match favourites.get(&ctx.author().id.to_string()) {
        Some(ok) => ok,
        None => {
            ctx.say("Looks like you don't have any favorites saved yet!").await?;
            return Ok(());
        }
    };

    // Get the category requested, otherwise fall back to 'uncategorised'
    let category = match category {
        Some(category) => category,
        None => "uncategorised".to_string()
    };

    let favourites =  match user_favourites.get(&category) {
        Some(ok) => ok,
        None => {
            ctx.say(format!("Looks like you don't have any favorites with the category `{category}`.")).await?;
            return Ok(());
        }
    };

    // If a favorite is specified, get it, otherwise get a random one
    let cursor = match id {
        Some(fav_id) => fav_id,
        None => rand::thread_rng().gen_range(0..favourites.len())
    };
    let favorite = match favourites.get(cursor) {
        Some(user_favorites) => user_favorites,
        None => {
            ctx.say("Seems there is no favorite with that ID, sorry!").await?;
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

    // NSFW check - If the channel we are sending to is NOT nsfw, but the content is, don't send.
    if let Ok(author_channel) = ctx.channel_id().to_channel(ctx).await {
        if let Ok(message_channel) = message.channel(ctx).await {
            if !author_channel.is_nsfw() && message_channel.is_nsfw() {
                ctx.say("Stop trying to send something NSFW to a SFW channel, dork!").await?;
                return Ok(());
            }
        }
    };

    let embed = embed(
        &message,
        accent_colour,
        ctx.guild(),
        cursor,
        hide,
        ctx.serenity_context().cache.clone()
    );

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
