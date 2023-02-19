use anyhow::Result;
use rand::Rng;
use tracing::{debug, warn};
use twilight_interactions::command::{CommandInputData, CommandModel, CreateCommand, ResolvedUser};
use twilight_model::{
    application::interaction::{application_command::InteractionChannel, Interaction},
    channel::message::{
        component::{ActionRow, TextInput, TextInputStyle},
        embed::{EmbedAuthor, EmbedFooter},
        Component, MessageFlags,
    },
    guild,
    http::interaction::{InteractionResponse, InteractionResponseData, InteractionResponseType},
    id::{
        marker::{ChannelMarker, GuildMarker, UserMarker},
        Id,
    },
    user::User,
};
use twilight_util::builder::embed::{EmbedAuthorBuilder, EmbedBuilder};

use crate::{config::Heck, Luro, functions::get_guild_avatar::get_guild_avatar_url};

#[derive(CommandModel, CreateCommand)]
#[command(name = "heck", desc = "Send a playful, silly message at someone")]
pub enum HeckCommand {
    #[command(name = "user")]
    User(HeckUser),
    #[command(name = "add")]
    Add(AddHeck),
    #[command(name = "add")]
    Edit(EditHeck),
    #[command(name = "add")]
    Delete(DeleteHeck),
}

#[derive(CommandModel, CreateCommand)]
#[command(name = "user", desc = "Send a playful, silly message at someone")]
pub struct HeckUser {
    /// The user to heck
    user: ResolvedUser,
    /// Send the heck as plaintext, instead of as an embed
    plaintext: Option<bool>,
    /// Get a specific heck by ID number
    heck_id: Option<i64>,
}

#[derive(CommandModel, CreateCommand)]
#[command(name = "add", desc = "Create your special own heck!")]
pub struct AddHeck {}

#[derive(CommandModel, CreateCommand)]
#[command(name = "edit", desc = "Edit a heck that you created")]
pub struct EditHeck {
    /// The ID of the heck to edit
    heck_id: i64,
}

#[derive(CommandModel, CreateCommand)]
#[command(name = "delete", desc = "Delete a heck if it belongs to you")]
pub struct DeleteHeck {
    /// The ID of the heck to delete
    heck_id: i64,
}

/// Replace <user> and <author> with the hecked user's username and author's name
async fn format_heck(heck: &Heck, heck_author: &User, hecked_user: &ResolvedUser) -> Heck {
    Heck {
        heck_message: heck
            .heck_message
            .replace("<user>", format!("<@{}>", hecked_user.resolved.id).as_str())
            .replace("<author>", format!("<@{}>", heck_author.id).as_str()),
        author_id: heck.author_id,
    }
}

