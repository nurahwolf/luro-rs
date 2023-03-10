use luro_core::{
    heck::{Heck, Hecks},
    Data, Error, HECK_FILE_PATH
};
use luro_utilities::guild_accent_colour;
use luro_utilities::nsfw_check;

use poise::{
    serenity_prelude::{Colour, CreateEmbed, User},
    Modal
};
use rand::{seq::SliceRandom, Rng};
use tracing::{debug, error};

/// Open the database as writable in case we need to reload the hecks
async fn check_hecks_are_present(data: &Data) -> Result<(), Error> {
    let heck_db = &mut data.heck.write().await;

    if heck_db.sfw_heck_ids.is_empty() || heck_db.nsfw_heck_ids.is_empty() {
        heck_db.reload_all_heck_ids();
    };

    Ok(())
}

/// Open the database as writeable and remove a NSFW heck from it, returning the heck removed
async fn get_nsfw_heck(data: &Data, heck_id: Option<usize>) -> Result<(Heck, usize), String> {
    let heck_db = &mut data.heck.write().await;

    let heck = match heck_id {
        Some(heck_id) => match heck_db.nsfw_hecks.get(heck_id) {
            Some(heck) => Ok((heck.to_owned(), heck_id)),
            None => Err("Failed to find that heck ID".to_string())
        },
        None => {
            // Get a random heck
            let random_number = rand::thread_rng().gen_range(0..heck_db.nsfw_heck_ids.len());
            let random_heck_id = heck_db.nsfw_heck_ids.remove(random_number);

            match heck_db.nsfw_hecks.get(random_heck_id) {
                Some(heck) => Ok((heck.to_owned(), random_number)),
                None => Err("Failed to get a random heck".to_string())
            }
        }
    };

    heck
}

async fn create_heck(data: &Data, heck_message: String, author_id: u64, nsfw: bool) -> (Heck, usize) {
    let heck_db = &mut data.heck.read().await;

    let heck = Heck { heck_message, author_id };

    let hecks_in_db = if nsfw {
        heck_db.nsfw_hecks.len()
    } else {
        heck_db.sfw_hecks.len()
    };

    (heck, hecks_in_db)
}

