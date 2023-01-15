use crate::{constants::TIMEOUT_DURIATION, structs::saucenao::SauceNAO, Context};
use futures::StreamExt;
use poise::{
    serenity_prelude::{ButtonStyle, CreateComponents, InteractionResponseType},
    ReplyHandle
};
use std::time::Duration;

async fn reverse_client(url: &String, api_key: &String) -> Result<SauceNAO, reqwest::Error> {
    let client = reqwest::Client::builder().build()?;

    let response = client
        .get("https://saucenao.com/search.php")
        .query(&[
            ("api_key", api_key),
            // ("testmode","1"),
            ("output_type", &"2".to_string()), // https://saucenao.com/tools/examples/api/index_details.txt
            ("dbs[]", &"25".to_string()),      // Gelbooru
            ("dbs[]", &"29".to_string()),      // E621
            ("dbs[]", &"34".to_string()),      // DeviantArt
            ("dbs[]", &"39".to_string()),      // ArtStation
            ("dbs[]", &"40".to_string()),      // FurAffinity
            ("dbs[]", &"41".to_string()),      // Twitter
            ("dbs[]", &"42".to_string()),      // Furry Network
            ("dbs[]", &"43".to_string()),      // Kemono
            ("url", url)
        ])
        .send()
        .await;

    let response_resolved = match response {
        Ok(response) => response.json::<SauceNAO>().await,
        Err(err) => {
            return Err(err);
        }
    };

    match response_resolved {
        Ok(saucenao) => Ok(saucenao),
        Err(err) => {
            println!("Saucenao: {err}\nRequest URL: {url}");
            if let Some(status_code) = err.status() {
                println!("Status Code: {status_code}");
            };
            Err(err)
        }
    }
}

pub async fn interactive_response<'a>(
    ctx: Context<'a>,
    url: String,
    api_key: &String
) -> Result<ReplyHandle<'a>, serenity::Error> {
    let mut cursor = 0;
    let sauce = match reverse_client(&url, api_key).await {
        Ok(saucenao) => saucenao,
        Err(err) => {
            if let Some(status_code) = err.status() {
                return ctx.say(format!("This API is limited to 100 requests a day because Nurah is not gonna spend 6$ a month for this... If you *really* want more requests, consider donating.\nStatus Code: {status_code}")).await;
            } else {
                return ctx.say(format!("API Error: {err}")).await;
            };
        }
    };

    if sauce.header.status == -2 {
        return ctx
            .say("This API is limited to 100 requests a day because Nurah is not gonna spend 6$ a month for this... If you *really* want more requests, consider donating.".to_string())
            .await;
    };

    let results = match &sauce.results {
        Some(results) => match results.get(cursor) {
            Some(result) => result,
            None => return ctx.say("Did not find anything ;)").await
        },
        None => return ctx.say("Did not find anything ;)").await
    };

    let reply_handle = match results.data.ext_urls.first() {
        Some(result) => {
            ctx.send(|builder| {
                builder.content(result);
                builder.components(|c| {
                    *c = components(&sauce, &cursor, false, false);
                    c
                });
                builder
            })
            .await?
        }
        None => {
            return ctx.say("Did not find anything ;)").await;
        }
    };

    let mut interaction_stream = reply_handle
        .message()
        .await?
        .await_component_interactions(ctx)
        .timeout(Duration::from_secs(TIMEOUT_DURIATION))
        .build();

    // Act on our interaction context
    while let Some(interaction) = interaction_stream.next().await {
        interaction
            .create_interaction_response(ctx, |f| f.kind(InteractionResponseType::UpdateMessage))
            .await?;

        if interaction.data.custom_id.contains("prev") {
            cursor -= 1;

            if let Some(results) = &sauce.results {
                if let Some(result) = results.get(cursor) {
                    reply_handle
                        .edit(ctx, |builder| {
                            builder.content(result.data.ext_urls.first().unwrap());
                            builder.components(|c| {
                                *c = components(&sauce, &cursor, false, false);
                                c
                            });
                            builder
                        })
                        .await?
                } else {
                    reply_handle
                        .edit(ctx, |builder| builder.content("Did not find anything ;)"))
                        .await?
                }
            } else {
                reply_handle
                    .edit(ctx, |builder| builder.content("Did not find anything ;)"))
                    .await?
            };
        }

        if interaction.data.custom_id.contains("next") {
            cursor += 1;

            if let Some(results) = &sauce.results {
                if let Some(result) = results.get(cursor) {
                    reply_handle
                        .edit(ctx, |builder| {
                            builder.content(result.data.ext_urls.first().unwrap());
                            builder.components(|c| {
                                *c = components(&sauce, &cursor, false, false);
                                c
                            });
                            builder
                        })
                        .await?
                } else {
                    reply_handle
                        .edit(ctx, |builder| builder.content("Did not find anything ;)"))
                        .await?
                }
            } else {
                reply_handle
                    .edit(ctx, |builder| builder.content("Did not find anything ;)"))
                    .await?
            };
        }
    }

    reply_handle
        .edit(ctx, |builder| {
            builder.components(|c| {
                let components = components(&sauce, &cursor, true, true);
                *c = components;
                c
            })
        })
        .await?;

    Ok(reply_handle)
}

fn components(saucenao: &SauceNAO, cursor: &usize, disable_left: bool, disable_right: bool) -> CreateComponents {
    let mut components = CreateComponents::default();
    components.create_action_row(|row| {
        row.create_button(|button| {
            let disabled = if disable_left { true } else { cursor <= &0 };

            button
                .custom_id("prev")
                .label("Previous Source")
                .style(ButtonStyle::Primary)
                .disabled(disabled);
            button
        });

        row.create_button(|button| {
            let disabled = if disable_right {
                true
            } else {
                cursor >= &saucenao.results.clone().unwrap().len()
            };

            button
                .custom_id("next")
                .label("Next Source")
                .style(ButtonStyle::Primary)
                .disabled(disabled);
            button
        });
        row
    });
    components
}