pub async fn heck_command<'a>(luro: &Luro, interaction: &Interaction) -> Result<()> {
    let command_data = match Luro::get_interaction_data(interaction).await {
        Ok(ok) => ok,
        Err(why) => {
            warn!("Failed to get interaction data - {why}");
            return Ok(());
        }
    };

    let interaction_data =
        match HeckCommand::from_interaction(CommandInputData::from(*command_data)) {
            Ok(ok) => ok,
            Err(why) => {
                warn!("Failed to parse interaction data - {why}");
                return Ok(());
            }
        };

    let response = match interaction_data {
        HeckCommand::User(heck_data) => {
            let mut heck_db = luro.hecks.write().await;

            let author = match interaction.author() {
                Some(ok) => ok,
                None => todo!(),
            };

            // Get a random heck
            let random_number = rand::thread_rng().gen_range(0..heck_db.nsfw_heck_ids.len());
            let random_heck_id = heck_db.nsfw_heck_ids.remove(random_number);

            let (heck, heck_id) = match heck_db.nsfw_hecks.get(random_heck_id) {
                Some(heck) => (heck.to_owned(), random_number),
                None => panic!("Failed to get a random heck"),
            };

            let formatted_heck = format_heck(&heck, author, &heck_data.user).await;

            if let Some(plaintext) = heck_data.plaintext && plaintext {
                    InteractionResponse {
                        kind: InteractionResponseType::ChannelMessageWithSource,
                        data: Some(InteractionResponseData {
                            content: Some(formatted_heck.heck_message),
                            ..Default::default()
                        }),
                    }
            } else {
                let author_id: Id<UserMarker> = Id::new(heck.author_id);
                let author = match luro.http.user(author_id).await {
                    Ok(ok) => match ok.model().await {
                        Ok(ok) => ok,
                        Err(why) => {
                            warn!("Failed to resolve user ID {} - {}", heck.author_id, why);
                            return Ok(());
                        },
                    },
                    Err(why) => {
                        warn!("Failed to resolve user ID {} - {}", heck.author_id, why);
                        return Ok(());
                    },
                };
                let embed = create_heck_embed(luro, interaction.guild_id, formatted_heck.heck_message, &author, true, heck_id).await.build();

                InteractionResponse {
                    kind: InteractionResponseType::ChannelMessageWithSource,
                    data: Some(InteractionResponseData {
                        embeds: Some(vec![embed]),
                        ..Default::default()
                    }),
                }
            }
        }
        HeckCommand::Add(add_heck) => {
            let componets = vec![Component::ActionRow(ActionRow {
                components: vec![Component::TextInput(TextInput {
                    custom_id: "modal_input".to_string(),
                    label: "Your new heck".to_string(),
                    max_length: None,
                    min_length: None,
                    placeholder: Some("<user> just pat <author> on the head!".to_string()),
                    required: None,
                    style: TextInputStyle::Paragraph,
                    value: None,
                })],
            })];

            InteractionResponse {
                kind: InteractionResponseType::Modal,
                data: Some(InteractionResponseData {
                    components: Some(componets),
                    content: Some("You must specify at least <user>!".to_owned()),
                    custom_id: Some("modal".to_string()),
                    title: Some("Add a heck!".to_string()),
                    ..Default::default()
                }),
            }
        }
        HeckCommand::Edit(_) => todo!(),
        HeckCommand::Delete(_) => todo!(),
    };

    match luro
        .http
        .interaction(luro.application_id)
        .create_response(interaction.id, &interaction.token, &response)
        .await
    {
        Ok(ok) => ok,
        Err(why) => {
            warn!("Failed to send interaction: {why}");
            return Ok(());
        }
    };

    Ok(())
}

/// Checks to see if hecks are present, if they are not then reload the database. Note that this opens the heck DB as writable!
async fn check_hecks_are_present(luro: &Luro) -> Result<()> {
    let heck_db = &mut luro.hecks.write().await;

    if heck_db.sfw_heck_ids.is_empty() || heck_db.nsfw_heck_ids.is_empty() {
        heck_db.reload_all_heck_ids();
    };

    Ok(())
}

/// Open the database as writeable and remove a NSFW heck from it, returning the heck removed
async fn get_nsfw_heck(luro: &Luro, heck_id: Option<usize>) -> Result<(Heck, usize), String> {
    let heck_db = &mut luro.hecks.write().await;

    let heck = match heck_id {
        Some(heck_id) => match heck_db.nsfw_hecks.get(heck_id) {
            Some(heck) => Ok((heck.to_owned(), heck_id)),
            None => Err("Failed to find that heck ID".to_string()),
        },
        None => {
            // Get a random heck
            let random_number = rand::thread_rng().gen_range(0..heck_db.nsfw_heck_ids.len());
            let random_heck_id = heck_db.nsfw_heck_ids.remove(random_number);

            match heck_db.nsfw_hecks.get(random_heck_id) {
                Some(heck) => Ok((heck.to_owned(), random_number)),
                None => Err("Failed to get a random heck".to_string()),
            }
        }
    };

    heck
}

async fn create_heck(
    luro: &Luro,
    heck_message: String,
    author_id: u64,
    nsfw: bool,
) -> (Heck, usize) {
    let heck_db = &mut luro.hecks.read().await;

    let heck = Heck {
        heck_message,
        author_id,
    };

    let hecks_in_db = if nsfw {
        heck_db.nsfw_hecks.len()
    } else {
        heck_db.sfw_hecks.len()
    };

    (heck, hecks_in_db)
}

