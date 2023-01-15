use crate::{Context, Error};
use poise::serenity_prelude::{self as serenity};

#[poise::command(slash_command, prefix_command, reuse_response)]
pub async fn test_reuse_response(ctx: Context<'_>) -> Result<(), Error> {
    let image_url = "https://raw.githubusercontent.com/serenity-rs/serenity/current/logo.png";
    ctx.send(|b| {
        b.content("message 1")
            .embed(|b| b.description("embed 1").image(image_url))
            .components(|b| {
                b.create_action_row(|b| {
                    b.create_button(|b| b.label("button 1").style(serenity::ButtonStyle::Primary).custom_id(1))
                })
            })
    })
    .await?;

    tokio::time::sleep(std::time::Duration::from_secs(2)).await;

    let image_url =
        "https://raw.githubusercontent.com/serenity-rs/serenity/current/examples/e09_create_message_builder/ferris_eyes.png";
    ctx.send(|b| {
        b.content("message 2")
            .embed(|b| b.description("embed 2").image(image_url))
            .components(|b| {
                b.create_action_row(|b| {
                    b.create_button(|b| b.label("button 2").style(serenity::ButtonStyle::Danger).custom_id(2))
                })
            })
    })
    .await?;

    Ok(())
}
