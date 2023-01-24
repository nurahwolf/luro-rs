use std::collections::hash_map::Entry;

use luro_core::{
    favorites::{Favorite, Favs},
    Context, Error, FAVORITES_FILE_PATH
};
use luro_utilities::guild_accent_colour;
use poise::serenity_prelude::Message;
use rand::Rng;

/// Get a message from your favorites.
#[poise::command(slash_command, category = "Favs")]
async fn get(
    ctx: Context<'_>,
    id: Option<usize>,
    #[description = "Hide advanced information"]
    #[flag]
    hide: bool
) -> Result<(), Error> {
    // Get favorites and accent_colour from datastore / config
    let favorites = &ctx.data().user_favorites.read().await.favs;
    let accent_colour = ctx.data().config.read().await.accent_colour;

    // Get favorites from author
    let user_favorites = match favorites.get(&ctx.author().id.to_string()) {
        Some(user_favorites) => user_favorites,
        None => {
            ctx.say("Looks like you don't have any favorites saved yet!").await?;
            return Ok(());
        }
    };

    // If a favorite is specified, get it, otherwise get a random one
    let cursor = match id {
        Some(fav_id) => fav_id,
        None => rand::thread_rng().gen_range(0..user_favorites.len())
    };
    let favorite = match user_favorites.get(cursor) {
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

    // Message resolved, send it!
    ctx.send(|builder| {
        builder.embed(|embed| {
            embed
                .author(|author| {
                    author
                        .name(&message.author.name)
                        .icon_url(&message.author.avatar_url().unwrap_or_default())
                })
                .title("Message Link")
                .url(message.link())
                .color(guild_accent_colour(accent_colour, ctx.guild()))
                .description(&message.content)
                .footer(|footer| footer.text(format!("Fav ID: {cursor}")));

            if !hide {
                embed.field("Message ID", message.id, true);
                if let Some(guild_id) = message.guild_id {
                    embed.field("Guild ID", guild_id, true);
                }
                embed.field("Author", format!("{} (ID: {})", message.author, message.author.id), true);
            }

            if !message.attachments.is_empty() {
                if let Some(attachment) = message.attachments.first() {
                    embed.attachment(attachment.url.clone());
                }
            }

            if let Some(guild) = message.guild(ctx) {
                embed.footer(|footer| footer.icon_url(guild.icon_url().unwrap_or_default()).text(guild.name));
            };

            embed
        })
    })
    .await?;

    Ok(())
}

/// Add a message as a 'favorite', allowing you to recall things you love!
#[poise::command(context_menu_command = "Add to favs", slash_command, category = "Favs", subcommands("get"))]
pub async fn fav(ctx: Context<'_>, message: Message) -> Result<(), Error> {
    let accent_colour = ctx.data().config.read().await.accent_colour;

    // Write to disk
    let new_fav = &mut vec![Favorite {
        message_id: message.id.0,
        channel_id: message.channel_id.0
    }];
    let favorites = &mut ctx.data().user_favorites.write().await;
    let user_favs = match favorites.favs.entry(ctx.author().id.to_string()) {
        Entry::Occupied(occupied) => occupied.into_mut(),
        Entry::Vacant(vacant) => vacant.insert(new_fav.to_vec())
    };

    user_favs.append(new_fav);
    let user_favs_length = user_favs.len();
    Favs::write(favorites, FAVORITES_FILE_PATH).await;

    ctx.send(|builder| {
        builder.embed(|embed| {
            embed
                .author(|author| {
                    author
                        .name(&message.author.name)
                        .icon_url(&message.author.avatar_url().unwrap_or_default())
                })
                .title("Message Link")
                .url(message.link())
                .color(guild_accent_colour(accent_colour, ctx.guild()))
                .description(&message.content)
                .footer(|footer| footer.text(format!("Fav ID: {user_favs_length}")));

            if !message.attachments.is_empty() {
                if let Some(attachment) = message.attachments.first() {
                    embed.attachment(attachment.url.clone());
                }
            }

            embed.field("Message ID", message.id, true);
            if let Some(guild_id) = message.guild_id {
                embed.field("Guild ID", guild_id, true);
            }
            embed.field("Author", format!("{} (ID: {})", message.author, message.author.id), true);

            if let Some(guild) = message.guild(ctx) {
                embed.footer(|footer| footer.icon_url(guild.icon_url().unwrap_or_default()).text(guild.name));
            };

            embed
        })
    })
    .await?;

    Ok(())
}
