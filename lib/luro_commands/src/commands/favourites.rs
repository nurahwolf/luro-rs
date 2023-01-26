use std::collections::{hash_map::Entry, HashMap};

use luro_core::{
    favourites::{Favorite, Favs},
    Context, Error, FAVOURITES_FILE_PATH
};
use poise::serenity_prelude::Message;
use crate::commands::favourites::embed::embed;
use crate::commands::favourites::get::get;
use crate::commands::favourites::list::list;
use crate::commands::favourites::change_category::change_category;
use crate::commands::favourites::remove::remove;

mod get;
mod embed;
mod change_category;
mod list;
mod remove;

/// Add a message as a 'favorite', allowing you to recall things you love!
#[poise::command(context_menu_command = "Add to favs", slash_command, category = "Favs", subcommands("get", "list", "change_category", "remove"))]
pub async fn favourites(ctx: Context<'_>, message: Message) -> Result<(), Error> {
    let accent_colour = ctx.data().config.read().await.accent_colour;
    let category = String::from("uncategorised");

    let new_favourite = vec![Favorite {
        message_id: message.id.0,
        channel_id: message.channel_id.0
    }];
    let new_category = HashMap::from([(category.clone(), new_favourite.clone())]);
    let favourites = &mut ctx.data().user_favourites.write().await;
    // This is hardcoded to get to the 'uncatagorised' category
    let user_favourites = match favourites.favs.entry(ctx.author().id.to_string()) {
        Entry::Occupied(occupied) => occupied.into_mut().entry(category),
        Entry::Vacant(vacant) => vacant.insert(new_category).entry(category)
    };

    // Make sure we have something in the hashset, if not add it
    let uncategorised_favourites_length = match user_favourites {
        Entry::Occupied(mut occupied) => {
            occupied.get_mut().append(&mut vec![Favorite {
            message_id: message.id.0,
            channel_id: message.channel_id.0
        }]);
        // We are after the 'ID', not the human readable length. Arrays start at 0 after all ;)
        occupied.get().len() - 1
    },
        Entry::Vacant(vacant) => vacant.insert(new_favourite).len() - 1
    };

    // Get the new vector length, so we know the ID
    Favs::write(favourites, FAVOURITES_FILE_PATH).await;

    let embed = embed(&message, accent_colour, ctx.guild(), uncategorised_favourites_length, false, ctx.serenity_context().cache.clone());

    ctx.send(|builder| {
        builder.embed(|e| {
            *e = embed;
            e
        })
    })
    .await?;

    Ok(())
}

