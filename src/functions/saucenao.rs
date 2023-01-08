use crate::{structs::saucenao::SauceNAO, Context, TIMEOUT_DURIATION};
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
            ("output_type", &"2".to_string()),
            ("dbs[]", &"29".to_string()),
            ("dbs[]", &"39".to_string()),
            ("dbs[]", &"40".to_string()),
            ("dbs[]", &"41".to_string()),
            ("dbs[]", &"42".to_string()),
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
            println!("Furaffinity: Failed to decode to JSON object, reason following... {err}");
            Err(err)
        }
    }
}

pub async fn interactive_response<'a>(ctx: Context<'a>, url: String, api_key: &String) -> Result<ReplyHandle<'a>, serenity::Error> {
    let mut cursor = 0;
    let sauce = match reverse_client(&url, &api_key).await {
        Ok(saucenao) => saucenao,
        Err(err) => return ctx.say(format!("API Error: {err}")).await
    };

    let result = match sauce.results.get(cursor) {
        Some(result) => result,
        None => return ctx.say("Did not find anything ;)").await
    };

    let reply_handle = match result.data.ext_urls.first() {
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
        interaction.create_interaction_response(ctx, |f| f.kind(InteractionResponseType::UpdateMessage)).await?;

        if interaction.data.custom_id.contains("prev") {
            cursor -= 1;

            let result = match sauce.results.get(cursor) {
                Some(result) => result,
                None => return ctx.say("Did not find anything ;)").await
            };

            reply_handle
                .edit(ctx, |builder| {
                    builder.content(result.data.ext_urls.first().unwrap());
                    builder.components(|c| {
                        *c = components(&sauce, &cursor, false, false);
                        c
                    });
                    builder
                })
                .await?;
        }
        if interaction.data.custom_id.contains("next") {
            cursor += 1;

            let result = match sauce.results.get(cursor) {
                Some(result) => result,
                None => return ctx.say("Did not find anything ;)").await
            };

            reply_handle
                .edit(ctx, |builder| {
                    builder.content(result.data.ext_urls.first().unwrap());
                    builder.components(|c| {
                        *c = components(&sauce, &cursor, false, false);
                        c
                    });
                    builder
                })
                .await?;
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

            button.custom_id("prev").label("Previous Source").style(ButtonStyle::Primary).disabled(disabled);
            button
        });

        row.create_button(|button| {
            let disabled = if disable_right { true } else { cursor >= &saucenao.results.len() };

            button.custom_id("next").label("Next Source").style(ButtonStyle::Primary).disabled(disabled);
            button
        });
        row
    });
    components
}
