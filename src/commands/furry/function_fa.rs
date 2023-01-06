use crate::commands::furry::struct_furaffinity::FurAffinity;
use crate::utils::guild_accent_colour;
use crate::{Data, Error, FURAFFINITY_REGEX};

use futures::StreamExt;
use poise::serenity_prelude::{ButtonStyle, CreateComponents, CreateEmbed, CreateMessage, EditMessage, Message};
use poise::serenity_prelude::{Colour, InteractionResponseType};
use poise::CreateReply;
use regex::Regex;
use serde::{Deserialize, Serialize};
use serenity::client::Context;
use std::time::Duration;
use std::vec;

#[derive(Serialize, Deserialize, Debug)]
pub struct Cookies {
    pub name: String,
    pub value: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestBody {
    pub cookies: Vec<Cookies>,
    pub bbcode: bool
}

fn embed(furaffinity: &FurAffinity, embed_colour: Option<Colour>) -> CreateEmbed {
    let mut embed = CreateEmbed::default();
    if let Some(colour) = embed_colour {
        embed.colour(colour);
    } else {
        embed.colour(Colour::from_rgb(218, 190, 239));
    }

    if !furaffinity.title.is_empty() {
        embed.title(&furaffinity.title);
    }

    if !furaffinity.author.name.is_empty() && !furaffinity.author.avatar_url.is_empty() {
        embed.author(|author| author.name(&furaffinity.author.name).icon_url(&furaffinity.author.avatar_url));
    }

    if !furaffinity.file_url.is_empty() {
        embed.image(&furaffinity.file_url);
    }

    if !furaffinity.description.is_empty() {
        let re = Regex::new("<([^>]*)>").unwrap();
        let description_modified = furaffinity.description.replace("<br>", "\n");
        let result = re.replace_all(&description_modified, "");
        embed.description(result);
    }

    embed
}

pub fn components(furaffinity: &FurAffinity, disabled: bool) -> CreateComponents {
    let mut components = CreateComponents::default();
    components.create_action_row(|row| {
        row.create_button(|button| button.url(&furaffinity.file_url).label("View on FA").style(ButtonStyle::Link));
        if let Some(_prev) = furaffinity.prev {
            row.create_button(|button| button.custom_id("prev").label("Previous Post").style(ButtonStyle::Primary).disabled(disabled));
        }

        if let Some(_next) = furaffinity.next {
            row.create_button(|button| button.custom_id("next").label("Next Post").style(ButtonStyle::Primary).disabled(disabled));
        }
        row
    });
    components
}

pub async fn fa_message_edit(furaffinity: &FurAffinity, embed_colour: Option<Colour>) -> EditMessage<'static> {
    let embed = embed(furaffinity, embed_colour);
    let components = components(furaffinity, false);
    let mut edit_message = EditMessage::default();

    edit_message
        .embed(|e| {
            *e = embed;
            e
        })
        .components(|c| {
            *c = components;
            c
        });

    edit_message
}

pub async fn fa_message(furaffinity: &FurAffinity, embed_colour: Option<Colour>, message: &Message) -> CreateMessage<'static> {
    let embed = embed(furaffinity, embed_colour);
    let components = components(furaffinity, false);
    let mut create_message = CreateMessage::default();

    create_message
        .embed(|e| {
            *e = embed;
            e
        })
        .components(|c| {
            *c = components;
            c
        }).reference_message(message);

    create_message
}

pub async fn fa_reply(furaffinity: &FurAffinity, embed_colour: Option<Colour>) -> CreateReply<'static> {
    let embed = embed(furaffinity, embed_colour);
    let components = components(furaffinity, false);
    let mut create_reply = CreateReply::default();

    create_reply
        .embed(|e| {
            *e = embed;
            e
        })
        .components(|c| {
            *c = components;
            c
        });

    create_reply
}