/// Open the database as writeable and remove a SFW heck from it, returning the heck removed
async fn get_sfw_heck(data: &Data, heck_id: Option<usize>) -> Result<(Heck, usize), String> {
    let heck_db = &mut data.heck.write().await;

    let heck = match heck_id {
        Some(heck_id) => match heck_db.sfw_hecks.get(heck_id) {
            Some(heck) => Ok((heck.to_owned(), heck_id)),
            None => Err("Failed to find that heck ID".to_string())
        },
        None => {
            // Get a random heck
            let random_number = rand::thread_rng().gen_range(0..heck_db.sfw_heck_ids.len());
            let random_heck_id = heck_db.sfw_heck_ids.remove(random_number);

            match heck_db.sfw_hecks.get(random_heck_id) {
                Some(heck) => Ok((heck.to_owned(), random_number)),
                None => Err("Failed to get a random heck".to_string())
            }
        }
    };

    heck
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
    // Make sure that the hecks have not been fully drained from the database, if so, reload them. This opens the DB as writable for the life of the function
    debug!("Checking to make sure we have hecks to get");
    check_hecks_are_present(ctx.data).await?;
    debug!("Checks done");

    if new_heck {
        let nsfw_channel = nsfw_check(Some(&ctx.serenity_context.cache), ctx.channel_id());
        debug!("User wants to add a heck, so open the DB as writable");
        let new_heck = match AddHeck::execute(ctx).await {
            Ok(heck) => heck,
            Err(err) => {
                ctx.say(format!("The model had an error: {err}")).await?;
                return Ok(());
            }
        };
        debug!("The model responded, so lets make our heck");

        let (mut heck, heckid) = if let Some(new_heck) = new_heck {
            create_heck(ctx.data, new_heck.heck, ctx.author().id.0, nsfw_channel).await
        } else {
            ctx.say("Your heck was not present. Make sure you include `<user>`!\n\nFor example: `<author> topped <user>!` You can hold shift and press enter for a newline.")
                .await?;
            return Ok(());
        };
        debug!("Heck made, opening DB as writable");
        let heck_db = &mut ctx.data.heck.write().await;
        debug!("Got DB as writable, let's create our heck");

        // First Check: If an owner is running the command, don't check to make sure the message contains either <user> or <author>.
        // This is so you can have fully custom messages, and its implied the owners knows what they are doing...
        // Second Check: Make sure the input contains at least <user>
        if ctx.framework().options.owners.contains(&ctx.author().id) || heck.heck_message.contains("<user>") {
            // We want to write the raw string to disk, so we update the heck again AFTER it has been written.
            if nsfw_channel {
                heck_db.nsfw_hecks.append(&mut vec![heck.clone()]);
            } else {
                heck_db.sfw_hecks.append(&mut vec![heck.clone()]);
            };

            // Save our new heck to the database, unformatted.
            Hecks::write(heck_db, HECK_FILE_PATH).await;
            // Format our heck then send it!
            heck = format_heck(&heck, ctx.author(), &user).await;
            send_heck(heck, &heckid, plaintext, ctx, user).await?;
            return Ok(());
        } else {
            // Format not allowed!
            ctx.say(format!(
                "Your heck was `{}` but the format was wrong. Make sure you include at least `<user>`!\n\nFor example: `<author> topped <user>!`",
                heck.heck_message
            ))
            .await?;
            return Ok(()); // We can exit the function now
        }
    };

    // The user has requested that we get a random user, so let's get one
    if random_user && let Some(guild) = ctx.guild() && let Ok(members) = guild.members(&ctx.serenity_context.http, None, None).await {
        let random_member = members.choose(&mut rand::thread_rng());
        if let Some(random_member_matched) = random_member {
            user = random_member_matched.user.clone();
        }
    };

    let heck = if nsfw_check(Some(&ctx.serenity_context.cache), ctx.channel_id()) {
        match get_nsfw_heck(ctx.data, heck_id).await {
            Ok(heck) => heck,
            Err(err) => {
                error!("Failed to get heck: {err}");
                ctx.say(format!("Failed to get heck: {err}")).await?;
                return Ok(());
            }
        }
    } else {
        match get_sfw_heck(ctx.data, heck_id).await {
            Ok(heck) => heck,
            Err(err) => {
                error!("Failed to get heck: {err}");
                ctx.say(format!("Failed to get heck: {err}")).await?;
                return Ok(());
            }
        }
    };

    let formatted_heck = format_heck(&heck.0, ctx.author(), &user).await;
    send_heck(formatted_heck, &heck.1, plaintext, ctx, user).await?;

    Ok(())
}

/// Send a silly message at a user - Context menu edition
#[poise::command(category = "Testing", context_menu_command = "Heck this user :3c")]
pub async fn heck_user(
    ctx: poise::ApplicationContext<'_, Data, Error>,
    #[description = "User to heck"] user: User
) -> Result<(), Error> {
    // Make sure that the hecks have not been fully drained from the database, if so, reload them. This opens the DB as writable for the life of the function
    debug!("Checking to make sure we have hecks to get");
    check_hecks_are_present(ctx.data).await?;
    debug!("Checks done");

    let heck = if nsfw_check(Some(&ctx.serenity_context.cache), ctx.channel_id()) {
        match get_nsfw_heck(ctx.data, None).await {
            Ok(heck) => heck,
            Err(err) => {
                error!("Failed to get heck: {err}");
                ctx.say(format!("Failed to get heck: {err}")).await?;
                return Ok(());
            }
        }
    } else {
        match get_sfw_heck(ctx.data, None).await {
            Ok(heck) => heck,
            Err(err) => {
                error!("Failed to get heck: {err}");
                ctx.say(format!("Failed to get heck: {err}")).await?;
                return Ok(());
            }
        }
    };

    let formatted_heck = format_heck(&heck.0, ctx.author(), &user).await;
    send_heck(formatted_heck, &heck.1, false, ctx, user).await?;

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

    let nsfw_heck = nsfw_check(Some(&ctx.serenity_context.cache), ctx.channel_id());
    let embed = embed(accent_colour, heck.heck_message.clone(), heck_id, heck_author, nsfw_heck).await;

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

async fn embed(
    accent_colour: Colour,
    heck_message: String,
    heck_id: &usize,
    heck_author: Option<User>,
    nsfw_heck: bool
) -> CreateEmbed {
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
    let footer_text = match nsfw_heck {
        true => format!("--NSFW Heck-- | Heck {heck_id}"),
        false => format!("--SFW Heck-- | Heck {heck_id}")
    };

    embed.footer(|footer| footer.text(footer_text));

    embed
}
