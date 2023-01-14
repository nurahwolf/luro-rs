use poise::{
    serenity_prelude::{CacheHttp, Colour, CreateEmbed, User},
    Modal
};
use rand::Rng;

use crate::{
    data::heck::{Heck, HeckInt},
    functions::guild_accent_colour::guild_accent_colour,
    Context, Error, HECK_FILE_PATH
};

async fn heck_function(author: &User, user: &User, hecks: &Vec<HeckInt>, heck_id: Option<usize>) -> (HeckInt, usize) {
    let heck_id = match heck_id {
        Some(ok) => ok,
        None => {
            let rng = &mut rand::thread_rng();
            rng.gen_range(0..hecks.len())
        }
    };

    match hecks.get(heck_id) {
        Some(heck) => (
            HeckInt {
                heck: heck.heck.replace("<user>", user.to_string().as_str()).replace("<author>", author.to_string().as_str()),
                author_id: heck.author_id
            },
            heck_id
        ),
        None => (
            HeckInt {
                heck: "No hecks found! If you specified an ID, make sure that ID exists. If you randomly tried to get one, make sure `heck.toml` exists within the data directory."
                    .to_string(),
                author_id: author.id.0
            },
            heck_id
        )
    }
}

#[derive(Debug, Modal)]
#[name = "Add your heck"] // Struct name by default
struct AddHeck {
    #[name = "You must specify at least <user>!"]
    #[paragraph] // Switches from single-line input to multiline text box
    #[placeholder = "<author> topped <user>!"]
    heck: String // Option means optional input
}

/// Send a silly message at a user
#[poise::command(slash_command, category = "Testing")]
pub async fn heck(
    ctx: poise::ApplicationContext<'_, crate::Data, crate::Error>,
    #[description = "User to heck"] user: User,
    #[description = "Set to true if you want to add a heck :)"]
    #[flag]
    new_heck: bool,
    #[description = "Return the heck as plaintext"]
    #[flag]
    plaintext: bool,
    #[description = "Get a particular heck. Random heck returned if not found"] heck_id: Option<usize>
) -> Result<(), Error> {
    let mut heck;
    // User wants us to add a heck
    if new_heck {
        heck = if let Some(new_heck) = AddHeck::execute(ctx).await? {
            (
                HeckInt {
                    heck: new_heck.heck,
                    author_id: ctx.author().id.0
                },
                69
            ) // The 69 here is not needed, it's for a laugh :)
        } else {
            ctx.say("Your heck was not present. Make sure you include `<user>`!\n\nFor example: `<author> topped <user>!` You can use `\\n` for a newline")
                .await?;
            return Ok(());
        };

        let mut write = ctx.data().heck.write().await;
        // First Check: If an owner is running the command, don't check to make sure the message contains both <user> and <author>.
        // This is so you can have custom messages, and its implied the owners know what they are doing...
        // Second Check: Make sure the input contains both <user> and <author>
        if ctx.framework().options.owners.contains(&ctx.author().id) || heck.0.heck.contains("<user>") {
            // We want to write the raw string to disk, so we update heck again AFTER it has been written.
            write.heck.append(&mut vec![heck.0.clone()]);
            heck = (
                HeckInt {
                    heck: heck.0.heck.replace("<user>", user.to_string().as_str()).replace("<author>", ctx.author().to_string().as_str()), // Format the heck to mention the user in this instance,
                    author_id: ctx.author().id.0
                },
                write.heck.len() - 1
            );
        } else {
            // Format not allowed!
            ctx.say(format!(
                "Your heck was `{}` but the format was wrong. Make sure you include `<user>`!\n\nFor example: `<author> topped <user>!`",
                heck.0.heck
            ))
            .await?;
            return Ok(()); // We can exit the function now
        }
        Heck::write(&write, HECK_FILE_PATH); // Save our new heck to the database, unformatted.
    } else {
        // Not adding a heck, so let's get one
        let hecks = &ctx.data.heck.read().await.heck;
        heck = heck_function(ctx.author(), &user, hecks, heck_id).await;
    };

    if plaintext {
        // Split the message if it is too big!
        if heck.0.heck.len() > 2000 {
            let (split1, split2) = heck.0.heck.split_at(2000);
            ctx.say(split1).await?;
            ctx.say(split2).await?;
        } else {
            ctx.say(heck.0.heck).await?;
        }
    } else {
        let config = ctx.data.config.read().await;
        let accent_colour = guild_accent_colour(config.accent_colour, ctx.guild());
        // Try getting the author from the cache
        let heck_author = if let Some(cache) = ctx.serenity_context.cache() {
            cache.user(heck.0.author_id)
        } else {
            match ctx.serenity_context.http.get_user(heck.0.author_id).await {
                Ok(user) => Some(user),
                Err(_) => None
            }
        };
        let embed = embed(accent_colour, heck.0.heck.clone(), heck.1, heck_author).await;

        ctx.send(|builder| {
            builder
                .embed(|e| {
                    *e = embed;
                    e
                })
                .content(&user.to_string())
        })
        .await?;
    };

    Ok(())
}

/// Send a silly message at a user - Context menu edition
#[poise::command(category = "Testing", context_menu_command = "Heck this user :3c")]
pub async fn heck_user(ctx: Context<'_>, #[description = "User to heck"] user: User) -> Result<(), Error> {
    let hecks = &ctx.data().heck.read().await.heck;
    let heck = heck_function(ctx.author(), &user, hecks, None).await;

    let config = ctx.data().config.read().await;
    let accent_colour = guild_accent_colour(config.accent_colour, ctx.guild());

    // Try getting the author from the cache
    let heck_author = if let Some(cache) = ctx.cache() {
        cache.user(heck.0.author_id)
    } else {
        match ctx.http().get_user(heck.0.author_id).await {
            Ok(user) => Some(user),
            Err(_) => None
        }
    };

    let embed = embed(accent_colour, heck.0.heck.clone(), heck.1, heck_author).await;

    ctx.send(|builder| {
        builder
            .embed(|e| {
                *e = embed;
                e
            })
            .content(&user.to_string())
    })
    .await?;
    Ok(())
}

async fn embed(accent_colour: Colour, heck: String, heck_id: usize, heck_author: Option<User>) -> CreateEmbed {
    let mut embed = CreateEmbed::default();

    embed.color(accent_colour);
    embed.description(heck);
    if let Some(heck_author) = heck_author {
        embed.author(|embed_author| {
            embed_author
                .name(format!("Heck created by {}", heck_author.name))
                .icon_url(heck_author.avatar_url().unwrap_or_default())
        });
    };
    embed.footer(|footer| footer.text(format!("Heck ID: {heck_id}")));

    embed
}