pub async fn furaffinity_client(url: Option<&String>, submission_id: Option<i64>, cookies: &[String; 2]) -> Result<FurAffinity, reqwest::Error> {
    let client = reqwest::Client::builder().build()?;
    let cookies = vec![
        Cookies {
            name: "a".to_string(),
            value: cookies[0].to_string()
        },
        Cookies {
            name: "b".to_string(),
            value: cookies[1].to_string()
        },
    ];

    let body = RequestBody { cookies, bbcode: false };

    let request_url = if let Some(url) = url {
        let regex = Regex::new(FURAFFINITY_REGEX).unwrap();
        let mut post_id = Vec::new();
        for cap in regex.captures_iter(url) {
            post_id.push(cap[1].to_string())
        }
        let temp = format!("https://furaffinity-api.herokuapp.com/submission/{}/", post_id.first().unwrap());
        temp
    } else if let Some(submission_id) = submission_id {
        let temp = format!("https://furaffinity-api.herokuapp.com/submission/{submission_id}/");
        temp
    } else {
        panic!("Furaffinity: A URL or submission ID was not passed to the furaffinity client!");
    };

    println!("Furaffinity: Attempting to get post: {request_url}");

    let response = client.post(request_url).json(&body).send().await;

    let response_resolved = match response {
        Ok(response) => response.json::<FurAffinity>().await,
        Err(err) => {
            return Err(err);
        }
    };

    let furaffinity = match response_resolved {
        Ok(furaffinity) => furaffinity,
        Err(err) => {
            return Err(err);
        }
    };

    Ok(furaffinity)
}

/// Get a post from FA and send an embed response.
pub async fn event_furaffinity(ctx: &Context, framework: poise::FrameworkContext<'_, Data, Error>, message: &Message) -> Result<(), reqwest::Error> {
    let url = &message.content;
    let fa_client = furaffinity_client(Some(url), None, &framework.user_data.secrets.furaffinity_cookies).await; // Get a FA client, build it on the url input
    let colour = guild_accent_colour(framework.user_data.config.lock().unwrap().accent_colour, message.guild(ctx));

    let mut fa = match fa_client {
        // Make sure it is valid
        Ok(fa) => fa,
        Err(_) => {
            return Ok(());
        }
    };

    // Build a message based on our response, setting it up for interaction
    let furaffinity_message = fa_message(&fa, Some(colour), message).await;
    let reply = message
        .channel_id
        .send_message(ctx.http.clone(), |builder| {
            *builder = furaffinity_message;
            builder
        })
        .await;

    let mut reply_handle = match reply {
        Ok(message) => message,
        Err(err) => panic!("Furaffinity: Failed to send message to channel! {err}")
    };

    let mut interaction_stream = reply_handle.await_component_interactions(ctx).timeout(Duration::from_secs(60 * 3)).build();

    // Act on our interaction context
    while let Some(interaction) = interaction_stream.next().await {
        match interaction.create_interaction_response(ctx, |f| f.kind(InteractionResponseType::UpdateMessage)).await {
            Ok(_) => {}
            Err(err) => panic!("Furaffinity: Had a fuckywucky: {err}")
        }

        if interaction.data.custom_id.contains("prev") {
            fa = match furaffinity_client(None, Some(fa.prev.unwrap()), &framework.user_data.secrets.furaffinity_cookies).await {
                // Make sure it is valid
                Ok(fa) => fa,
                Err(err) => {
                    panic!("Furaffinity: Failed to send message to channel! {err}")
                }
            };

            // Build a message based on our response, setting it up for interaction
            let message = fa_message_edit(&fa, Some(colour)).await;
            match reply_handle
                .edit(ctx, |builder| {
                    *builder = message;
                    builder
                })
                .await
            {
                Ok(_) => {}
                Err(err) => panic!("Furaffinity: Had a fuckywucky: {err}")
            }
        }

        if interaction.data.custom_id.contains("next") {
            fa = match furaffinity_client(None, Some(fa.next.unwrap()), &framework.user_data.secrets.furaffinity_cookies).await {
                // Make sure it is valid
                Ok(fa) => fa,
                Err(err) => {
                    panic!("Furaffinity: Failed to send message to channel! {err}")
                }
            };

            // Build a message based on our response, setting it up for interaction
            let message = fa_message_edit(&fa, Some(colour)).await;
            match reply_handle
                .edit(ctx, |builder| {
                    *builder = message;
                    builder
                })
                .await
            {
                Ok(_) => {}
                Err(err) => panic!("Furaffinity: Had a fuckywucky: {err}")
            }
        }
    }

    match reply_handle.edit(ctx, |builder|
    builder.components(|c|{
        let components = components(&fa, true);
        *c = components;
        c
    })).await {
        Ok(_) => {}
        Err(err) => panic!("Furaffinity: Had a fuckywucky: {err}")
    }

    Ok(())
}
