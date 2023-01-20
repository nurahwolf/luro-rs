use luro_core::{
    heck::{Heck, Hecks},
    Data, Error, HECK_FILE_PATH
};
use luro_utilities::guild_accent_colour;
use poise::{
    serenity_prelude::{Colour, CreateEmbed, User},
    Modal
};
use rand::{seq::SliceRandom, Rng};

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
    ctx: poise::ApplicationContext<'_, Data, Error>,
    #[description = "User to heck"] mut user: User,
    #[description = "Set to true if you want to add a heck :)"]
    #[flag]
    new_heck: bool,
    #[description = "Heck a random user EXCEPT the person you specified!"]
    #[flag]
    random_user: bool,
    #[description = "Return the heck as plaintext"]
    #[flag]
    plaintext: bool,
    #[description = "Get a particular heck. Random heck returned if not found"] heck_id: Option<usize>
) -> Result<(), Error> {
    let heck_db = &mut ctx.data().heck.write().await;

    if heck_db.available_heck_ids.is_empty() {
        heck_db.reload_ids();
    };

    let mut heck;

    if new_heck {
        let heckid;
        heck = if let Some(new_heck) = AddHeck::execute(ctx).await? {
            heckid = heck_db.hecks.len() + 1;
            Heck {
                heck_message: new_heck.heck,
                author_id: ctx.author().id.0
            }
        } else {
            ctx.say("Your heck was not present. Make sure you include `<user>`!\n\nFor example: `<author> topped <user>!` You can use `\\n` for a newline")
                .await?;
            return Ok(());
        };

        // First Check: If an owner is running the command, don't check to make sure the message contains both <user> and <author>.
        // This is so you can have custom messages, and its implied the owners know what they are doing...
        // Second Check: Make sure the input contains both <user> and <author>
        if ctx.framework().options.owners.contains(&ctx.author().id) || heck.heck_message.contains("<user>") {
            // We want to write the raw string to disk, so we update heck again AFTER it has been written.
            // Add our new heck
            heck_db.hecks.append(&mut vec![heck.clone()]);
            Hecks::write(&heck_db, HECK_FILE_PATH).await; // Save our new heck to the database, unformatted.
                                                          // Now regenerate the heck IDs
            heck_db.reload_ids();
            // Format our pretty new heck and send it
            heck = format_heck(&heck, ctx.author(), &user).await;
            send_heck(heck, &heckid, plaintext, ctx, user).await?;
            return Ok(());
        } else {
            // Format not allowed!
            ctx.say(format!(
                "Your heck was `{}` but the format was wrong. Make sure you include `<user>`!\n\nFor example: `<author> topped <user>!`",
                heck.heck_message
            ))
            .await?;
            return Ok(()); // We can exit the function now
        }
    };

    // Get a specific heck
    if let Some(heck_id) = heck_id {
        match heck_db.hecks.get(heck_id) {
            Some(h) => {
                heck = format_heck(&h, ctx.author(), &user).await;
                send_heck(heck.clone(), &heck_id, plaintext, ctx, user).await?;
                return Ok(());
            }
            None => {
                ctx.say("Failed to find that heck ID").await?;
            }
        };
    }

    // Get a random heck
    let random_number = rand::thread_rng().gen_range(0..heck_db.available_heck_ids.len());
    let random_heck_id = heck_db.available_heck_ids.remove(random_number);

    heck = match heck_db.hecks.get(random_heck_id) {
        Some(heck_id) => heck_id.to_owned(),
        None => {
            ctx.say("Failed to get a random heck").await?;
            return Ok(());
        }
    };

    // The user has requested that we get a random user, so let's get one
    if random_user && let Some(guild) = ctx.guild() && let Ok(members) = guild.members(&ctx.serenity_context.http, None, None).await {
        let random_member = members.choose(&mut rand::thread_rng());
        if let Some(random_member_matched) = random_member {
            user = random_member_matched.user.clone();
        }
    };

    heck = format_heck(&heck, ctx.author(), &user).await;
    send_heck(heck, &random_heck_id, plaintext, ctx, user).await?;

    Ok(())
}

/// Send a silly message at a user - Context menu edition
#[poise::command(category = "Testing", context_menu_command = "Heck this user :3c")]
pub async fn heck_user(
    ctx: poise::ApplicationContext<'_, Data, Error>,
    #[description = "User to heck"] user: User
) -> Result<(), Error> {
    let heck_db = &mut ctx.data().heck.write().await;

    if heck_db.available_heck_ids.is_empty() {
        heck_db.reload_ids();
    };

    let random_number = rand::thread_rng().gen_range(0..heck_db.available_heck_ids.len());
    let random_heck_id = heck_db.available_heck_ids.remove(random_number);

    let mut random_heck = match heck_db.hecks.get(random_heck_id) {
        Some(heck_id) => heck_id.to_owned(),
        None => {
            ctx.say("Failed to get a random heck").await?;
            return Ok(());
        }
    };

    random_heck = format_heck(&random_heck, ctx.author(), &user).await;
    send_heck(random_heck, &random_heck_id, false, ctx, user).await?;

    Ok(())
}

/// Replace <user> and <author> with the hecked user's username and author's name
async fn format_heck(heck: &Heck, heck_author: &User, hecked_user: &User) -> Heck {
    Heck {
        heck_message: heck
            .heck_message
            .replace("<user>", hecked_user.to_string().as_str())
            .replace("<author>", &heck_author.to_string()),
        author_id: heck.author_id
    }
}

/// Send the heck to the channel, formatted depending on what the user requested
async fn send_heck(
    heck: Heck,
    heck_id: &usize,
    send_as_plaintext: bool,
    ctx: poise::ApplicationContext<'_, Data, Error>,
    user_to_heck: User
) -> Result<(), Error> {
    if send_as_plaintext {
        // Split the message if it is too big!
        if heck.heck_message.len() > 2000 {
            let (split1, split2) = heck.heck_message.split_at(2000);
            ctx.say(split1).await?;
            ctx.say(split2).await?;
        } else {
            ctx.say(heck.heck_message.clone()).await?;
        }
        return Ok(());
    }

    let accent_colour = guild_accent_colour(ctx.data().config.read().await.accent_colour, ctx.guild());
    // Try getting the author from the cache, fail back to just getting the user if not found
    let heck_author = match ctx.serenity_context().cache.user(heck.author_id) {
        Some(user) => Some(user),
        None => match ctx.serenity_context().http.get_user(heck.author_id).await {
            Ok(user) => Some(user),
            Err(_) => None
        }
    };

    let embed = embed(accent_colour, heck.heck_message.clone(), heck_id, heck_author).await;

    ctx.send(|builder| {
        builder
            .embed(|e| {
                *e = embed;
                e
            })
            .content(&user_to_heck.to_string())
    })
    .await?;

    Ok(())
}

async fn embed(accent_colour: Colour, heck_message: String, heck_id: &usize, heck_author: Option<User>) -> CreateEmbed {
    let mut embed = CreateEmbed::default();

    embed.color(accent_colour);
    embed.description(heck_message);
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
