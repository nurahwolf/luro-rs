use std::time::Duration;

use futures::StreamExt;
use luro_core::{Data, Error, TIMEOUT_DURIATION};
use luro_utilities::guild_accent_colour;
use poise::{
    serenity_prelude::{InteractionResponseType, Message},
    FrameworkContext
};

use crate::functions::{components, fa_message, fa_message_edit, fa_reply, furaffinity_client};

/// Turn a FurAffinity link into a fancy embed!
#[poise::command(slash_command, prefix_command, category = "Furry")]
pub async fn furaffinity(
    ctx: luro_core::Context<'_>,
    #[description = "The post URL to get"]
    #[rest]
    url: String
) -> Result<(), Error> {
    let fa_client = furaffinity_client(Some(&url), None, &ctx.data().secrets.furaffinity_cookies).await; // Get a FA client, build it on the url input
    let colour = guild_accent_colour(ctx.data().config.read().await.accent_colour, ctx.guild());

    let mut fa = match fa_client {
        // Make sure it is valid
        Ok(fa) => fa,
        Err(err) => {
            ctx.say(format!("Had a fucky wucky! {err}")).await?;
            return Ok(());
        }
    };

    // Build a message based on our response, setting it up for interaction
    let message = fa_reply(&fa, Some(colour)).await;
    let reply_handle = ctx
        .send(|builder| {
            *builder = message;
            builder
        })
        .await?;
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
            fa = match furaffinity_client(None, Some(fa.prev.unwrap()), &ctx.data().secrets.furaffinity_cookies).await {
                // Make sure it is valid
                Ok(fa) => fa,
                Err(err) => {
                    ctx.say(format!("Had a fucky wucky! {err}")).await?;
                    return Ok(());
                }
            };

            // Build a message based on our response, setting it up for interaction
            let next_message = fa_reply(&fa, Some(colour)).await;
            reply_handle
                .edit(ctx, |builder| {
                    *builder = next_message;
                    builder
                })
                .await?;
        }

        if interaction.data.custom_id.contains("next") {
            fa = match furaffinity_client(None, Some(fa.next.unwrap()), &ctx.data().secrets.furaffinity_cookies).await {
                // Make sure it is valid
                Ok(fa) => fa,
                Err(err) => {
                    ctx.say(format!("Had a fucky wucky! {err}")).await?;
                    return Ok(());
                }
            };

            // Build a message based on our response, setting it up for interaction
            let next_message = fa_reply(&fa, Some(colour)).await;
            reply_handle
                .edit(ctx, |builder| {
                    *builder = next_message;
                    builder
                })
                .await?;
        }
    }

    reply_handle
        .edit(ctx, |builder| {
            builder.components(|c| {
                let components = components(&fa, true);
                *c = components;
                c
            })
        })
        .await?;

    Ok(())
}

/// Get a post from FA and send an embed response.
pub async fn event_furaffinity(
    ctx: &poise::serenity_prelude::Context,
    framework: &FrameworkContext<'_, Data, Error>,
    message: &Message
) -> Result<(), reqwest::Error> {
    let url = &message.content;
    let fa_client = furaffinity_client(Some(url), None, &framework.user_data.secrets.furaffinity_cookies).await; // Get a FA client, build it on the url input
    let colour = guild_accent_colour(framework.user_data.config.read().await.accent_colour, message.guild(ctx));

    let mut fa = match fa_client {
        // Make sure it is valid
        Ok(fa) => fa,
        Err(err) => {
            panic!("Furaffinity: Error in request client - {err}");
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

    let mut interaction_stream = reply_handle
        .await_component_interactions(ctx)
        .timeout(Duration::from_secs(60 * 3))
        .build();

    // Act on our interaction context
    while let Some(interaction) = interaction_stream.next().await {
        match interaction
            .create_interaction_response(ctx, |f| f.kind(InteractionResponseType::UpdateMessage))
            .await
        {
            Ok(_) => {}
            Err(err) => panic!("Furaffinity: Had a fuckywucky: {err}")
        }

        if interaction.data.custom_id.contains("prev") {
            fa = match furaffinity_client(None, Some(fa.prev.unwrap()), &framework.user_data.secrets.furaffinity_cookies).await
            {
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
            fa = match furaffinity_client(None, Some(fa.next.unwrap()), &framework.user_data.secrets.furaffinity_cookies).await
            {
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

    match reply_handle
        .edit(ctx, |builder| {
            builder.components(|c| {
                let components = components(&fa, true);
                *c = components;
                c
            })
        })
        .await
    {
        Ok(_) => {}
        Err(err) => panic!("Furaffinity: Had a fuckywucky: {err}")
    }

    Ok(())
}
