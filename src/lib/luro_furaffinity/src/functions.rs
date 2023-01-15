use luro_core::FURAFFINITY_REGEX;
use poise::{serenity_prelude::{Colour, CreateEmbed, CreateComponents, ButtonStyle, EditMessage, CreateMessage, Message}, CreateReply};
use regex::Regex;

use crate::structs::{Cookies, FurAffinity, RequestBody};

pub fn embed(furaffinity: &FurAffinity, embed_colour: Option<Colour>) -> CreateEmbed {
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
        let re = match Regex::new("<([^>]*)>") {
            Ok(ok) => ok,
            Err(err) => {
                panic!("Failed to match the regex - {err}");
            }
        };
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
            row.create_button(|button| {
                button
                    .custom_id("prev")
                    .label("Previous Post")
                    .style(ButtonStyle::Primary)
                    .disabled(disabled)
            });
        }

        if let Some(_next) = furaffinity.next {
            row.create_button(|button| {
                button
                    .custom_id("next")
                    .label("Next Post")
                    .style(ButtonStyle::Primary)
                    .disabled(disabled)
            });
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
        })
        .reference_message(message);

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

pub async fn furaffinity_client(
    url: Option<&String>,
    submission_id: Option<i64>,
    cookies: &[String; 2]
) -> Result<FurAffinity, reqwest::Error> {
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
        let regex = match Regex::new(FURAFFINITY_REGEX) {
            Ok(ok) => ok,
            Err(err) => {
                panic!("Failed to match the regex - {err}");
            }
        };
        let mut post_id = Vec::new();
        for cap in regex.captures_iter(url) {
            if let Some(capture) = cap.name("submission_id") {
                post_id.push(capture.as_str())
            }
        }

        if let Some(post) = post_id.first() {
            format!("https://furaffinity-api.herokuapp.com/submission/{post}/")
        } else {
            panic!("Furaffinity: no `submission_id` was found in the message!");
        }
    } else if let Some(submission_id) = submission_id {
        let temp = format!("https://furaffinity-api.herokuapp.com/submission/{submission_id}/");
        temp
    } else {
        panic!("Furaffinity: A URL or submission ID was not passed to the furaffinity client!");
    };

    let response = client.post(&request_url).json(&body).send().await;

    let response_resolved = match response {
        Ok(response) => response.json::<FurAffinity>().await,
        Err(err) => {
            return Err(err);
        }
    };

    let furaffinity = match response_resolved {
        Ok(furaffinity) => furaffinity,
        Err(err) => {
            panic!("Furaffinity: {err}\nRequest URL: {request_url}");
        }
    };

    Ok(furaffinity)
}