/// Open the database as writeable and remove a SFW heck from it, returning the heck removed
async fn get_sfw_heck(luro: &Luro, heck_id: Option<usize>) -> Result<(Heck, usize), String> {
    let heck_db = &mut luro.hecks.write().await;

    let heck = match heck_id {
        Some(heck_id) => match heck_db.sfw_hecks.get(heck_id) {
            Some(heck) => Ok((heck.to_owned(), heck_id)),
            None => Err("Failed to find that heck ID".to_string()),
        },
        None => {
            // Get a random heck
            let random_number = rand::thread_rng().gen_range(0..heck_db.sfw_heck_ids.len());
            let random_heck_id = heck_db.sfw_heck_ids.remove(random_number);

            match heck_db.sfw_hecks.get(random_heck_id) {
                Some(heck) => Ok((heck.to_owned(), random_number)),
                None => Err("Failed to get a random heck".to_string()),
            }
        }
    };

    heck
}

// pub async fn heck() -> Result<()> {
//     // Make sure that the hecks have not been fully drained from the database, if so, reload them. This opens the DB as writable for the life of the function
//     debug!("Checking to make sure we have hecks to get");
//     check_hecks_are_present(ctx.data).await?;
//     debug!("Checks done");

//     if new_heck {
//         let nsfw_channel = nsfw_check(Some(&ctx.serenity_context.cache), ctx.channel_id());
//         debug!("User wants to add a heck, so open the DB as writable");
//         let new_heck = match AddHeck::execute(ctx).await {
//             Ok(heck) => heck,
//             Err(err) => {
//                 ctx.say(format!("The model had an error: {err}")).await?;
//                 return Ok(());
//             }
//         };
//         debug!("The model responded, so lets make our heck");

//         let (mut heck, heckid) = if let Some(new_heck) = new_heck {
//             create_heck(ctx.data, new_heck.heck, ctx.author().id.0, nsfw_channel).await
//         } else {
//             ctx.say("Your heck was not present. Make sure you include `<user>`!\n\nFor example: `<author> topped <user>!` You can hold shift and press enter for a newline.")
//                 .await?;
//             return Ok(());
//         };
//         debug!("Heck made, opening DB as writable");
//         let heck_db = &mut ctx.data.heck.write().await;
//         debug!("Got DB as writable, let's create our heck");

//         // First Check: If an owner is running the command, don't check to make sure the message contains either <user> or <author>.
//         // This is so you can have fully custom messages, and its implied the owners knows what they are doing...
//         // Second Check: Make sure the input contains at least <user>
//         if ctx.framework().options.owners.contains(&ctx.author().id) || heck.heck_message.contains("<user>") {
//             // We want to write the raw string to disk, so we update the heck again AFTER it has been written.
//             if nsfw_channel {
//                 heck_db.nsfw_hecks.append(&mut vec![heck.clone()]);
//             } else {
//                 heck_db.sfw_hecks.append(&mut vec![heck.clone()]);
//             };

//             // Save our new heck to the database, unformatted.
//             Hecks::write(heck_db, HECK_FILE_PATH).await;
//             // Format our heck then send it!
//             heck = format_heck(&heck, ctx.author(), &user).await;
//             send_heck(heck, &heckid, plaintext, ctx, user).await?;
//             return Ok(());
//         } else {
//             // Format not allowed!
//             ctx.say(format!(
//                 "Your heck was `{}` but the format was wrong. Make sure you include at least `<user>`!\n\nFor example: `<author> topped <user>!`",
//                 heck.heck_message
//             ))
//             .await?;
//             return Ok(()); // We can exit the function now
//         }
//     };

//     // The user has requested that we get a random user, so let's get one
//     if random_user && let Some(guild) = ctx.guild() && let Ok(members) = guild.members(&ctx.serenity_context.http, None, None).await {
//         let random_member = members.choose(&mut rand::thread_rng());
//         if let Some(random_member_matched) = random_member {
//             user = random_member_matched.user.clone();
//         }
//     };

//     let heck = if nsfw_check(Some(&ctx.serenity_context.cache), ctx.channel_id()) {
//         match get_nsfw_heck(ctx.data, heck_id).await {
//             Ok(heck) => heck,
//             Err(err) => {
//                 error!("Failed to get heck: {err}");
//                 ctx.say(format!("Failed to get heck: {err}")).await?;
//                 return Ok(());
//             }
//         }
//     } else {
//         match get_sfw_heck(ctx.data, heck_id).await {
//             Ok(heck) => heck,
//             Err(err) => {
//                 error!("Failed to get heck: {err}");
//                 ctx.say(format!("Failed to get heck: {err}")).await?;
//                 return Ok(());
//             }
//         }
//     };

