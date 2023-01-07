use crate::functions::furaffinity::{furaffinity_client, fa_reply, components};
use crate::functions::guild_accent_colour::guild_accent_colour;
use crate::{Context, Error};

use futures::StreamExt;
use poise::serenity_prelude::{InteractionResponseType};
use std::time::Duration;
use std::vec;


/// Turn a FurAffinity link into a fancy embed!
#[poise::command(slash_command, prefix_command, nsfw_only, category = "Furry")]
pub async fn fa(
    ctx: Context<'_>,
    #[description = "The post URL to get"]
    #[rest]
    url: String
) -> Result<(), Error> {
    let fa_client = furaffinity_client(Some(&url), None, &ctx.data().secrets.furaffinity_cookies).await; // Get a FA client, build it on the url input
    let colour = guild_accent_colour(ctx.data().config.lock().unwrap().accent_colour, ctx.guild());

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
    let mut interaction_stream = reply_handle.message().await?.await_component_interactions(ctx).timeout(Duration::from_secs(60 * 3)).build();

    // Act on our interaction context
    while let Some(interaction) = interaction_stream.next().await {
        interaction.create_interaction_response(ctx, |f| f.kind(InteractionResponseType::UpdateMessage)).await?;

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
    
    reply_handle.edit(ctx, |builder|
    builder.components(|c|{
        let components = components(&fa, true);
        *c = components;
        c
    })).await?;

    Ok(())
}