//     let formatted_heck = format_heck(&heck.0, ctx.author(), &user).await;
//     send_heck(formatted_heck, &heck.1, plaintext, ctx, user).await?;

//     Ok(())
// }

// pub async fn heck_user() -> Result<()> {
//     // Make sure that the hecks have not been fully drained from the database, if so, reload them. This opens the DB as writable for the life of the function
//     debug!("Checking to make sure we have hecks to get");
//     check_hecks_are_present(ctx.data).await?;
//     debug!("Checks done");

//     let heck = if nsfw_check(Some(&ctx.serenity_context.cache), ctx.channel_id()) {
//         match get_nsfw_heck(ctx.data, None).await {
//             Ok(heck) => heck,
//             Err(err) => {
//                 error!("Failed to get heck: {err}");
//                 ctx.say(format!("Failed to get heck: {err}")).await?;
//                 return Ok(());
//             }
//         }
//     } else {
//         match get_sfw_heck(ctx.data, None).await {
//             Ok(heck) => heck,
//             Err(err) => {
//                 error!("Failed to get heck: {err}");
//                 ctx.say(format!("Failed to get heck: {err}")).await?;
//                 return Ok(());
//             }
//         }
//     };

//     let formatted_heck = format_heck(&heck.0, ctx.author(), &user).await;
//     send_heck(formatted_heck, &heck.1, false, ctx, user).await?;

//     Ok(())
// }

// /// Send the heck to the channel, formatted depending on what the user requested
// async fn send_heck() -> Result<()> {
//     if send_as_plaintext {
//         // Split the message if it is too big!
//         if heck.heck_message.len() > 2000 {
//             let (split1, split2) = heck.heck_message.split_at(2000);
//             ctx.say(split1).await?;
//             ctx.say(split2).await?;
//         } else {
//             ctx.say(heck.heck_message.clone()).await?;
//         }
//         return Ok(());
//     }

//     let accent_colour = guild_accent_colour(ctx.data().config.read().await.accent_colour, ctx.guild());
//     // Try getting the author from the cache, fail back to just getting the user if not found
//     let heck_author = match ctx.serenity_context().cache.user(heck.author_id) {
//         Some(user) => Some(user),
//         None => match ctx.serenity_context().http.get_user(heck.author_id).await {
//             Ok(user) => Some(user),
//             Err(_) => None
//         }
//     };

//     let nsfw_heck = nsfw_check(Some(&ctx.serenity_context.cache), ctx.channel_id());
//     let embed = embed(accent_colour, heck.heck_message.clone(), heck_id, heck_author, nsfw_heck).await;

//     ctx.send(|builder| {
//         builder
//             .embed(|e| {
//                 *e = embed;
//                 e
//             })
//             .content(&user_to_heck.to_string())
//     })
//     .await?;

//     Ok(())
// }

async fn create_heck_embed(
    luro: &Luro,
    guild: Option<Id<GuildMarker>>,
    heck_message: String,
    heck_author: &User,
    nsfw: bool,
    heck_id: usize,
) -> EmbedBuilder {
    let embed = EmbedBuilder::default()
        .color(luro.accent_colour(guild).await)
        .description(heck_message)
        .author(EmbedAuthor {
            icon_url: Some(get_guild_avatar_url(&guild.unwrap(), &heck_author.id, &heck_author.avatar.unwrap())),
            name: format!("Heck created by {}", heck_author.name),
            proxy_icon_url: None,
            url: None,
        });

    let embed = match nsfw {
        true => embed.footer(EmbedFooter {
            icon_url: None,
            proxy_icon_url: None,
            text: format!("<NSFW Heck> | Heck {heck_id}"),
        }),
        false => embed.footer(EmbedFooter {
            icon_url: None,
            proxy_icon_url: None,
            text: format!("<SFW Heck> | Heck {heck_id}"),
        }),
    };

    embed
}
